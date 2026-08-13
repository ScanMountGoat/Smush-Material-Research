use indexmap::IndexMap;
use indoc::indoc;
use log::error;
use query::*;
use rayon::prelude::*;
use smol_str::{SmolStr, format_smolstr};
use smush_shader::{ShaderDatabase, ShaderExprs, ShaderProgram, Value};
use ssbh_data::shdr_data::Metadata;
use std::{borrow::Cow, collections::BTreeSet, path::Path};
use xc3_shader::{
    expr::{ExprCache, OutputExpr, output_expr},
    graph::{
        BinaryOp, Graph, UnaryOp,
        glsl::{GlslGraph, merge_vertex_fragment, shader_source_no_extensions},
        query::query_nodes_glsl,
    },
};

use smush_shader::Operation as Op;

mod query;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct Operation(smush_shader::Operation);

impl From<Operation> for smush_shader::Operation {
    fn from(value: Operation) -> Self {
        value.0
    }
}

impl From<smush_shader::Operation> for Operation {
    fn from(value: smush_shader::Operation) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl xc3_shader::expr::Operation for Operation {
    fn query_operation_args<'a>(
        graph: &'a Graph,
        expr: &'a xc3_shader::graph::Expr,
    ) -> Option<(Self, Vec<&'a xc3_shader::graph::Expr>)> {
        // TODO: how to handle bitfieldExtract to be compatible with WGSL?
        binary_op(graph, expr, BinaryOp::Add, Op::Add.into())
            .or_else(|| binary_op(graph, expr, BinaryOp::Sub, Op::Sub.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::Mul, Op::Mul.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::Div, Op::Div.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::Equal, Op::Equal.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::NotEqual, Op::NotEqual.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::GreaterEqual, Op::GreaterEqual.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::LessEqual, Op::LessEqual.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::LeftShift, Op::LeftShift.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::RightShift, Op::RightShift.into()))
            .or_else(|| binary_op(graph, expr, BinaryOp::BitAnd, Op::BitAnd.into()))
            .or_else(|| op_func(graph, expr, "fma", Op::Fma.into()))
            .or_else(|| op_func(graph, expr, "min", Op::Min.into()))
            .or_else(|| op_func(graph, expr, "max", Op::Max.into()))
            .or_else(|| op_func(graph, expr, "exp2", Op::Exp2.into()))
            .or_else(|| op_func(graph, expr, "clamp", Op::Clamp.into()))
            .or_else(|| op_func(graph, expr, "inversesqrt", Op::InverseSqrt.into()))
            .or_else(|| op_func(graph, expr, "log2", Op::Log2.into()))
            .or_else(|| op_func(graph, expr, "abs", Op::Abs.into()))
            .or_else(|| op_func(graph, expr, "sqrt", Op::Sqrt.into()))
            .or_else(|| op_func(graph, expr, "floor", Op::Floor.into()))
            .or_else(|| op_func(graph, expr, "trunc", Op::Trunc.into()))
            .or_else(|| op_func(graph, expr, "sin", Op::Sin.into()))
            .or_else(|| op_func(graph, expr, "cos", Op::Cos.into()))
            .or_else(|| op_func(graph, expr, "intBitsToFloat", Op::IntBitsToFloat.into()))
            .or_else(|| op_func(graph, expr, "uintBitsToFloat", Op::UintBitsToFloat.into()))
            .or_else(|| op_func(graph, expr, "floatBitsToInt", Op::FloatBitsToInt.into()))
            .or_else(|| op_func(graph, expr, "floatBitsToUint", Op::FloatBitsToUint.into()))
            .or_else(|| op_func(graph, expr, "int", Op::Int.into()))
            .or_else(|| op_func(graph, expr, "uint", Op::Uint.into()))
            .or_else(|| op_func(graph, expr, "float", Op::Float.into()))
            .or_else(|| op_func(graph, expr, "unpackHalf2x16", Op::Unpack2Float16.into()))
            .or_else(|| pack_half(graph, expr))
            .or_else(|| unary_op(graph, expr, UnaryOp::Negate, Op::Negate.into()))
            .or_else(|| unary_op(graph, expr, UnaryOp::Not, Op::Not.into()))
            .or_else(|| ternary(graph, expr))
            .or_else(|| {
                error!("Unsupported expression {expr:?}");
                None
            })
    }

    fn preprocess_expr<'a>(
        _graph: &'a Graph,
        expr: &'a xc3_shader::graph::Expr,
    ) -> std::borrow::Cow<'a, xc3_shader::graph::Expr> {
        Cow::Borrowed(expr)
    }

    fn preprocess_value_expr<'a>(
        _graph: &'a Graph,
        expr: &'a xc3_shader::graph::Expr,
    ) -> std::borrow::Cow<'a, xc3_shader::graph::Expr> {
        Cow::Borrowed(expr)
    }
}

pub fn shader_from_glsl(vertex: GlslGraph, fragment: GlslGraph) -> ShaderExprs {
    // Create a combined graph that links vertex outputs to fragment inputs.
    // This effectively moves all shader logic to the fragment shader.
    // This simplifies generating shader code or material nodes in 3D applications.
    let frag_attributes = fragment.attributes.clone();

    // TODO: keep the named vertex outputs?
    let graph = merge_vertex_fragment(
        GlslGraph {
            graph: vertex.graph.simplify(),
            attributes: vertex.attributes,
        },
        fragment,
        |_, e| e.clone(),
    );
    let graph = graph.simplify();

    let mut exprs = ExprCache::<Operation>::default();

    let mut output_dependencies = IndexMap::default();

    for output_name in frag_attributes.output_locations.left_values() {
        for c in "xyzw".chars() {
            // Find the most recent assignment for the output variable.
            let node = graph
                .nodes
                .iter()
                .rfind(|n| &n.output.name == output_name && n.output.channel == Some(c))
                .unwrap();
            let expr = &graph.exprs[node.input];

            let value = output_expr(expr, &graph, &mut exprs);
            output_dependencies.insert(format_smolstr!("{output_name}.{c}"), value);
        }
    }

    let exprs = exprs
        .into_exprs()
        .into_iter()
        .map(|e| match e {
            OutputExpr::Value(value) => OutputExpr::Value(value),
            OutputExpr::Func { op, args } => OutputExpr::Func {
                op: op.into(),
                args,
            },
        })
        .collect();

    // TODO: Create a type for this and add it to the parent ShaderProgram type?
    // TODO: This function shouldn't use any game specific data?
    ShaderExprs {
        output_dependencies,
        exprs,
    }
}

pub fn export_shader_database(
    nufx_file: String,
    binary_folder: String,
    source_folder: String,
    output_file: String,
) -> anyhow::Result<usize> {
    // Generate the shader info JSON for ssbh_wgpu.
    let nufx = ssbh_lib::formats::nufx::Nufx::from_file(&nufx_file)?;
    if let ssbh_lib::formats::nufx::Nufx::V1(nufx) = nufx {
        // TODO: Make excluding duplicate render pass entries optional?
        // All "SFX_PBS..." programs support all render passes.
        // Only consider one render pass per program since the entries are identical.
        let mut database = ShaderDatabase {
            programs: nufx
                .programs
                .elements
                .par_iter()
                .filter(|program| program.render_pass.to_str() == Some("nu::Final"))
                .map(|program| {
                    // We can infer information from the shader source using some basic heurstics.
                    let pixel_shader = program.shaders.pixel_shader.to_string_lossy();
                    let pixel_source = shader_source(&source_folder, &pixel_shader);

                    let vertex_shader = program.shaders.vertex_shader.to_string_lossy();
                    let vertex_source = shader_source(&source_folder, &vertex_shader);

                    // Alpha testing in Smash Ultimate is done in shader, so check for discard.
                    // There may be false positives if the discard code path is unused.
                    let discard = pixel_source
                        .as_ref()
                        .map(|source| source.contains("discard;"))
                        .unwrap_or_default();

                    let vert = vertex_source.as_ref().ok().map(|source| {
                        let glsl = shader_source_no_extensions(source);
                        GlslGraph::parse_glsl(glsl).unwrap()
                    });

                    let frag = pixel_source.as_ref().ok().map(|source| {
                        let glsl = shader_source_no_extensions(source);
                        GlslGraph::parse_glsl(glsl).unwrap()
                    });

                    let premultiplied = frag
                        .as_ref()
                        .map(|frag| is_premultiplied_alpha(&frag.graph).unwrap_or_default())
                        .unwrap_or_default();

                    let anisotropic_rotation = frag
                        .as_ref()
                        .map(|frag| {
                            frag.graph.nodes.iter().any(|n| {
                                // TODO: does this require a more specific query?
                                let query = indoc! {"
                                        prm = prm;
                                        alpha = prm.w;
                                        result = fma(alpha, 2.0, -1.0);
                                    "};

                                query_nodes_glsl(&frag.graph.exprs[n.input], &frag.graph, query)
                                    .is_some()
                            })
                        })
                        .unwrap_or_default();

                    let pixel_metadata = shader_metadata(&binary_folder, pixel_shader);

                    let attrs = vert.as_ref().map(vertex_attributes).unwrap_or_default();

                    // TODO: Don't count comment lines?
                    // This assumes each line of code takes has the same cost.
                    // Some lines will cost more in practice like texture loads.
                    let lines_of_code = pixel_source
                        .as_ref()
                        .map(|s| s.lines().count())
                        .unwrap_or_default()
                        + vertex_source
                            .as_ref()
                            .map(|s| s.lines().count())
                            .unwrap_or_default();

                    // Texture15 is always the shadow map texture.
                    // Shaders with Texture15 also have IN_ShadowMap.
                    // Just check if the shadow map is present for now.
                    // Checking shadow map usage requires mapping decompiled texture handles to uniforms.
                    let receives_shadow = pixel_metadata
                        .as_ref()
                        .map(|p| p.uniforms.iter().any(|u| u.name == "Texture15"))
                        .unwrap_or_default();

                    // Spherical harmonic ambient lighting is passed from the vertex shader.
                    let sh = pixel_metadata
                        .as_ref()
                        .map(|p| p.inputs.iter().any(|i| i.name == "IN_shLighting"))
                        .unwrap_or_default();

                    // Some models with baked lighting don't use the light set.
                    // A negative offset means that the buffer doesn't contain the uniform.
                    let lighting = pixel_metadata
                        .as_ref()
                        .map(|p| {
                            p.uniforms.iter().any(|u| {
                                u.name == "lightDirColor1" && u.uniform_buffer_offset != -1
                            })
                        })
                        .unwrap_or_default();

                    let exprs = if let (Some(vert), Some(frag)) = (vert, frag) {
                        shader_from_glsl(vert, frag)
                    } else {
                        ShaderExprs::default()
                    };

                    let params = material_parameters(program, &exprs.exprs);

                    (
                        program.name.to_string_lossy().into(),
                        ShaderProgram {
                            discard,
                            premultiplied,
                            receives_shadow,
                            sh,
                            lighting,
                            anisotropic_rotation,
                            attributes: attrs,
                            parameters: params,
                            complexity: lines_of_code as f64,
                            exprs,
                        },
                    )
                })
                .collect(),
        };

        // Normalize shader complexity so the highest complexity is 1.0.
        // Prevent a potential division by zero.
        let total_lines_of_code = database
            .programs
            .values()
            .map(|s| s.complexity)
            .reduce(f64::max)
            .unwrap_or_default()
            .max(1.0);

        for s in database.programs.values_mut() {
            s.complexity /= total_lines_of_code;
        }

        database.save(&output_file)?;
    } else {
        error!("Unsupported NUFX version");
    }
    Ok(0)
}

fn shader_metadata(binary_folder: &str, shader: String) -> anyhow::Result<Metadata> {
    let file = Path::new(binary_folder).join(shader).with_extension("bin");
    Metadata::from_file(file).map_err(Into::into)
}

fn shader_source(source_folder: &str, shader: &String) -> Result<String, std::io::Error> {
    let file = Path::new(source_folder).join(shader).with_extension("glsl");
    std::fs::read_to_string(file)
}

fn material_parameters(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    exprs: &[OutputExpr<Op>],
) -> Vec<SmolStr> {
    program
        .material_parameters
        .elements
        .iter()
        .map(|p| {
            let name = p.parameter_name.to_string_lossy();

            if name.contains("Texture") {
                // "Texture0.xyz"
                let channels = texture_color_channels(&name, exprs);
                format_channels(&name, &channels)
            } else if name.contains("CustomVector") {
                // "CustomVector8.xyzw"
                let channels = parameter_color_channels(&name, "nuPerMaterial", exprs);
                format_channels(&name, &channels)
            } else {
                // BlendState0
                name.into()
            }
        })
        .collect()
}

fn texture_color_channels(name: &str, exprs: &[OutputExpr<Op>]) -> BTreeSet<char> {
    exprs
        .iter()
        .filter_map(|e| {
            if let OutputExpr::Value(Value::Texture(t)) = e {
                if t.name == name { t.channel } else { None }
            } else {
                None
            }
        })
        .collect()
}

fn parameter_color_channels(
    name: &str,
    buffer_name: &str,
    exprs: &[OutputExpr<Op>],
) -> BTreeSet<char> {
    exprs
        .iter()
        .filter_map(|e| {
            if let OutputExpr::Value(Value::Parameter(p)) = e
                && p.name == buffer_name
                && p.field == name
            {
                p.channel
            } else {
                None
            }
        })
        .collect()
}

fn vertex_attributes(vertex: &GlslGraph) -> Vec<SmolStr> {
    vertex
        .attributes
        .input_locations
        .left_values()
        .map(|attribute_name| {
            let channels: BTreeSet<_> = vertex
                .graph
                .exprs
                .iter()
                .filter_map(|e| {
                    if let xc3_shader::graph::Expr::Global { name, channel } = e
                        && name == attribute_name
                    {
                        Some((*channel)?)
                    } else {
                        None
                    }
                })
                .collect();

            let attribute_name = attribute_name.trim_start_matches("IN_");
            format_channels(attribute_name, &channels)
        })
        .collect()
}

fn format_channels(name: &str, channels: &BTreeSet<char>) -> SmolStr {
    if !channels.is_empty() {
        let channels: String = "xyzw".chars().filter(|c| channels.contains(c)).collect();
        format_smolstr!("{name}.{channels}")
    } else {
        name.into()
    }
}

fn is_premultiplied_alpha(graph: &Graph) -> Option<bool> {
    let node = graph
        .nodes
        .iter()
        .rfind(|n| n.output.name == "out_attr0" && n.output.channel == Some('w'))?;

    // Check if the RGB outputs are multiplied by alpha.
    let query = indoc! {"
        alpha_final = fma(alpha2, temp, alpha);
        red = temp * alpha;
        green = temp * alpha;
        blue = temp * alpha;
        result.x = red;
        result.y = green;
        result.z = blue;
        result.w = alpha_final;
    "};

    // This handles changes in variable names and algebraic identities like a*b == b*a.
    let result = query_nodes_glsl(&graph.exprs[node.input], graph, query)?;

    Some(!result.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn pixel_source_premultiplied() {
        let source = indoc! {"
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
        let graph = Graph::parse_glsl(source).unwrap();
        assert!(is_premultiplied_alpha(&graph).unwrap_or_default());
    }

    #[test]
    fn pixel_source_premultiplied_commutative() {
        let source = indoc! {"
            void main() {
                temp_743 = fma(temp_736, temp_742, temp_736);
                temp_744 = temp_736 * temp_739;
                temp_745 = temp_736 * temp_740;
                temp_746 = temp_736 * temp_741;
                out_attr0.x = temp_745;
                out_attr0.y = temp_744;
                out_attr0.z = temp_746;
                out_attr0.w = temp_743;
            }
        "};
        let graph = Graph::parse_glsl(source).unwrap();
        assert!(is_premultiplied_alpha(&graph).unwrap_or_default());
    }

    #[test]
    fn pixel_source_premultiplied_different_fma_expr() {
        // SFX_PBS_0d00000000090000
        let source = indoc! {"
            void main() {
                temp_92 = 0.0 - temp_76;
                temp_93 = fma(temp_86, temp_92, temp_76);
                temp_94 = temp_89 * temp_76;
                temp_95 = temp_90 * temp_76;
                temp_96 = temp_91 * temp_76;
                out_attr0.x = temp_94;
                out_attr0.y = temp_95;
                out_attr0.z = temp_96;
                out_attr0.w = temp_93;
            }
        "};
        let graph = Graph::parse_glsl(source).unwrap();
        assert!(is_premultiplied_alpha(&graph).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied() {
        let source = indoc! {"
            void main() {
                temp_733 = temp_730 * temp_680;
                temp_734 = fma(temp_722, fp_c11_data[26].x, temp_732);
                temp_735 = fma(temp_724, fp_c11_data[26].x, temp_732);
                temp_736 = fma(temp_726, fp_c11_data[26].x, temp_732);
                out_attr0.x = temp_734;
                out_attr0.y = temp_735;
                out_attr0.z = temp_736;
                out_attr0.w = temp_733;
            }
        "};
        let graph = Graph::parse_glsl(source).unwrap();
        assert!(!is_premultiplied_alpha(&graph).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied_empty() {
        assert!(!is_premultiplied_alpha(&Graph::default()).unwrap_or_default());
    }
}
