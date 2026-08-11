use std::sync::LazyLock;

use indoc::indoc;
use xc3_shader::graph::{BinaryOp, Expr, Graph, UnaryOp, query::query_nodes};

use crate::database::Operation;

pub fn op_func<'a>(
    graph: &'a Graph,
    expr: &'a Expr,
    func: &str,
    op: Operation,
) -> Option<(Operation, Vec<&'a Expr>)> {
    match expr {
        Expr::Func { name, args, .. } => {
            if name == func {
                Some((op, args.iter().map(|a| &graph.exprs[*a]).collect()))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn ternary<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<(Operation, Vec<&'a Expr>)> {
    if let Expr::Ternary(cond, a, b) = expr {
        Some((
            Operation::Select,
            vec![&graph.exprs[*cond], &graph.exprs[*a], &graph.exprs[*b]],
        ))
    } else {
        None
    }
}

static PACK_HALF: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {"
        void main() {
            result = packHalf2x16(vec2(a, b));
        }
    "};
    Graph::parse_glsl(&query).unwrap().simplify()
});

pub fn pack_half<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<(Operation, Vec<&'a Expr>)> {
    // Convert the vec2 argument into two scalar arguments.
    let result = query_nodes(expr, graph, &PACK_HALF)?;
    Some((
        Operation::Pack2Float16,
        vec![result.get("a")?, result.get("b")?],
    ))
}

pub fn binary_op<'a>(
    graph: &'a Graph,
    expr: &'a Expr,
    binary_op: BinaryOp,
    operation: Operation,
) -> Option<(Operation, Vec<&'a Expr>)> {
    if let Expr::Binary(op, a0, a1) = expr
        && *op == binary_op
    {
        Some((operation, vec![&graph.exprs[*a0], &graph.exprs[*a1]]))
    } else {
        None
    }
}

pub fn unary_op<'a>(
    graph: &'a Graph,
    expr: &'a Expr,
    unary_op: UnaryOp,
    operation: Operation,
) -> Option<(Operation, Vec<&'a Expr>)> {
    if let Expr::Unary(op, a) = expr
        && *op == unary_op
    {
        Some((operation, vec![&graph.exprs[*a]]))
    } else {
        None
    }
}
