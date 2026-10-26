use std::sync::LazyLock;

use indoc::{formatdoc, indoc};
use xc3_shader::graph::{
    BinaryOp, Expr, Graph, UnaryOp,
    query::{assign_x_recursive, query_nodes},
};

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
    Graph::parse_glsl(&query).unwrap().simplify()
});

fn instance_view_query_xz(offset: u32) -> String {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][i].x
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][i].z
    formatdoc! {"
        void main() {{
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_15 = temp_2 + {offset};
            temp_16 = uint(temp_15) >> 2;
            temp_17 = temp_16 >> 2;
            temp_18 = int(temp_16) & 3;
            temp_19 = nuPerViewCBuffer.data[int(temp_17)][temp_18];
        }}
    "}
}

fn instance_view_query_yw(offset: u32) -> String {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][i].y
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][i].w
    formatdoc! {"
        void main() {{
            temp_0 = intBitsToFloat(gl_InstanceID);
            temp_2 = floatBitsToInt(temp_0) << 6;
            temp_15 = temp_2 + {offset};
            temp_16 = uint(temp_15) >> 2;
            temp_20 = int(temp_16) + 1;
            temp_21 = uint(temp_20) >> 2;
            temp_22 = temp_20 & 3;
            temp_23 = nuPerViewCBuffer.data[int(temp_21)][temp_22];
        }}
    "}
}

static INSTANCE_VIEW_COL1_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].x
    let query = instance_view_query_xz(16);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].y
    let query = instance_view_query_yw(16);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].x
    let query = instance_view_query_xz(32);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].y
    let query = instance_view_query_yw(32);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_X: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].x
    let query = instance_view_query_xz(48);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_Y: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].y
    let query = instance_view_query_yw(48);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL0_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][0].z
    let query = instance_view_query_xz(8);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL0_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][0].w
    let query = instance_view_query_yw(8);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].z
    let query = instance_view_query_xz(24);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL1_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][1].w
    let query = instance_view_query_yw(24);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].z
    let query = instance_view_query_xz(40);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL2_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][2].w
    let query = instance_view_query_yw(40);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_Z: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].z
    let query = instance_view_query_xz(56);
    Graph::parse_glsl(&query).unwrap().simplify()
});

static INSTANCE_VIEW_COL3_W: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_00000000010ac240_VS.glsl
    // nuPerViewCBuffer.viewMatrix[gl_InstanceID][3].w
    let query = instance_view_query_yw(56);
    Graph::parse_glsl(&query).unwrap().simplify()
});

pub fn instance_view_matrix<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    let (i, channel) = query_nodes(expr, graph, &INSTANCE_VIEW_COL0_X)
        .map(|_| (0, 'x'))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL0_Y).map(|_| (0, 'y')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL0_Z).map(|_| (0, 'z')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL0_W).map(|_| (0, 'w')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL1_X).map(|_| (1, 'x')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL1_Y).map(|_| (1, 'y')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL1_Z).map(|_| (1, 'z')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL1_W).map(|_| (1, 'w')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL2_X).map(|_| (2, 'x')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL2_Y).map(|_| (2, 'y')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL2_Z).map(|_| (2, 'z')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL2_W).map(|_| (2, 'w')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL3_X).map(|_| (3, 'x')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL3_Y).map(|_| (3, 'y')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL3_Z).map(|_| (3, 'z')))
        .or_else(|| query_nodes(expr, graph, &INSTANCE_VIEW_COL3_W).map(|_| (3, 'w')))?;

    // Assume the necessary index exprs already exist if any of the queries match.
    // TODO: insert the appropriate exprs instead?
    let index = graph
        .exprs
        .iter()
        .position(|e| matches!(e, Expr::Global { name, channel } if name == "gl_InstanceID"))
        .unwrap();

    let index2 = graph.exprs.iter().position(|e| e == &Expr::Int(i)).unwrap();

    Some(Expr::Parameter {
        name: "nuPerViewCBuffer".into(),
        field: Some("viewMatrix".into()),
        index: Some(index),
        index2: Some(index2),
        channel: Some(channel),
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

static PREMULTIPLIED_ALPHA_RGB: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {
        "void main() {
            result = a * b; 
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

static PREMULTIPLIED_ALPHA_A: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {
        "void main() {
            result = fma(alpha, result, alpha);
        }
    "};
    Graph::parse_glsl(query).unwrap()
});

static PREMULTIPLIED_ALPHA_A2: LazyLock<Graph> = LazyLock::new(|| {
    let query = indoc! {
        "void main() {
            neg_alpha = 0.0 - alpha;
            result = fma(result, neg_alpha, alpha);
        }
    "};
    Graph::parse_glsl(query).unwrap()
});

pub fn is_premultiplied_alpha(graph: &Graph) -> Option<bool> {
    let alpha_node = graph
        .nodes
        .iter()
        .rfind(|n| n.output.name == "OUT_Color" && n.output.channel == Some('w'))?;
    // The graph isn't simplified yet, so reduce any assignment chains.
    let alpha_expr = assign_x_recursive(graph, &graph.exprs[alpha_node.input]);
    let result = query_nodes(alpha_expr, graph, &PREMULTIPLIED_ALPHA_A)
        .or_else(|| query_nodes(alpha_expr, graph, &PREMULTIPLIED_ALPHA_A2))?;
    let alpha_expr = assign_x_recursive(graph, result.get("alpha")?);

    // Check if the RGB outputs are all multiplied by alpha.
    let mut is_premultiplied = true;
    for c in "xyz".chars() {
        let node = graph
            .nodes
            .iter()
            .rfind(|n| n.output.name == "OUT_Color" && n.output.channel == Some(c))?;

        let expr = assign_x_recursive(graph, &graph.exprs[node.input]);
        let result = query_nodes(expr, graph, &PREMULTIPLIED_ALPHA_RGB)?;
        let a = assign_x_recursive(graph, result.get("a")?);
        let b = assign_x_recursive(graph, result.get("b")?);
        if a != alpha_expr && b != alpha_expr {
            is_premultiplied = false;
        }
    }
    Some(is_premultiplied)
}
