use crate::annotation::VEC4_SIZE;
use indexmap::IndexMap;
use indoc::indoc;
use log::error;
use rayon::prelude::*;
use serde::Serialize;
use smol_str::{SmolStr, format_smolstr};
use ssbh_data::shdr_data::Metadata;
use std::{borrow::Cow, collections::BTreeMap, error::Error, path::Path};
use xc3_shader::{
    expr::{ExprCache, OutputExpr, output_expr},
    graph::{
        BinaryOp, Graph, UnaryOp,
        glsl::{GlslGraph, merge_vertex_fragment, shader_source_no_extensions},
        query::query_nodes_glsl,
    },
};

mod query;
use query::*;

#[derive(Debug, Serialize)]
struct ShaderDatabase {
    shaders: BTreeMap<String, ShaderProgram>,
}

#[derive(Debug, Serialize)]
struct ShaderProgram {
    discard: bool,
    premultiplied: bool,
    receives_shadow: bool,
    sh: bool,
    lighting: bool,
    anisotropic_rotation: bool,
    attrs: Vec<String>,
    params: Vec<String>,
    complexity: f64,
    // TODO: add ShaderExprs here?
    exprs: ShaderExprs,
}

#[derive(Debug, Default, Serialize)]
pub struct ShaderExprs {
    pub output_dependencies: IndexMap<SmolStr, usize>,
    pub exprs: Vec<OutputExpr<Operation>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default, Serialize)]
pub enum Operation {
    #[default]
    Unk,
    Add,
    Sub,
    Mul,
    Div,
    Fma,
    Min,
    Max,
    Exp2,
    Clamp,
    Negate,
    InverseSqrt,
    Log2,
    Abs,
    Select,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl xc3_shader::expr::Operation for Operation {
    fn query_operation_args<'a>(
        graph: &'a Graph,
        expr: &'a xc3_shader::graph::Expr,
    ) -> Option<(Self, Vec<&'a xc3_shader::graph::Expr>)> {
        // TODO: port basic operations from sm4sh_shader
        // TODO: log errors
        binary_op(graph, expr, BinaryOp::Add, Operation::Add)
            .or_else(|| binary_op(graph, expr, BinaryOp::Sub, Operation::Sub))
            .or_else(|| binary_op(graph, expr, BinaryOp::Mul, Operation::Mul))
            .or_else(|| binary_op(graph, expr, BinaryOp::Div, Operation::Div))
            .or_else(|| op_func(graph, expr, "fma", Operation::Fma))
            .or_else(|| op_func(graph, expr, "min", Operation::Min))
            .or_else(|| op_func(graph, expr, "max", Operation::Max))
            .or_else(|| op_func(graph, expr, "exp2", Operation::Exp2))
            .or_else(|| op_func(graph, expr, "clamp", Operation::Clamp))
            .or_else(|| op_func(graph, expr, "inversesqrt", Operation::InverseSqrt))
            .or_else(|| op_func(graph, expr, "log2", Operation::Log2))
            .or_else(|| op_func(graph, expr, "abs", Operation::Abs))
            .or_else(|| unary_op(graph, expr, UnaryOp::Negate, Operation::Negate))
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

    let exprs = exprs.into_exprs();

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
            shaders: nufx
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

                    // TODO: Perform operations using the parsed graphs instead of source strings.
                    let vert = vertex_source.as_ref().map(|source| {
                        let glsl = shader_source_no_extensions(source);
                        GlslGraph::parse_glsl(glsl).unwrap()
                    });

                    let frag = pixel_source.as_ref().map(|source| {
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
                    let vertex_metadata = shader_metadata(&binary_folder, vertex_shader);

                    let params = material_parameters(
                        program,
                        &vertex_metadata,
                        &pixel_metadata,
                        &vertex_source,
                        &pixel_source,
                    );

                    let attrs = vertex_attributes(program, vertex_metadata, &vertex_source);

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

                    let exprs = if let (Ok(vert), Ok(frag)) = (vert, frag) {
                        shader_from_glsl(vert, frag)
                    } else {
                        ShaderExprs::default()
                    };

                    (
                        program.name.to_string_lossy(),
                        ShaderProgram {
                            discard,
                            premultiplied,
                            receives_shadow,
                            sh,
                            lighting,
                            anisotropic_rotation,
                            attrs,
                            params,
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
            .shaders
            .values()
            .map(|s| s.complexity)
            .reduce(f64::max)
            .unwrap_or_default()
            .max(1.0);

        for s in database.shaders.values_mut() {
            s.complexity /= total_lines_of_code;
        }

        // TODO: Make pretty printing optional.
        let json = serde_json::to_string_pretty(&database).unwrap();
        std::fs::write(output_file, json).unwrap();
    } else {
        error!("Unsupported NUFX version");
    }
    Ok(0)
}

fn shader_metadata(
    binary_folder: &str,
    shader: String,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let file = Path::new(binary_folder).join(shader).with_extension("bin");
    Metadata::from_file(file)
}

fn shader_source(source_folder: &str, shader: &String) -> Result<String, std::io::Error> {
    let file = Path::new(source_folder).join(shader).with_extension("glsl");
    std::fs::read_to_string(file)
}

fn material_parameters(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    vertex_binary_data: &Result<Metadata, Box<dyn std::error::Error>>,
    pixel_binary_data: &Result<Metadata, Box<dyn std::error::Error>>,
    vertex_source: &Result<String, std::io::Error>,
    pixel_source: &Result<String, std::io::Error>,
) -> Vec<String> {
    program
        .material_parameters
        .elements
        .iter()
        .map(|p| {
            let mut name = p.parameter_name.to_string_lossy();

            // TODO: Clean this up.
            if name.contains("Texture") {
                let pixel_channels = texture_color_channels(&name, pixel_binary_data, pixel_source)
                    .unwrap_or_default();

                let channels: String = "xyzw"
                    .chars()
                    .enumerate()
                    .filter(|(i, _)| pixel_channels[*i])
                    .map(|(_, c)| c)
                    .collect();

                if !channels.is_empty() {
                    name = format!("{name}.{channels}")
                }
            } else if name.contains("CustomVector") {
                // Check what Vector4 color channels are used.
                let pixel_channels =
                    vector4_color_channels(&name, "fp_c9_data", pixel_binary_data, pixel_source)
                        .unwrap_or_default();
                let vertex_channels =
                    vector4_color_channels(&name, "vp_c9_data", vertex_binary_data, vertex_source)
                        .unwrap_or_default();

                // Channels may be accessed in either shader.
                let channels: String = "xyzw"
                    .chars()
                    .enumerate()
                    .filter(|(i, _)| pixel_channels[*i] || vertex_channels[*i])
                    .map(|(_, c)| c)
                    .collect();

                if !channels.is_empty() {
                    name = format!("{name}.{channels}")
                }
            }

            name
        })
        .collect()
}

fn texture_color_channels(
    name: &str,
    binary_data: &Result<Metadata, Box<dyn Error>>,
    source: &Result<String, std::io::Error>,
) -> Option<[bool; 4]> {
    let uniform = binary_data
        .as_ref()
        .ok()?
        .uniforms
        .iter()
        .find(|u| u.name == name)?;

    // Check what color channels are used.
    Some(texture_color_channels_from_source(
        &uniform.name,
        source.as_ref().ok()?,
    ))
}

fn texture_color_channels_from_source(texture_name: &str, source: &str) -> [bool; 4] {
    // Assume accesses will be combined like xyzw or xw.
    // TODO: regex?
    let access = format!("({texture_name}");
    let access_line = source.lines().find(|l| l.contains(&access)).unwrap();
    let start = access_line.chars().position(|c| c == '.').unwrap();
    let end = access_line.chars().position(|c| c == ';').unwrap();
    let components = &access_line[start..end];

    let mut channels = [false; 4];
    for (channel, component) in channels.iter_mut().zip("xyzw".chars()) {
        if components.contains(component) {
            *channel = true;
        }
    }

    channels
}

fn vector4_color_channels(
    name: &str,
    buffer_name: &str,
    binary_data: &Result<Metadata, Box<dyn Error>>,
    source: &Result<String, std::io::Error>,
) -> Option<[bool; 4]> {
    let uniform = binary_data
        .as_ref()
        .ok()?
        .uniforms
        .iter()
        .find(|u| u.name == name)?;

    // Check what Vector4 color channels are used.
    Some(vector4_color_channels_from_source(
        uniform,
        source.as_ref().ok()?,
        buffer_name,
    ))
}

fn vector4_color_channels_from_source(
    uniform: &ssbh_data::shdr_data::Uniform,
    source: &str,
    buffer_name: &str,
) -> [bool; 4] {
    let mut channels = [false; 4];
    let vec4_index = uniform.uniform_buffer_offset / VEC4_SIZE as i32;
    for (channel, component) in channels.iter_mut().zip("xyzw".chars()) {
        let access = format!("{buffer_name}[{vec4_index}].{component}");
        if source.contains(&access) {
            *channel = true;
        }
    }

    channels
}

fn vertex_attributes(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    vertex_binary_data: Result<Metadata, Box<dyn std::error::Error>>,
    vertex_source: &Result<String, std::io::Error>,
) -> Vec<String> {
    program
        .vertex_attributes
        .elements
        .iter()
        .map(|a| {
            let mut name = a.attribute_name.to_string_lossy();

            // Check the vertex shader since it uses the same naming conventions.
            // Some attributes are combined before passing to the pixel shader.
            // This may overestimate used channels since we don't include the pixel shader.
            let input_name = format!("IN_{name}");
            if let Some(location) = vertex_binary_data.as_ref().ok().and_then(|data| {
                data.inputs
                    .iter()
                    .find(|i| i.name == input_name)
                    .map(|i| i.location)
            }) && let Ok(vertex) = &vertex_source
            {
                let channels = input_attribute_color_channels(location, vertex);
                if !channels.is_empty() {
                    name = format!("{name}.{channels}")
                }
            }
            name
        })
        .collect()
}

fn input_attribute_color_channels(location: i32, source: &str) -> String {
    // Assume the name is the location like "layout (location = 1) in vec4 in_attr1;"
    let mut channels = String::new();
    for component in "xyzw".chars() {
        let access = format!("in_attr{location}.{component}");
        if source.contains(&access) {
            channels.push(component);
        }
    }

    channels
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

    #[test]
    fn texture_color_channels_source_2d() {
        let channels = texture_color_channels_from_source(
            "fp_tex_tcb_10",
            "temp_10 = texture(fp_tex_tcb_10, vec2(temp_2, temp_4)).zw;",
        );
        assert_eq!([false, false, true, true], channels);
    }

    #[test]
    fn texture_color_channels_source_cube() {
        let channels = texture_color_channels_from_source(
            "fp_tex_tcb_10",
            "temp_10 = textureLod(fp_tex_tcb_10, vec2(temp_2, temp_4)).xzw;",
        );
        assert_eq!([true, false, true, true], channels);
    }
}
