use glsl::{
    parser::Parse,
    syntax::{Expr, ShaderStage, SimpleStatement},
    transpiler::glsl::{show_expr, show_simple_statement},
    visitor::{Host, Visit, Visitor},
};

#[derive(Debug, Default)]
struct AssignmentVisitor {
    assignments: Vec<AssignmentDependency>,
}

#[derive(Debug, Default)]
struct AssignmentDependency {
    output: String,
    inputs: Vec<String>,
    expr: String,
}

pub fn print_glsl_dependencies(input_path: &str, var: &str) -> usize {
    let source = std::fs::read_to_string(input_path).unwrap();

    // TODO: Write to file and move to main.rs?
    let code = source_dependencies(&source, var);
    println!("{code}");

    1
}

fn add_vars(expr: &Expr, vars: &mut Vec<String>) {
    // Collect and variables used in an expression.
    // Code like fma(a, b, c) should return [a, b, c].
    // TODO: Treat variables like a.x and a.y as distinct?
    // TODO: Include constants?
    match expr {
        Expr::Variable(i) => vars.push(i.0.clone()),
        Expr::IntConst(_) => (),
        Expr::UIntConst(_) => (),
        Expr::BoolConst(_) => (),
        Expr::FloatConst(_) => (),
        Expr::DoubleConst(_) => (),
        Expr::Unary(_, e) => add_vars(e, vars),
        Expr::Binary(_, lh, rh) => {
            add_vars(lh, vars);
            add_vars(rh, vars);
        }
        Expr::Ternary(a, b, c) => {
            add_vars(a, vars);
            add_vars(b, vars);
            add_vars(c, vars);
        }
        Expr::Assignment(_, _, _) => todo!(),
        Expr::Bracket(e, _) => add_vars(e, vars),
        Expr::FunCall(_, es) => {
            for e in es {
                add_vars(e, vars);
            }
        }
        Expr::Dot(e, _) => add_vars(e, vars),
        Expr::PostInc(e) => add_vars(e, vars),
        Expr::PostDec(e) => add_vars(e, vars),
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
            SimpleStatement::Expression(Some(expr)) => match expr {
                glsl::syntax::Expr::Assignment(lh, _, rh) => {
                    let mut inputs = Vec::new();
                    add_vars(rh, &mut inputs);

                    let assignment = AssignmentDependency {
                        output: print_expr(lh),
                        inputs,
                        expr: print_statement(statement),
                    };
                    self.assignments.push(assignment);

                    Visit::Children
                }
                _ => Visit::Children,
            },
            // TODO: Is this the right way to handle statements like "float a = b;"?
            SimpleStatement::Declaration(decl) => match decl {
                glsl::syntax::Declaration::FunctionPrototype(_) => todo!(),
                glsl::syntax::Declaration::InitDeclaratorList(l) => {
                    if let Some(glsl::syntax::Initializer::Simple(init)) =
                        l.head.initializer.as_ref()
                    {
                        let mut inputs = Vec::new();
                        add_vars(&init, &mut inputs);

                        let assignment = AssignmentDependency {
                            output: l.head.name.as_ref().unwrap().0.clone(),
                            inputs,
                            expr: print_statement(statement),
                        };
                        self.assignments.push(assignment);
                    }

                    Visit::Children
                }
                glsl::syntax::Declaration::Precision(_, _) => todo!(),
                glsl::syntax::Declaration::Block(_) => todo!(),
                glsl::syntax::Declaration::Global(_, _) => todo!(),
            },
            _ => Visit::Children,
        }
    }
}

fn source_dependencies(source: &str, var: &str) -> String {
    // TODO: avoid unwrap?
    let translation_unit = ShaderStage::parse(source).unwrap();

    // Visit each assignment to establish data dependencies.
    // This converts the code to a directed acyclic graph (DAG).
    let mut visitor = AssignmentVisitor::default();
    translation_unit.visit(&mut visitor);

    // dbg!(&visitor.assignments);

    // Find the last assignment containing the desired variable.
    let (assignment_index, assignment) = visitor
        .assignments
        .iter()
        .enumerate()
        .rfind(|(_, a)| a.output == var)
        .unwrap();

    // Follow data dependencies backwards to find all relevant lines.
    let mut dependencies = vec![(assignment_index, assignment)];
    add_dependencies(
        &mut dependencies,
        assignment_index,
        assignment,
        &visitor.assignments,
    );

    // Sort by line number and remove duplicates.
    dependencies.sort_by_key(|(i, _)| *i);
    dependencies.dedup_by_key(|(i, _)| *i);

    // Combine all the lines into source code again.
    dependencies
        .iter()
        .map(|(_, d)| d.expr.clone())
        .collect::<Vec<_>>()
        .join("")
}

fn add_dependencies<'a>(
    dependencies: &mut Vec<(usize, &'a AssignmentDependency)>,
    assignment_index: usize,
    assignment: &AssignmentDependency,
    assignments: &'a [AssignmentDependency],
) {
    // Recursively collect lines that the given assignment depends on.
    for input in &assignment.inputs {
        if let Some(dependency) = find_most_recent_assignment(assignment_index, &input, assignments)
        {
            dependencies.push(dependency);

            add_dependencies(dependencies, dependency.0, dependency.1, assignments);
        }
    }
}

fn find_most_recent_assignment<'a>(
    line_index: usize,
    var: &str,
    assignments: &'a [AssignmentDependency],
) -> Option<(usize, &'a AssignmentDependency)> {
    // Only search the assignments up to but not including the current one.
    // TODO: Cache repeated lookups based var and line index?
    assignments
        .iter()
        .enumerate()
        .take(line_index)
        .rfind(|(_, a)| a.output == var)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn source_dependencies_test() {
        let glsl = indoc! {"
            void main() 
            {
                float a = 1.0;
                float b = 2.0;
                float c = a * b;
                float d = fma(a, b, c);
                d = d + 1.0;
                OUT_Color.x = c + d;
            }
        "};

        assert_eq!(
            indoc! {"
                float a = 1.;
                float b = 2.;
                float c = a*b;
                float d = fma(a, b, c);
                d = d+1.;
                OUT_Color.x = c+d;
            "},
            source_dependencies(glsl)
        );
    }
}
