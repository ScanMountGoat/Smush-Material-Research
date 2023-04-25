use std::collections::HashMap;

use glsl::{
    parser::Parse,
    syntax::{Expr, ShaderStage, SimpleStatement},
    transpiler::glsl::{show_expr, show_simple_statement},
    visitor::{Host, Visit, Visitor},
};

#[derive(Debug, Default)]
struct AssignmentVisitor {
    assignments: Vec<AssignmentDependency>,

    // Cache the last line where each variable was assigned.
    last_assignment_index: HashMap<String, usize>,
}

impl AssignmentVisitor {
    fn add_assignment(&mut self, output: String, input: &Expr, statement: &SimpleStatement) {
        let mut inputs = Vec::new();
        add_vars(input, &mut inputs, &self.last_assignment_index);

        let assignment = AssignmentDependency {
            output,
            input_last_assignments: inputs,
            expr: print_statement(statement),
        };
        // The visitor doesn't track line numbers.
        // We only need to look up the assignments, so use the index instead.
        self.last_assignment_index
            .insert(assignment.output.clone(), self.assignments.len());
        self.assignments.push(assignment);
    }
}

#[derive(Debug, Default)]
struct AssignmentDependency {
    output: String,
    expr: String,
    // Include where any inputs were last initialized.
    // This makes edge traversal O(1) later.
    input_last_assignments: Vec<usize>,
}

fn add_vars(expr: &Expr, vars: &mut Vec<usize>, most_recent_assignment: &HashMap<String, usize>) {
    // Collect and variables used in an expression.
    // Code like fma(a, b, c) should return [a, b, c].
    // TODO: Treat variables like a.x and a.y as distinct?
    // TODO: Include constants?
    match expr {
        Expr::Variable(i) => {
            // The base case is a single variable like temp_01.
            // Ignore identifiers without assignments like uniform buffer names.
            if let Some(line_number) = most_recent_assignment.get(&i.0) {
                vars.push(*line_number);
            }
        }
        Expr::IntConst(_) => (),
        Expr::UIntConst(_) => (),
        Expr::BoolConst(_) => (),
        Expr::FloatConst(_) => (),
        Expr::DoubleConst(_) => (),
        Expr::Unary(_, e) => add_vars(e, vars, most_recent_assignment),
        Expr::Binary(_, lh, rh) => {
            add_vars(lh, vars, most_recent_assignment);
            add_vars(rh, vars, most_recent_assignment);
        }
        Expr::Ternary(a, b, c) => {
            add_vars(a, vars, most_recent_assignment);
            add_vars(b, vars, most_recent_assignment);
            add_vars(c, vars, most_recent_assignment);
        }
        Expr::Assignment(_, _, _) => todo!(),
        Expr::Bracket(e, _) => add_vars(e, vars, most_recent_assignment),
        Expr::FunCall(_, es) => {
            for e in es {
                add_vars(e, vars, most_recent_assignment);
            }
        }
        Expr::Dot(e, _) => add_vars(e, vars, most_recent_assignment),
        Expr::PostInc(e) => add_vars(e, vars, most_recent_assignment),
        Expr::PostDec(e) => add_vars(e, vars, most_recent_assignment),
        Expr::Comma(_, _) => todo!(),
    }
}

fn print_expr(expr: &Expr) -> String {
    let mut text = String::new();
    show_expr(&mut text, expr);
    text
}

fn print_statement(statement: &SimpleStatement) -> String {
    // TODO: Find a way to pretty print instead?
    let mut text = String::new();
    show_simple_statement(&mut text, statement);
    text
}

impl Visitor for AssignmentVisitor {
    fn visit_simple_statement(&mut self, statement: &SimpleStatement) -> Visit {
        match statement {
            SimpleStatement::Expression(Some(glsl::syntax::Expr::Assignment(lh, _, rh))) => {
                let output = print_expr(lh);
                self.add_assignment(output, rh, statement);
                Visit::Children
            }
            SimpleStatement::Declaration(glsl::syntax::Declaration::InitDeclaratorList(l)) => {
                // TODO: is it worth handling complex initializers?
                if let Some(glsl::syntax::Initializer::Simple(init)) = l.head.initializer.as_ref() {
                    let output = l.head.name.as_ref().unwrap().0.clone();
                    self.add_assignment(output, init, statement);
                }

                Visit::Children
            }
            _ => Visit::Children,
        }
    }
}

pub fn source_dependencies(source: &str, var: &str) -> String {
    // TODO: avoid unwrap?
    let translation_unit = ShaderStage::parse(source).unwrap();

    // Visit each assignment to establish data dependencies.
    // This converts the code to a directed acyclic graph (DAG).
    let mut visitor = AssignmentVisitor::default();
    translation_unit.visit(&mut visitor);

    // Find the last assignment containing the desired variable.
    let (assignment_index, assignment) = visitor
        .assignments
        .iter()
        .enumerate()
        .rfind(|(_, a)| a.output == var)
        .unwrap();

    // Follow data dependencies backwards to find all relevant lines.
    let mut dependencies = vec![(assignment_index, assignment)];
    add_dependencies(&mut dependencies, assignment, &visitor.assignments);

    // Sort by line number and remove duplicates.
    dependencies.sort_by_key(|(i, _)| *i);
    dependencies.dedup_by_key(|(i, _)| *i);

    // Combine all the lines into source code again.
    // These won't exactly match the originals due to formatting differences.
    dependencies
        .iter()
        .map(|(_, d)| d.expr.clone())
        .collect::<Vec<_>>()
        .join("")
}

fn add_dependencies<'a>(
    dependencies: &mut Vec<(usize, &'a AssignmentDependency)>,
    assignment: &AssignmentDependency,
    assignments: &'a [AssignmentDependency],
) {
    // Recursively collect lines that the given assignment depends on.
    for index in &assignment.input_last_assignments {
        let last_assignment = &assignments[*index];
        dependencies.push((*index, last_assignment));

        add_dependencies(dependencies, last_assignment, assignments);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn source_dependencies_final_assignment() {
        let glsl = indoc! {"
            layout (binding = 9, std140) uniform fp_c9
            {
                vec4 fp_c9_data[0x1000];
            };

            void main() 
            {
                float a = fp_c9_data[0].x;
                float b = 2.0;
                float c = a * b;
                float d = fma(a, b, c);
                d = d + 1.0;
                OUT_Color.x = c + d;
            }
        "};

        assert_eq!(
            indoc! {"
                float a = fp_c9_data[0].x;
                float b = 2.;
                float c = a*b;
                float d = fma(a, b, c);
                d = d+1.;
                OUT_Color.x = c+d;
            "},
            source_dependencies(glsl, "OUT_Color.x")
        );
    }

    #[test]
    fn source_dependencies_intermediate_assignment() {
        let glsl = indoc! {"
            void main() 
            {
                float a = 1.0;
                float b = 2.0;
                float d = fma(a, b, -1.0);
                float c = 2 * b;
                d = d + 1.0;
                OUT_Color.x = c + d;
            }
        "};

        assert_eq!(
            indoc! {"
                float b = 2.;
                float c = 2*b;
            "},
            source_dependencies(glsl, "c")
        );
    }
}
