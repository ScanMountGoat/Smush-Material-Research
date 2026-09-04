use std::sync::LazyLock;

use indoc::indoc;
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

static UNK_DISTANCE: LazyLock<Graph> = LazyLock::new(|| {
    // SFX_PBS_0000008009008a68_VS.glsl
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
            temp_98 = temp_13 * nuPerViewCBuffer.projectionMatrix[1].w;
            temp_103 = fma(temp_9, nuPerViewCBuffer.projectionMatrix[0].w, temp_98);
            temp_107 = temp_22 * nuPerViewCBuffer.projectionMatrix[1].w;
            temp_110 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].w, temp_66);
            temp_111 = fma(temp_23, nuPerWorldCBuffer.worldMatrix[2].z, temp_67);
            temp_113 = temp_32 * nuPerViewCBuffer.projectionMatrix[1].w;
            temp_118 = fma(temp_18, nuPerViewCBuffer.projectionMatrix[0].w, temp_107);
            temp_121 = fma(temp_37, nuPerViewCBuffer.projectionMatrix[2].w, temp_103);
            temp_123 = fma(temp_28, nuPerViewCBuffer.projectionMatrix[0].w, temp_113);
            temp_127 = temp_54 * nuPerViewCBuffer.projectionMatrix[1].w;
            temp_128 = temp_80 + nuPerWorldCBuffer.worldMatrix[3].x;
            temp_129 = fma(temp_60, nuPerViewCBuffer.projectionMatrix[2].w, temp_118);
            temp_130 = fma(temp_41, nuPerViewCBuffer.projectionMatrix[3].w, temp_121);
            temp_134 = temp_86 + nuPerWorldCBuffer.worldMatrix[3].y;
            temp_135 = fma(temp_50, nuPerViewCBuffer.projectionMatrix[0].w, temp_127);
            temp_136 = fma(temp_64, nuPerViewCBuffer.projectionMatrix[3].w, temp_129);
            temp_137 = temp_130 * temp_128;
            temp_138 = fma(temp_72, nuPerViewCBuffer.projectionMatrix[2].w, temp_123);
            temp_148 = fma(temp_136, temp_134, temp_137);
            temp_149 = temp_111 + nuPerWorldCBuffer.worldMatrix[3].z;
            temp_150 = fma(temp_76, nuPerViewCBuffer.projectionMatrix[3].w, temp_138);
            temp_152 = fma(temp_92, nuPerViewCBuffer.projectionMatrix[2].w, temp_135);
            temp_156 = fma(temp_150, temp_149, temp_148);
            temp_157 = temp_110 + nuPerWorldCBuffer.worldMatrix[3].w;
            temp_158 = fma(temp_96, nuPerViewCBuffer.projectionMatrix[3].w, temp_152);
            temp_162 = fma(temp_158, temp_157, temp_156);
        }
    "};
    Graph::parse_glsl(query).unwrap().simplify()
});

pub fn unk_distance<'a>(graph: &'a Graph, expr: &'a Expr) -> Option<Expr> {
    query_nodes(expr, graph, &UNK_DISTANCE).map(|_| Expr::Global {
        name: "unk_distance".into(),
        channel: None,
    })
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
    // SFX_PBS_0000000800084100_VS.glsl.glsl, OUT_uv_map1.y
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
    // SFX_PBS_0000000000000500_VS.glsl.glsl, OUT_uv_map1.y
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
