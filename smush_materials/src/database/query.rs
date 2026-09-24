use std::sync::LazyLock;

use indoc::{formatdoc, indoc};
use xc3_shader::graph::{BinaryOp, Expr, Graph, UnaryOp, query::query_nodes};

use crate::database::Operation;
use smush_shader::Operation as Op;

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
            Op::Select.into(),
            vec![&graph.exprs[*cond], &graph.exprs[*a], &graph.exprs[*b]],
        ))
    } else {
        None
    }
}

pub fn binary_ops<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<(Operation, Vec<&'a Expr>)> {
    if let Expr::Binary(op, a0, a1) = expr {
        let args = vec![&graph.exprs[*a0], &graph.exprs[*a1]];

        match op {
            BinaryOp::Add => Some((Op::Add.into(), args)),
            BinaryOp::Sub => Some((Op::Sub.into(), args)),
            BinaryOp::Mul => Some((Op::Mul.into(), args)),
            BinaryOp::Div => Some((Op::Div.into(), args)),
            BinaryOp::LeftShift => Some((Op::LeftShift.into(), args)),
            BinaryOp::RightShift => Some((Op::RightShift.into(), args)),
            BinaryOp::BitOr => None,
            BinaryOp::BitXor => None,
            BinaryOp::BitAnd => Some((Op::BitAnd.into(), args)),
            BinaryOp::Equal => Some((Op::Equal.into(), args)),
            BinaryOp::NotEqual => Some((Op::NotEqual.into(), args)),
            BinaryOp::Less => Some((Op::Less.into(), args)),
            BinaryOp::Greater => Some((Op::Greater.into(), args)),
            BinaryOp::LessEqual => Some((Op::LessEqual.into(), args)),
            BinaryOp::GreaterEqual => Some((Op::GreaterEqual.into(), args)),
            BinaryOp::Or => Some((Op::Or.into(), args)),
            BinaryOp::And => Some((Op::And.into(), args)),
        }
    } else {
        None
    }
}

pub fn unary_ops<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<(Operation, Vec<&'a Expr>)> {
    if let Expr::Unary(op, a) = expr {
        match op {
            UnaryOp::Negate => Some((Op::Negate.into(), vec![&graph.exprs[*a]])),
            UnaryOp::Not => Some((Op::Not.into(), vec![&graph.exprs[*a]])),
            UnaryOp::Complement => None,
        }
    } else {
        None
    }
}

static UNPACK_HALF_X: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {"
        void main() {
            result = unpackHalf2x16(arg).x;
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static UNPACK_HALF_Y: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {"
        void main() {
            result = unpackHalf2x16(arg).y;
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn unpack_half<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<(Operation, Vec<&'a Expr>)> {
    let (op, result) = query_nodes(expr, graph, &UNPACK_HALF_X)
        .map(|r| (Op::Unpack2Float16X, r))
        .or_else(|| query_nodes(expr, graph, &UNPACK_HALF_Y).map(|r| (Op::Unpack2Float16Y, r)))?;
    Some((op.into(), vec![result.get("arg")?]))
}

static PACK_HALF: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {"
        void main() {
            result = packHalf2x16(vec2(a, b));
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn pack_half<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<(Operation, Vec<&'a Expr>)> {
    // Convert the vec2 argument into two scalar arguments.
    let result = query_nodes(expr, graph, &PACK_HALF)?;
    Some((
        Op::Pack2Float16.into(),
        vec![result.get("a")?, result.get("b")?],
    ))
}

// TODO: reduce repetition in queries
static INSTANCE_VIEW_COL0_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][0].x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_4 = uint(temp_2) >> 2;
            temp_5 = temp_4 >> 2;
            temp_6 = int(temp_4) & 3;
            temp_7 = nuPerViewCBuffer.data[int(temp_5)][temp_6];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL0_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][0].y
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_4 = uint(temp_2) >> 2;
            temp_8 = int(temp_4) + 1;
            temp_9 = uint(temp_8) >> 2;
            temp_10 = temp_8 & 3;
            temp_11 = nuPerViewCBuffer.data[int(temp_9)][temp_10];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_15 = temp_2 + 16;
            temp_16 = uint(temp_15) >> 2;
            temp_17 = temp_16 >> 2;
            temp_18 = int(temp_16) & 3;
            temp_19 = nuPerViewCBuffer.data[int(temp_17)][temp_18];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].y
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_15 = temp_2 + 16;
            temp_16 = uint(temp_15) >> 2;
            temp_20 = int(temp_16) + 1;
            temp_21 = uint(temp_20) >> 2;
            temp_22 = temp_20 & 3;
            temp_23 = nuPerViewCBuffer.data[int(temp_21)][temp_22];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_25 = temp_2 + 32;
            temp_26 = uint(temp_25) >> 2;
            temp_27 = temp_26 >> 2;
            temp_28 = int(temp_26) & 3;
            temp_29 = nuPerViewCBuffer.data[int(temp_27)][temp_28];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].y
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_25 = temp_2 + 32;
            temp_26 = uint(temp_25) >> 2;
            temp_30 = int(temp_26) + 1;
            temp_31 = uint(temp_30) >> 2;
            temp_32 = temp_30 & 3;
            temp_33 = nuPerViewCBuffer.data[int(temp_31)][temp_32];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_38 = temp_2 + 48;
            temp_39 = uint(temp_38) >> 2;
            temp_40 = temp_39 >> 2;
            temp_41 = int(temp_39) & 3;
            temp_42 = nuPerViewCBuffer.data[int(temp_40)][temp_41];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].y
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_38 = temp_2 + 48;
            temp_39 = uint(temp_38) >> 2;
            temp_43 = int(temp_39) + 1;
            temp_44 = uint(temp_43) >> 2;
            temp_45 = temp_43 & 3;
            temp_46 = nuPerViewCBuffer.data[int(temp_44)][temp_45];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL0_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][0].z
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_53 = temp_2 + 8;
            temp_54 = uint(temp_53) >> 2;
            temp_55 = temp_54 >> 2;
            temp_56 = int(temp_54) & 3;
            temp_57 = nuPerViewCBuffer.data[int(temp_55)][temp_56];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL0_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][0].w
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_53 = temp_2 + 8;
            temp_54 = uint(temp_53) >> 2;
            temp_58 = int(temp_54) + 1;
            temp_59 = uint(temp_58) >> 2;
            temp_60 = temp_58 & 3;
            temp_61 = nuPerViewCBuffer.data[int(temp_59)][temp_60];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].z
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_65 = temp_2 + 24;
            temp_66 = uint(temp_65) >> 2;
            temp_67 = temp_66 >> 2;
            temp_68 = int(temp_66) & 3;
            temp_69 = nuPerViewCBuffer.data[int(temp_67)][temp_68];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].w
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_65 = temp_2 + 24;
            temp_66 = uint(temp_65) >> 2;
            temp_70 = int(temp_66) + 1;
            temp_71 = uint(temp_70) >> 2;
            temp_72 = temp_70 & 3;
            temp_73 = nuPerViewCBuffer.data[int(temp_71)][temp_72];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].z
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_75 = temp_2 + 40;
            temp_76 = uint(temp_75) >> 2;
            temp_77 = temp_76 >> 2;
            temp_78 = int(temp_76) & 3;
            temp_79 = nuPerViewCBuffer.data[int(temp_77)][temp_78];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].w
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_75 = temp_2 + 40;
            temp_76 = uint(temp_75) >> 2;
            temp_80 = int(temp_76) + 1;
            temp_81 = uint(temp_80) >> 2;
            temp_82 = temp_80 & 3;
            temp_83 = nuPerViewCBuffer.data[int(temp_81)][temp_82];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].z
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_95 = temp_2 + 56;
            temp_96 = uint(temp_95) >> 2;
            temp_97 = temp_96 >> 2;
            temp_98 = int(temp_96) & 3;
            temp_99 = nuPerViewCBuffer.data[int(temp_97)][temp_98];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].w
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_95 = temp_2 + 56;
            temp_96 = uint(temp_95) >> 2;
            temp_100 = int(temp_96) + 1;
            temp_101 = uint(temp_100) >> 2;
            temp_102 = temp_100 & 3;
            temp_103 = nuPerViewCBuffer.data[int(temp_101)][temp_102];
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn instance_view_matrix<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    // Assume the necessary index exprs already exist if any of the queries match.
    // TODO: insert the appropriate exprs instead?
    // TODO: Find a way to reduce repetition.
    query_nodes(expr, graph, &INSTANCE_VIEW_COL0_X)
        .map(|_| Expr::Parameter {
            name: "nuPerViewCBuffer".into(),
            field: Some("viewMatrix".into()),
            index: Some(find_instance_id(graph)),
            index2: Some(find_int(graph, 0)),
            channel: Some('x'),
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL0_Y).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 0)),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL0_Z).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 0)),
                channel: Some('z'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL0_W).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 0)),
                channel: Some('w'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL1_X).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 1)),
                channel: Some('x'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL1_Y).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 1)),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL1_Z).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 1)),
                channel: Some('z'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL1_W).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 1)),
                channel: Some('w'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL2_X).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 2)),
                channel: Some('x'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL2_Y).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 2)),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL2_Z).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 2)),
                channel: Some('z'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL2_W).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 2)),
                channel: Some('w'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL3_X).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 3)),
                channel: Some('x'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL3_Y).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 3)),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL3_Z).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 3)),
                channel: Some('z'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &INSTANCE_VIEW_COL3_W).map(|_| Expr::Parameter {
                name: "nuPerViewCBuffer".into(),
                field: Some("viewMatrix".into()),
                index: Some(find_instance_id(graph)),
                index2: Some(find_int(graph, 3)),
                channel: Some('w'),
            })
        })
}

fn find_instance_id(graph: &Graph) -> usize {
    graph
        .exprs
        .iter()
        .position(|e| matches!(e, Expr::Global { name, channel } if name == "gl_InstanceID"))
        .unwrap()
}

fn find_int(graph: &Graph, i: i32) -> usize {
    graph.exprs.iter().position(|e| e == &Expr::Int(i)).unwrap()
}

static UNK_POSITION_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_0000008009008a68_VS.glsl, gl_Position.x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_1 = IN_Position.x;
            temp_3 = IN_Position.y;
            temp_4 = floatBitsToInt(temp_0) << 6;
            temp_6 = uint(temp_4) >> 2;
            temp_8 = int(temp_6) & 3;
            temp_9 = nuPerViewCBuffer.data[int(temp_7)][temp_8];
            temp_10 = int(temp_6) + 1;
            temp_12 = temp_10 & 3;
            temp_13 = nuPerViewCBuffer.data[int(temp_11)][temp_12];
            temp_14 = temp_4 + 16;
            temp_15 = uint(temp_14) >> 2;
            temp_17 = int(temp_15) & 3;
            temp_18 = nuPerViewCBuffer.data[int(temp_16)][temp_17];
            temp_19 = int(temp_15) + 1;
            temp_21 = temp_19 & 3;
            temp_22 = nuPerViewCBuffer.data[int(temp_20)][temp_21];
            temp_23 = IN_Position.z;
            temp_24 = temp_4 + 32;
            temp_25 = uint(temp_24) >> 2;
            temp_27 = int(temp_25) & 3;
            temp_28 = nuPerViewCBuffer.data[int(temp_26)][temp_27];
            temp_29 = int(temp_25) + 1;
            temp_31 = temp_29 & 3;
            temp_32 = nuPerViewCBuffer.data[int(temp_30)][temp_31];
            temp_33 = temp_4 + 8;
            temp_34 = uint(temp_33) >> 2;
            temp_36 = int(temp_34) & 3;
            temp_37 = nuPerViewCBuffer.data[int(temp_35)][temp_36];
            temp_38 = int(temp_34) + 1;
            temp_40 = temp_38 & 3;
            temp_41 = nuPerViewCBuffer.data[int(temp_39)][temp_40];
            temp_42 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].w;
            temp_44 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].z;
            temp_46 = temp_4 + 48;
            temp_47 = uint(temp_46) >> 2;
            temp_49 = int(temp_47) & 3;
            temp_50 = nuPerViewCBuffer.data[int(temp_48)][temp_49];
            temp_51 = int(temp_47) + 1;
            temp_53 = temp_51 & 3;
            temp_54 = nuPerViewCBuffer.data[int(temp_52)][temp_53];
            temp_55 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].x;
            temp_56 = temp_4 + 24;
            temp_57 = uint(temp_56) >> 2;
            temp_59 = int(temp_57) & 3;
            temp_60 = nuPerViewCBuffer.data[int(temp_58)][temp_59];
            temp_61 = int(temp_57) + 1;
            temp_63 = temp_61 & 3;
            temp_64 = nuPerViewCBuffer.data[int(temp_62)][temp_63];
            temp_66 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].w, temp_42);
            temp_67 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].z, temp_44);
            temp_68 = temp_4 + 40;
            temp_69 = uint(temp_68) >> 2;
            temp_71 = int(temp_69) & 3;
            temp_72 = nuPerViewCBuffer.data[int(temp_70)][temp_71];
            temp_73 = int(temp_69) + 1;
            temp_75 = temp_73 & 3;
            temp_76 = nuPerViewCBuffer.data[int(temp_74)][temp_75];
            temp_77 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].x, temp_55);
            temp_78 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].y;
            temp_80 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].x, temp_77);
            temp_83 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].y, temp_78);
            temp_86 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].y, temp_83);
            temp_88 = temp_4 + 56;
            temp_89 = uint(temp_88) >> 2;
            temp_91 = int(temp_89) & 3;
            temp_92 = nuPerViewCBuffer.data[int(temp_90)][temp_91];
            temp_93 = int(temp_89) + 1;
            temp_95 = temp_93 & 3;
            temp_96 = nuPerViewCBuffer.data[int(temp_94)][temp_95];
            temp_100 = temp_13 * nuPerViewCBuffer.projectionMatrix[1].x;
            temp_105 = fma(temp_9, nuPerViewCBuffer.projectionMatrix[0].x, temp_100);
            temp_108 = temp_22 * nuPerViewCBuffer.projectionMatrix[1].x;
            temp_110 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].w, temp_66);
            temp_111 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].z, temp_67);
            temp_117 = temp_32 * nuPerViewCBuffer.projectionMatrix[1].x;
            temp_120 = fma(temp_18, nuPerViewCBuffer.projectionMatrix[0].x, temp_108);
            temp_125 = fma(temp_28, nuPerViewCBuffer.projectionMatrix[0].x, temp_117);
            temp_128 = temp_80 + nuPerWorldCBuffer.worldMatrix[3].x;
            temp_132 = temp_54 * nuPerViewCBuffer.projectionMatrix[1].x;
            temp_134 = temp_86 + nuPerWorldCBuffer.worldMatrix[3].y;
            temp_140 = fma(temp_50, nuPerViewCBuffer.projectionMatrix[0].x, temp_132);
            temp_144 = fma(temp_37, nuPerViewCBuffer.projectionMatrix[2].x, temp_105);
            temp_147 = fma(temp_72, nuPerViewCBuffer.projectionMatrix[2].x, temp_125);
            temp_149 = temp_111 + nuPerWorldCBuffer.worldMatrix[3].z;
            temp_155 = fma(temp_60, nuPerViewCBuffer.projectionMatrix[2].x, temp_120);
            temp_157 = temp_110 + nuPerWorldCBuffer.worldMatrix[3].w;
            temp_161 = fma(temp_92, nuPerViewCBuffer.projectionMatrix[2].x, temp_140);
            temp_172 = fma(temp_41, nuPerViewCBuffer.projectionMatrix[3].x, temp_144);
            temp_180 = fma(temp_64, nuPerViewCBuffer.projectionMatrix[3].x, temp_155);
            temp_224 = fma(temp_76, nuPerViewCBuffer.projectionMatrix[3].x, temp_147);
            temp_248 = fma(temp_96, nuPerViewCBuffer.projectionMatrix[3].x, temp_161);
            temp_264 = temp_128 * temp_172;
            temp_271 = fma(temp_134, temp_180, temp_264);
            temp_294 = fma(temp_149, temp_224, temp_271);
            temp_299 = fma(temp_157, temp_248, temp_294);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static UNK_POSITION_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_0000008009008a68_VS.glsl, gl_Position.y
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_1 = IN_Position.x;
            temp_3 = IN_Position.y;
            temp_4 = floatBitsToInt(temp_0) << 6;
            temp_6 = uint(temp_4) >> 2;
            temp_8 = int(temp_6) & 3;
            temp_9 = nuPerViewCBuffer.data[int(temp_7)][temp_8];
            temp_10 = int(temp_6) + 1;
            temp_12 = temp_10 & 3;
            temp_13 = nuPerViewCBuffer.data[int(temp_11)][temp_12];
            temp_14 = temp_4 + 16;
            temp_15 = uint(temp_14) >> 2;
            temp_17 = int(temp_15) & 3;
            temp_18 = nuPerViewCBuffer.data[int(temp_16)][temp_17];
            temp_19 = int(temp_15) + 1;
            temp_21 = temp_19 & 3;
            temp_22 = nuPerViewCBuffer.data[int(temp_20)][temp_21];
            temp_23 = IN_Position.z;
            temp_24 = temp_4 + 32;
            temp_25 = uint(temp_24) >> 2;
            temp_27 = int(temp_25) & 3;
            temp_28 = nuPerViewCBuffer.data[int(temp_26)][temp_27];
            temp_29 = int(temp_25) + 1;
            temp_31 = temp_29 & 3;
            temp_32 = nuPerViewCBuffer.data[int(temp_30)][temp_31];
            temp_33 = temp_4 + 8;
            temp_34 = uint(temp_33) >> 2;
            temp_36 = int(temp_34) & 3;
            temp_37 = nuPerViewCBuffer.data[int(temp_35)][temp_36];
            temp_38 = int(temp_34) + 1;
            temp_40 = temp_38 & 3;
            temp_41 = nuPerViewCBuffer.data[int(temp_39)][temp_40];
            temp_42 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].w;
            temp_44 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].z;
            temp_46 = temp_4 + 48;
            temp_47 = uint(temp_46) >> 2;
            temp_49 = int(temp_47) & 3;
            temp_50 = nuPerViewCBuffer.data[int(temp_48)][temp_49];
            temp_51 = int(temp_47) + 1;
            temp_53 = temp_51 & 3;
            temp_54 = nuPerViewCBuffer.data[int(temp_52)][temp_53];
            temp_55 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].x;
            temp_56 = temp_4 + 24;
            temp_57 = uint(temp_56) >> 2;
            temp_59 = int(temp_57) & 3;
            temp_60 = nuPerViewCBuffer.data[int(temp_58)][temp_59];
            temp_61 = int(temp_57) + 1;
            temp_63 = temp_61 & 3;
            temp_64 = nuPerViewCBuffer.data[int(temp_62)][temp_63];
            temp_66 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].w, temp_42);
            temp_67 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].z, temp_44);
            temp_68 = temp_4 + 40;
            temp_69 = uint(temp_68) >> 2;
            temp_71 = int(temp_69) & 3;
            temp_72 = nuPerViewCBuffer.data[int(temp_70)][temp_71];
            temp_73 = int(temp_69) + 1;
            temp_75 = temp_73 & 3;
            temp_76 = nuPerViewCBuffer.data[int(temp_74)][temp_75];
            temp_77 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].x, temp_55);
            temp_78 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].y;
            temp_80 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].x, temp_77);
            temp_83 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].y, temp_78);
            temp_86 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].y, temp_83);
            temp_88 = temp_4 + 56;
            temp_89 = uint(temp_88) >> 2;
            temp_91 = int(temp_89) & 3;
            temp_92 = nuPerViewCBuffer.data[int(temp_90)][temp_91];
            temp_93 = int(temp_89) + 1;
            temp_95 = temp_93 & 3;
            temp_96 = nuPerViewCBuffer.data[int(temp_94)][temp_95];
            temp_99 = temp_13 * nuPerViewCBuffer.projectionMatrix[1].y;
            temp_104 = fma(temp_9, nuPerViewCBuffer.projectionMatrix[0].y, temp_99);
            temp_106 = temp_22 * nuPerViewCBuffer.projectionMatrix[1].y;
            temp_110 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].w, temp_66);
            temp_111 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].z, temp_67);
            temp_114 = temp_32 * nuPerViewCBuffer.projectionMatrix[1].y;
            temp_116 = fma(temp_18, nuPerViewCBuffer.projectionMatrix[0].y, temp_106);
            temp_122 = fma(temp_28, nuPerViewCBuffer.projectionMatrix[0].y, temp_114);
            temp_128 = temp_80 + nuPerWorldCBuffer.worldMatrix[3].x;
            temp_131 = temp_54 * nuPerViewCBuffer.projectionMatrix[1].y;
            temp_134 = temp_86 + nuPerWorldCBuffer.worldMatrix[3].y;
            temp_139 = fma(temp_50, nuPerViewCBuffer.projectionMatrix[0].y, temp_131);
            temp_141 = fma(temp_37, nuPerViewCBuffer.projectionMatrix[2].y, temp_104);
            temp_145 = fma(temp_72, nuPerViewCBuffer.projectionMatrix[2].y, temp_122);
            temp_149 = temp_111 + nuPerWorldCBuffer.worldMatrix[3].z;
            temp_154 = fma(temp_60, nuPerViewCBuffer.projectionMatrix[2].y, temp_116);
            temp_157 = temp_110 + nuPerWorldCBuffer.worldMatrix[3].w;
            temp_164 = fma(temp_92, nuPerViewCBuffer.projectionMatrix[2].y, temp_139);
            temp_170 = fma(temp_41, nuPerViewCBuffer.projectionMatrix[3].y, temp_141);
            temp_178 = fma(temp_64, nuPerViewCBuffer.projectionMatrix[3].y, temp_154);
            temp_223 = fma(temp_76, nuPerViewCBuffer.projectionMatrix[3].y, temp_145);
            temp_255 = fma(temp_96, nuPerViewCBuffer.projectionMatrix[3].y, temp_164);
            temp_281 = temp_128 * temp_170;
            temp_285 = fma(temp_134, temp_178, temp_281);
            temp_292 = fma(temp_149, temp_223, temp_285);
            temp_298 = fma(temp_157, temp_255, temp_292);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static UNK_POSITION_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_0000008009008a68_VS.glsl, gl_Position.z
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_1 = IN_Position.x;
            temp_3 = IN_Position.y;
            temp_4 = floatBitsToInt(temp_0) << 6;
            temp_6 = uint(temp_4) >> 2;
            temp_8 = int(temp_6) & 3;
            temp_9 = nuPerViewCBuffer.data[int(temp_7)][temp_8];
            temp_10 = int(temp_6) + 1;
            temp_12 = temp_10 & 3;
            temp_13 = nuPerViewCBuffer.data[int(temp_11)][temp_12];
            temp_14 = temp_4 + 16;
            temp_15 = uint(temp_14) >> 2;
            temp_17 = int(temp_15) & 3;
            temp_18 = nuPerViewCBuffer.data[int(temp_16)][temp_17];
            temp_19 = int(temp_15) + 1;
            temp_21 = temp_19 & 3;
            temp_22 = nuPerViewCBuffer.data[int(temp_20)][temp_21];
            temp_23 = IN_Position.z;
            temp_24 = temp_4 + 32;
            temp_25 = uint(temp_24) >> 2;
            temp_27 = int(temp_25) & 3;
            temp_28 = nuPerViewCBuffer.data[int(temp_26)][temp_27];
            temp_29 = int(temp_25) + 1;
            temp_31 = temp_29 & 3;
            temp_32 = nuPerViewCBuffer.data[int(temp_30)][temp_31];
            temp_33 = temp_4 + 8;
            temp_34 = uint(temp_33) >> 2;
            temp_36 = int(temp_34) & 3;
            temp_37 = nuPerViewCBuffer.data[int(temp_35)][temp_36];
            temp_38 = int(temp_34) + 1;
            temp_40 = temp_38 & 3;
            temp_41 = nuPerViewCBuffer.data[int(temp_39)][temp_40];
            temp_42 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].w;
            temp_44 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].z;
            temp_46 = temp_4 + 48;
            temp_47 = uint(temp_46) >> 2;
            temp_49 = int(temp_47) & 3;
            temp_50 = nuPerViewCBuffer.data[int(temp_48)][temp_49];
            temp_51 = int(temp_47) + 1;
            temp_53 = temp_51 & 3;
            temp_54 = nuPerViewCBuffer.data[int(temp_52)][temp_53];
            temp_55 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].x;
            temp_56 = temp_4 + 24;
            temp_57 = uint(temp_56) >> 2;
            temp_59 = int(temp_57) & 3;
            temp_60 = nuPerViewCBuffer.data[int(temp_58)][temp_59];
            temp_61 = int(temp_57) + 1;
            temp_63 = temp_61 & 3;
            temp_64 = nuPerViewCBuffer.data[int(temp_62)][temp_63];
            temp_66 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].w, temp_42);
            temp_67 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].z, temp_44);
            temp_68 = temp_4 + 40;
            temp_69 = uint(temp_68) >> 2;
            temp_71 = int(temp_69) & 3;
            temp_72 = nuPerViewCBuffer.data[int(temp_70)][temp_71];
            temp_73 = int(temp_69) + 1;
            temp_75 = temp_73 & 3;
            temp_76 = nuPerViewCBuffer.data[int(temp_74)][temp_75];
            temp_77 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].x, temp_55);
            temp_78 = temp_1 * nuPerWorldCBuffer.worldMatrix[0].y;
            temp_80 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].x, temp_77);
            temp_83 = fma(temp_3, nuPerWorldCBuffer.worldMatrix[1].y, temp_78);
            temp_86 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].y, temp_83);
            temp_88 = temp_4 + 56;
            temp_89 = uint(temp_88) >> 2;
            temp_91 = int(temp_89) & 3;
            temp_92 = nuPerViewCBuffer.data[int(temp_90)][temp_91];
            temp_93 = int(temp_89) + 1;
            temp_95 = temp_93 & 3;
            temp_96 = nuPerViewCBuffer.data[int(temp_94)][temp_95];
            temp_97 = temp_13 * nuPerViewCBuffer.projectionMatrix[1].z;
            temp_101 = temp_22 * nuPerViewCBuffer.projectionMatrix[1].z;
            temp_102 = fma(temp_9, nuPerViewCBuffer.projectionMatrix[0].z, temp_97);
            temp_110 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].w, temp_66);
            temp_111 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].z, temp_67);
            temp_115 = temp_32 * nuPerViewCBuffer.projectionMatrix[1].z;
            temp_119 = fma(temp_18, nuPerViewCBuffer.projectionMatrix[0].z, temp_101);
            temp_124 = fma(temp_28, nuPerViewCBuffer.projectionMatrix[0].z, temp_115);
            temp_126 = temp_54 * nuPerViewCBuffer.projectionMatrix[1].z;
            temp_128 = temp_80 + nuPerWorldCBuffer.worldMatrix[3].x;
            temp_133 = fma(temp_50, nuPerViewCBuffer.projectionMatrix[0].z, temp_126);
            temp_134 = temp_86 + nuPerWorldCBuffer.worldMatrix[3].y;
            temp_143 = fma(temp_37, nuPerViewCBuffer.projectionMatrix[2].z, temp_102);
            temp_146 = fma(temp_72, nuPerViewCBuffer.projectionMatrix[2].z, temp_124);
            temp_149 = temp_111 + nuPerWorldCBuffer.worldMatrix[3].z;
            temp_151 = fma(temp_60, nuPerViewCBuffer.projectionMatrix[2].z, temp_119);
            temp_157 = temp_110 + nuPerWorldCBuffer.worldMatrix[3].w;
            temp_166 = fma(temp_92, nuPerViewCBuffer.projectionMatrix[2].z, temp_133);
            temp_171 = fma(temp_41, nuPerViewCBuffer.projectionMatrix[3].z, temp_143);
            temp_179 = fma(temp_64, nuPerViewCBuffer.projectionMatrix[3].z, temp_151);
            temp_222 = fma(temp_76, nuPerViewCBuffer.projectionMatrix[3].z, temp_146);
            temp_256 = fma(temp_96, nuPerViewCBuffer.projectionMatrix[3].z, temp_166);
            temp_276 = temp_128 * temp_171;
            temp_286 = fma(temp_134, temp_179, temp_276);
            temp_295 = fma(temp_149, temp_222, temp_286);
            temp_300 = fma(temp_157, temp_256, temp_295);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn unk_position<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    query_nodes(expr, graph, &UNK_POSITION_X)
        .map(|_| Expr::Global {
            name: "unk_position".into(),
            channel: Some('x'),
        })
        .or_else(|| {
            query_nodes(expr, graph, &UNK_POSITION_Y).map(|_| Expr::Global {
                name: "unk_position".into(),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &UNK_POSITION_Z).map(|_| Expr::Global {
                name: "unk_position".into(),
                channel: Some('z'),
            })
        })
}

static UNK_PROJECTION_V: LazyLock<Graph> = LazyLock::new(|| {
    // TODO: Is this some sort of projected UV maps toggled by CustomBoolean5?
    // SFX_PBS_0000000800084100_VS.glsl, OUT_uv_map1.y
    let query = indoc! {"
        void main() {
            temp_5 = intBitsToFloat(gl_InstanceID);
            temp_8 = floatBitsToInt(temp_5) << 6;
            temp_130 = temp_8 + 48;
            temp_131 = uint(temp_130) >> 2;
            temp_132 = temp_131 >> 2;
            temp_133 = int(temp_131) & 3;
            temp_134 = nuPerViewCBuffer.data[int(temp_132)][temp_133];
            temp_135 = int(temp_131) + 1;
            temp_136 = uint(temp_135) >> 2;
            temp_137 = temp_135 & 3;
            temp_138 = nuPerViewCBuffer.data[int(temp_136)][temp_137];
            temp_142 = temp_8 + 56;
            temp_143 = uint(temp_142) >> 2;
            temp_144 = temp_143 >> 2;
            temp_145 = int(temp_143) & 3;
            temp_146 = nuPerViewCBuffer.data[int(temp_144)][temp_145];
            temp_147 = int(temp_143) + 1;
            temp_148 = uint(temp_147) >> 2;
            temp_149 = temp_147 & 3;
            temp_150 = nuPerViewCBuffer.data[int(temp_148)][temp_149];
            temp_168 = temp_138 * nuPerViewCBuffer.projectionMatrix[1].w;
            temp_172 = fma(temp_134, nuPerViewCBuffer.projectionMatrix[0].w, temp_168);
            temp_182 = fma(temp_146, nuPerViewCBuffer.projectionMatrix[2].w, temp_172);
            temp_189 = fma(temp_150, nuPerViewCBuffer.projectionMatrix[3].w, temp_182);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static UNK_PROJECTION_V2: LazyLock<Graph> = LazyLock::new(|| {
    // TODO: Is this some sort of projected UV maps toggled by CustomBoolean9?
    // SFX_PBS_0000000000000500_VS.glsl, OUT_uv_map1.y
    let query = indoc! {"
        void main() {
            temp_5 = intBitsToFloat(gl_InstanceID);
            temp_17 = floatBitsToInt(temp_5) << 6;
            temp_31 = temp_17 + 32;
            temp_32 = uint(temp_31) >> 2;
            temp_33 = temp_32 >> 2;
            temp_34 = int(temp_32) & 3;
            temp_35 = nuPerViewCBuffer.data[int(temp_33)][temp_34];
            temp_36 = int(temp_32) + 1;
            temp_37 = uint(temp_36) >> 2;
            temp_38 = temp_36 & 3;
            temp_39 = nuPerViewCBuffer.data[int(temp_37)][temp_38];
            temp_109 = temp_17 + 40;
            temp_110 = uint(temp_109) >> 2;
            temp_111 = temp_110 >> 2;
            temp_112 = int(temp_110) & 3;
            temp_113 = nuPerViewCBuffer.data[int(temp_111)][temp_112];
            temp_128 = temp_39 * nuPerViewCBuffer.projectionMatrix[1].x;
            temp_146 = fma(temp_35, nuPerViewCBuffer.projectionMatrix[0].x, temp_128);
            temp_192 = fma(temp_113, nuPerViewCBuffer.projectionMatrix[2].x, temp_146);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn unk_projection_v<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    query_nodes(expr, graph, &UNK_PROJECTION_V)
        .or_else(|| query_nodes(expr, graph, &UNK_PROJECTION_V2))
        .map(|_| Expr::Global {
            name: "unk_projection_v".into(),
            channel: None,
        })
}

fn renormal_query(c: char) -> String {
    // SFX_PBS_011000000800826b_VS.glsl
    formatdoc! {"
        void main() {{
            temp_0 = intBitsToFloat(gl_VertexID);
            temp_4 = floatBitsToInt(temp_0) & 65535;
            temp_5 = temp_4 * 36;
            temp_6 = floatBitsToUint(temp_0) >> 16;
            temp_7 = int(temp_6) * 36;
            temp_8 = temp_7 << 16;
            temp_9 = temp_8 + temp_5;
            temp_10 = 0;
            temp_12 = 0;
            temp_13 = 0.0;
            temp_14 = 0.0;
            temp_15 = 0.0;
            temp_20 = temp_10;
            temp_22 = temp_12;
            temp_23 = temp_13;
            temp_24 = temp_14;
            temp_25 = temp_15;
            temp_29 = temp_20 << 2;
            temp_30 = temp_29 + temp_9;
            temp_31 = uint(temp_30) >> 2;
            temp_32 = uintBitsToFloat(adjacenciesStruct.data[int(temp_31)]);
            temp_34 = floatBitsToUint(temp_32) >> 16;
            temp_35 = int(temp_34) != 65535;
            temp_36 = floatBitsToInt(temp_32) & 65535;
            temp_37 = temp_36 != 65535;
            temp_38 = temp_37 && temp_35;
            temp_39 = uint(temp_22);
            temp_40 = temp_23;
            temp_41 = temp_24;
            temp_42 = temp_25;
            temp_46 = temp_38 ? temp_36 & 65535 : temp_46;
            temp_47 = temp_38 ? temp_46 * 28 : temp_47;
            temp_48 = temp_38 ? int(temp_34) & 65535 : temp_48;
            temp_49 = temp_38 ? temp_48 * 28 : temp_49;
            temp_50 = temp_38 ? temp_49 + 8 : temp_50;
            temp_51 = temp_38 ? uint(temp_47) >> 2 : temp_51;
            temp_52 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_51)]) : temp_52;
            temp_53 = temp_38 ? temp_49 + 4 : temp_53;
            temp_54 = temp_38 ? temp_47 + 8 : temp_54;
            temp_55 = temp_38 ? temp_47 + 4 : temp_55;
            temp_56 = temp_38 ? uint(temp_53) >> 2 : temp_56;
            temp_57 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_56)]) : temp_57;
            temp_58 = temp_38 ? IN_Position.y : temp_58;
            temp_61 = temp_38 ? uint(temp_54) >> 2 : temp_61;
            temp_62 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_61)]) : temp_62;
            temp_65 = temp_38 ? uint(temp_50) >> 2 : temp_65;
            temp_66 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_65)]) : temp_66;
            temp_67 = temp_38 ? IN_Position.x : temp_67;
            temp_68 = temp_38 ? uint(temp_55) >> 2 : temp_68;
            temp_69 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_68)]) : temp_69;
            temp_70 = temp_38 ? temp_22 + 1 : temp_70;
            temp_71 = temp_38 ? IN_Position.z : temp_71;
            temp_72 = temp_38 ? uint(temp_49) >> 2 : temp_72;
            temp_73 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_72)]) : temp_73;
            temp_74 = temp_38 ? 0.0 - temp_57 : temp_74;
            temp_75 = temp_38 ? temp_74 + temp_58 : temp_75;
            temp_76 = temp_38 ? 0.0 - temp_62 : temp_76;
            temp_77 = temp_38 ? temp_76 + temp_71 : temp_77;
            temp_78 = temp_38 ? temp_75 * temp_77 : temp_78;
            temp_79 = temp_38 ? 0.0 - temp_69 : temp_79;
            temp_80 = temp_38 ? temp_79 + temp_58 : temp_80;
            temp_81 = temp_38 ? 0.0 - temp_66 : temp_81;
            temp_82 = temp_38 ? temp_81 + temp_71 : temp_82;
            temp_83 = temp_38 ? 0.0 - temp_52 : temp_83;
            temp_84 = temp_38 ? temp_83 + temp_67 : temp_84;
            temp_85 = temp_38 ? 0.0 - temp_73 : temp_85;
            temp_86 = temp_38 ? temp_85 + temp_67 : temp_86;
            temp_87 = temp_38 ? temp_82 * temp_84 : temp_87;
            temp_88 = temp_38 ? 0.0 - temp_78 : temp_88;
            temp_89 = temp_38 ? fma(temp_82, temp_80, temp_88) : temp_89;
            temp_90 = temp_38 ? temp_86 * temp_80 : temp_90;
            temp_91 = temp_38 ? 0.0 - temp_87 : temp_91;
            temp_92 = temp_38 ? fma(temp_86, temp_77, temp_91) : temp_92;
            temp_93 = temp_38 ? temp_89 * temp_89 : temp_93;
            temp_94 = temp_38 ? 0.0 - temp_90 : temp_94;
            temp_95 = temp_38 ? fma(temp_75, temp_84, temp_94) : temp_95;
            temp_96 = temp_38 ? fma(temp_92, temp_92, temp_93) : temp_96;
            temp_97 = temp_38 ? fma(temp_95, temp_95, temp_96) : temp_97;
            temp_98 = temp_38 ? inversesqrt(temp_97) : temp_98;
            temp_99 = temp_38 ? fma(temp_89, temp_98, temp_23) : temp_99;
            temp_100 = temp_38 ? fma(temp_92, temp_98, temp_24) : temp_100;
            temp_101 = temp_38 ? fma(temp_95, temp_98, temp_25) : temp_101;
            temp_39 = temp_38 ? uint(temp_70) : temp_39;
            temp_40 = temp_38 ? temp_99 : temp_40;
            temp_41 = temp_38 ? temp_100 : temp_41;
            temp_42 = temp_38 ? temp_101 : temp_42;
            temp_102 = temp_39;
            temp_103 = temp_40;
            temp_104 = temp_41;
            temp_105 = temp_42;
            temp_114 = float(temp_102);
            temp_118 = 1.0 / temp_114;
            temp_142 = temp_104 * temp_118;
            temp_144 = temp_103 * temp_118;
            temp_161 = temp_105 * temp_118;
            temp_159 = temp_144 * nuPerWorldCBuffer.worldMatrix[0].{c};
            temp_167 = fma(temp_142, nuPerWorldCBuffer.worldMatrix[1].{c}, temp_159);
            temp_175 = fma(temp_161, nuPerWorldCBuffer.worldMatrix[2].{c}, temp_167);
        }}
    "}
}

static RENORMAL_X: LazyLock<Graph> = LazyLock::new(|| {
    let query = renormal_query('x');
    Graph::parse_glsl(&query).unwrap().simplify()
});

static RENORMAL_Y: LazyLock<Graph> = LazyLock::new(|| {
    let query = renormal_query('y');
    Graph::parse_glsl(&query).unwrap().simplify()
});

static RENORMAL_Z: LazyLock<Graph> = LazyLock::new(|| {
    let query = renormal_query('z');
    Graph::parse_glsl(&query).unwrap().simplify()
});

pub fn renormal<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    // TODO: make this an operation that takes the position as an argument.
    query_nodes(expr, graph, &RENORMAL_X)
        .map(|_| Expr::Global {
            name: "IN_Normal".into(),
            channel: Some('x'),
        })
        .or_else(|| {
            query_nodes(expr, graph, &RENORMAL_Y).map(|_| Expr::Global {
                name: "IN_Normal".into(),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &RENORMAL_Z).map(|_| Expr::Global {
                name: "IN_Normal".into(),
                channel: Some('z'),
            })
        })
}

static UNK_RENORMAL_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_011000000800826b_VS.glsl, OUT_FinalOffset.x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_VertexID);
            temp_4 = floatBitsToInt(temp_0) & 65535;
            temp_5 = temp_4 * 36;
            temp_6 = floatBitsToUint(temp_0) >> 16;
            temp_7 = int(temp_6) * 36;
            temp_8 = temp_7 << 16;
            temp_9 = temp_8 + temp_5;
            temp_10 = 0;
            temp_16 = intBitsToFloat(undef);
            temp_20 = temp_10;
            temp_26 = floatBitsToInt(temp_16);
            temp_29 = temp_20 << 2;
            temp_30 = temp_29 + temp_9;
            temp_31 = uint(temp_30) >> 2;
            temp_32 = uintBitsToFloat(adjacenciesStruct.data[int(temp_31)]);
            temp_34 = floatBitsToUint(temp_32) >> 16;
            temp_35 = int(temp_34) != 65535;
            temp_36 = floatBitsToInt(temp_32) & 65535;
            temp_37 = temp_36 != 65535;
            temp_38 = temp_37 && temp_35;
            temp_43 = temp_26;
            temp_46 = temp_38 ? temp_36 & 65535 : temp_46;
            temp_47 = temp_38 ? temp_46 * 28 : temp_47;
            temp_51 = temp_38 ? uint(temp_47) >> 2 : temp_51;
            temp_52 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_51)]) : temp_52;
            temp_43 = temp_38 ? floatBitsToInt(temp_52) : temp_43;
            temp_106 = temp_43;
            temp_110 = intBitsToFloat(temp_106);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static UNK_RENORMAL_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_011000000800826b_VS.glsl, OUT_FinalOffset.y
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_VertexID);
            temp_4 = floatBitsToInt(temp_0) & 65535;
            temp_5 = temp_4 * 36;
            temp_6 = floatBitsToUint(temp_0) >> 16;
            temp_7 = int(temp_6) * 36;
            temp_8 = temp_7 << 16;
            temp_9 = temp_8 + temp_5;
            temp_10 = 0;
            temp_17 = intBitsToFloat(undef);
            temp_20 = temp_10;
            temp_27 = floatBitsToInt(temp_17);
            temp_29 = temp_20 << 2;
            temp_30 = temp_29 + temp_9;
            temp_31 = uint(temp_30) >> 2;
            temp_32 = uintBitsToFloat(adjacenciesStruct.data[int(temp_31)]);
            temp_34 = floatBitsToUint(temp_32) >> 16;
            temp_35 = int(temp_34) != 65535;
            temp_36 = floatBitsToInt(temp_32) & 65535;
            temp_37 = temp_36 != 65535;
            temp_38 = temp_37 && temp_35;
            temp_44 = temp_27;
            temp_46 = temp_38 ? temp_36 & 65535 : temp_46;
            temp_47 = temp_38 ? temp_46 * 28 : temp_47;
            temp_55 = temp_38 ? temp_47 + 4 : temp_55;
            temp_59 = temp_38 ? temp_55 + floatBitsToInt(vp_c0.data[17].x) : temp_59;
            temp_60 = temp_38 ? uint(temp_59) < uint(temp_55) : temp_60;
            temp_63 = temp_38 ? temp_60 ? -1 : 0 & 1 : temp_63;
            temp_64 = temp_38 ? floatBitsToInt(vp_c0.data[17].y) + temp_63 : temp_64;
            temp_44 = temp_38 ? temp_64 : temp_44;
            temp_107 = temp_44;
            temp_111 = intBitsToFloat(temp_107);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static UNK_RENORMAL_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_011000000800826b_VS.glsl, OUT_FinalOffset.z
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_VertexID);
            temp_4 = floatBitsToInt(temp_0) & 65535;
            temp_5 = temp_4 * 36;
            temp_6 = floatBitsToUint(temp_0) >> 16;
            temp_7 = int(temp_6) * 36;
            temp_8 = temp_7 << 16;
            temp_9 = temp_8 + temp_5;
            temp_10 = 0;
            temp_18 = intBitsToFloat(undef);
            temp_20 = temp_10;
            temp_28 = floatBitsToInt(temp_18);
            temp_29 = temp_20 << 2;
            temp_30 = temp_29 + temp_9;
            temp_31 = uint(temp_30) >> 2;
            temp_32 = uintBitsToFloat(adjacenciesStruct.data[int(temp_31)]);
            temp_34 = floatBitsToUint(temp_32) >> 16;
            temp_35 = int(temp_34) != 65535;
            temp_36 = floatBitsToInt(temp_32) & 65535;
            temp_37 = temp_36 != 65535;
            temp_38 = temp_37 && temp_35;
            temp_45 = temp_28;
            temp_46 = temp_38 ? temp_36 & 65535 : temp_46;
            temp_47 = temp_38 ? temp_46 * 28 : temp_47;
            temp_55 = temp_38 ? temp_47 + 4 : temp_55;
            temp_58 = temp_38 ? IN_Position.y : temp_58;
            temp_68 = temp_38 ? uint(temp_55) >> 2 : temp_68;
            temp_69 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_68)]) : temp_69;
            temp_79 = temp_38 ? 0.0 - temp_69 : temp_79;
            temp_80 = temp_38 ? temp_79 + temp_58 : temp_80;
            temp_45 = temp_38 ? floatBitsToInt(temp_80) : temp_45;
            temp_108 = temp_45;
            temp_112 = intBitsToFloat(temp_108);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn unk_renormal<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    // TODO: figure out how effect area and RENORMAL code interacts
    query_nodes(expr, graph, &UNK_RENORMAL_X)
        .map(|_| Expr::Global {
            name: "unk_renormal".into(),
            channel: Some('x'),
        })
        // TODO: why does this not work?
        .or_else(|| {
            query_nodes(expr, graph, &UNK_RENORMAL_Y).map(|_| Expr::Global {
                name: "unk_renormal".into(),
                channel: Some('y'),
            })
        })
        .or_else(|| {
            query_nodes(expr, graph, &UNK_RENORMAL_Z).map(|_| Expr::Global {
                name: "unk_renormal".into(),
                channel: Some('z'),
            })
        })
}

static NL_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_011000000800826b_VS.glsl, OUT_NL.x
    let query = indoc! {"
        void main() {
            temp_0 = intBitsToFloat(gl_VertexID);
            temp_4 = floatBitsToInt(temp_0) & 65535;
            temp_5 = temp_4 * 36;
            temp_6 = floatBitsToUint(temp_0) >> 16;
            temp_7 = int(temp_6) * 36;
            temp_8 = temp_7 << 16;
            temp_9 = temp_8 + temp_5;
            temp_10 = 0;
            temp_12 = 0;
            temp_13 = 0.0;
            temp_14 = 0.0;
            temp_15 = 0.0;
            temp_20 = temp_10;
            temp_22 = temp_12;
            temp_23 = temp_13;
            temp_24 = temp_14;
            temp_25 = temp_15;
            temp_29 = temp_20 << 2;
            temp_30 = temp_29 + temp_9;
            temp_31 = uint(temp_30) >> 2;
            temp_32 = uintBitsToFloat(adjacenciesStruct.data[int(temp_31)]);
            temp_34 = floatBitsToUint(temp_32) >> 16;
            temp_35 = int(temp_34) != 65535;
            temp_36 = floatBitsToInt(temp_32) & 65535;
            temp_37 = temp_36 != 65535;
            temp_38 = temp_37 && temp_35;
            temp_39 = uint(temp_22);
            temp_40 = temp_23;
            temp_41 = temp_24;
            temp_42 = temp_25;
            temp_46 = temp_38 ? temp_36 & 65535 : temp_46;
            temp_47 = temp_38 ? temp_46 * 28 : temp_47;
            temp_48 = temp_38 ? int(temp_34) & 65535 : temp_48;
            temp_49 = temp_38 ? temp_48 * 28 : temp_49;
            temp_50 = temp_38 ? temp_49 + 8 : temp_50;
            temp_51 = temp_38 ? uint(temp_47) >> 2 : temp_51;
            temp_52 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_51)]) : temp_52;
            temp_53 = temp_38 ? temp_49 + 4 : temp_53;
            temp_54 = temp_38 ? temp_47 + 8 : temp_54;
            temp_55 = temp_38 ? temp_47 + 4 : temp_55;
            temp_56 = temp_38 ? uint(temp_53) >> 2 : temp_56;
            temp_57 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_56)]) : temp_57;
            temp_58 = temp_38 ? IN_Position.y : temp_58;
            temp_61 = temp_38 ? uint(temp_54) >> 2 : temp_61;
            temp_62 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_61)]) : temp_62;
            temp_65 = temp_38 ? uint(temp_50) >> 2 : temp_65;
            temp_66 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_65)]) : temp_66;
            temp_67 = temp_38 ? IN_Position.x : temp_67;
            temp_68 = temp_38 ? uint(temp_55) >> 2 : temp_68;
            temp_69 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_68)]) : temp_69;
            temp_70 = temp_38 ? temp_22 + 1 : temp_70;
            temp_71 = temp_38 ? IN_Position.z : temp_71;
            temp_72 = temp_38 ? uint(temp_49) >> 2 : temp_72;
            temp_73 = temp_38 ? uintBitsToFloat(verticesStruct.data[int(temp_72)]) : temp_73;
            temp_74 = temp_38 ? 0.0 - temp_57 : temp_74;
            temp_75 = temp_38 ? temp_74 + temp_58 : temp_75;
            temp_76 = temp_38 ? 0.0 - temp_62 : temp_76;
            temp_77 = temp_38 ? temp_76 + temp_71 : temp_77;
            temp_78 = temp_38 ? temp_75 * temp_77 : temp_78;
            temp_79 = temp_38 ? 0.0 - temp_69 : temp_79;
            temp_80 = temp_38 ? temp_79 + temp_58 : temp_80;
            temp_81 = temp_38 ? 0.0 - temp_66 : temp_81;
            temp_82 = temp_38 ? temp_81 + temp_71 : temp_82;
            temp_83 = temp_38 ? 0.0 - temp_52 : temp_83;
            temp_84 = temp_38 ? temp_83 + temp_67 : temp_84;
            temp_85 = temp_38 ? 0.0 - temp_73 : temp_85;
            temp_86 = temp_38 ? temp_85 + temp_67 : temp_86;
            temp_87 = temp_38 ? temp_82 * temp_84 : temp_87;
            temp_88 = temp_38 ? 0.0 - temp_78 : temp_88;
            temp_89 = temp_38 ? fma(temp_82, temp_80, temp_88) : temp_89;
            temp_90 = temp_38 ? temp_86 * temp_80 : temp_90;
            temp_91 = temp_38 ? 0.0 - temp_87 : temp_91;
            temp_92 = temp_38 ? fma(temp_86, temp_77, temp_91) : temp_92;
            temp_93 = temp_38 ? temp_89 * temp_89 : temp_93;
            temp_94 = temp_38 ? 0.0 - temp_90 : temp_94;
            temp_95 = temp_38 ? fma(temp_75, temp_84, temp_94) : temp_95;
            temp_96 = temp_38 ? fma(temp_92, temp_92, temp_93) : temp_96;
            temp_97 = temp_38 ? fma(temp_95, temp_95, temp_96) : temp_97;
            temp_98 = temp_38 ? inversesqrt(temp_97) : temp_98;
            temp_99 = temp_38 ? fma(temp_89, temp_98, temp_23) : temp_99;
            temp_100 = temp_38 ? fma(temp_92, temp_98, temp_24) : temp_100;
            temp_101 = temp_38 ? fma(temp_95, temp_98, temp_25) : temp_101;
            temp_39 = temp_38 ? uint(temp_70) : temp_39;
            temp_40 = temp_38 ? temp_99 : temp_40;
            temp_41 = temp_38 ? temp_100 : temp_41;
            temp_42 = temp_38 ? temp_101 : temp_42;
            temp_102 = temp_39;
            temp_103 = temp_40;
            temp_104 = temp_41;
            temp_105 = temp_42;
            temp_114 = float(temp_102);
            temp_118 = 1.0 / temp_114;
            temp_142 = temp_104 * temp_118;
            temp_144 = temp_103 * temp_118;
            temp_159 = temp_144 * nuPerWorldCBuffer.worldMatrix[0].x;
            temp_160 = temp_144 * nuPerWorldCBuffer.worldMatrix[0].y;
            temp_161 = temp_105 * temp_118;
            temp_162 = temp_144 * nuPerWorldCBuffer.worldMatrix[0].z;
            temp_167 = fma(temp_142, nuPerWorldCBuffer.worldMatrix[1].x, temp_159);
            temp_169 = fma(temp_142, nuPerWorldCBuffer.worldMatrix[1].y, temp_160);
            temp_170 = fma(temp_142, nuPerWorldCBuffer.worldMatrix[1].z, temp_162);
            temp_175 = fma(temp_161, nuPerWorldCBuffer.worldMatrix[2].x, temp_167);
            temp_176 = fma(temp_161, nuPerWorldCBuffer.worldMatrix[2].y, temp_169);
            temp_177 = fma(temp_161, nuPerWorldCBuffer.worldMatrix[2].z, temp_170);
            temp_182 = temp_175 * temp_175;
            temp_183 = fma(temp_176, temp_176, temp_182);
            temp_186 = fma(temp_177, temp_177, temp_183);
            temp_187 = inversesqrt(temp_186);
            temp_200 = temp_175 * temp_187;
            temp_208 = temp_176 * temp_187;
            temp_209 = temp_200 * PerFrame.bgRotInv[0].x;
            temp_215 = fma(temp_208, PerFrame.bgRotInv[1].x, temp_209);
            temp_218 = temp_177 * temp_187;
            temp_219 = temp_200 * PerFrame.bgRotInv[0].y;
            temp_224 = temp_200 * PerFrame.bgRotInv[0].z;
            temp_225 = fma(temp_208, PerFrame.bgRotInv[1].y, temp_219);
            temp_226 = fma(temp_218, PerFrame.bgRotInv[2].x, temp_215);
            temp_231 = fma(temp_208, PerFrame.bgRotInv[1].z, temp_224);
            temp_232 = fma(temp_218, PerFrame.bgRotInv[2].y, temp_225);
            temp_233 = 0.0 - PerObject.lightDir1.x;
            temp_234 = temp_226 * temp_233;
            temp_241 = fma(temp_218, PerFrame.bgRotInv[2].z, temp_231);
            temp_242 = 0.0 - PerObject.lightDir1.y;
            temp_243 = fma(temp_232, temp_242, temp_234);
            temp_252 = 0.0 - PerObject.lightDir1.z;
            temp_253 = fma(temp_241, temp_252, temp_243);
            OUT_NL_x = temp_253;
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn nl_x<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    // TODO: is this some sort of normal length for RENORMAL shaders?
    query_nodes(expr, graph, &NL_X).map(|_| Expr::Global {
        name: "nl".into(),
        channel: Some('x'),
    })
}

static ANISOTROPIC_ROTATION: LazyLock<Graph> = LazyLock::new(|| {
    // TODO: does this require a more specific query?
    let query = indoc! {"
        void main() {
            prm = prm;
            alpha = prm.w;
            result = fma(alpha, 2.0, -1.0);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn has_anisotropic_rotation(graph: &Graph) -> bool {
    graph
        .nodes
        .iter()
        .any(|n| query_nodes(&graph.exprs[n.input], graph, &ANISOTROPIC_ROTATION).is_some())
}

static PREMULTIPLIED_ALPHA: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {"
        void main() {
            temp_743 = fma(temp_736, temp_742, temp_736);
            temp_744 = temp_739 * temp_736;
            temp_745 = temp_740 * temp_736;
            temp_746 = temp_741 * temp_736;
            out_attr0.x = temp_745;
            out_attr0.y = temp_744;
            out_attr0.z = temp_746;
            out_attr0.w = temp_743;
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn is_premultiplied_alpha(graph: &Graph) -> Option<bool> {
    // TODO: This doesn't have the correct output name for annotated graphs?
    let node = graph
        .nodes
        .iter()
        .rfind(|n| n.output.name == "out_attr0" && n.output.channel == Some('w'))?;

    // Check if the RGB outputs are multiplied by alpha.
    let result = query_nodes(&graph.exprs[node.input], graph, &PREMULTIPLIED_ALPHA)?;

    Some(!result.is_empty())
}
