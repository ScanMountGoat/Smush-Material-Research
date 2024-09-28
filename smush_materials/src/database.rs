use std::{error::Error, path::Path};

use indoc::indoc;
use serde::Serialize;
use ssbh_data::shdr_data::MetaData;
use xc3_shader::graph::glsl::shader_source_no_extensions;

use crate::annotation::{texture_handle_name, VEC4_SIZE};

#[derive(Debug, Serialize)]
struct ShaderDatabase {
    shaders: Vec<ShaderProgram>,
}

#[derive(Debug, Serialize)]
struct ShaderProgram {
    name: String,
    discard: bool,
    premultiplied: bool,
    receives_shadow: bool,
    sh: bool,
    lighting: bool,
    attrs: Vec<String>,
    params: Vec<String>,
    complexity: f64,
}

pub fn export_shader_database(
    nufx_file: String,
    binary_folder: String,
    source_folder: String,
    output_file: String,
) -> usize {
    // Generate the shader info JSON for ssbh_wgpu.
    match ssbh_lib::formats::nufx::Nufx::from_file(&nufx_file) {
        Ok(ssbh_lib::formats::nufx::Nufx::V1(nufx)) => {
            // TODO: Make excluding duplicate render pass entries optional?
            // All "SFX_PBS..." programs support all render passes.
            // Only consider one render pass per program since the entries are identical.
            let mut database = ShaderDatabase {
                shaders: nufx
                    .programs
                    .elements
                    .into_iter()
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

                        let premultiplied = pixel_source
                            .as_ref()
                            .map(|source| is_premultiplied_alpha(source).unwrap_or_default())
                            .unwrap_or_default();

                        let pixel_binary_data = shader_binary_data(&binary_folder, pixel_shader);
                        let vertex_binary_data = shader_binary_data(&binary_folder, vertex_shader);

                        let params = material_parameters(
                            &program,
                            &vertex_binary_data,
                            &pixel_binary_data,
                            &vertex_source,
                            &pixel_source,
                        );

                        let attrs = vertex_attributes(&program, vertex_binary_data, &vertex_source);

                        // TODO: Don't count comment lines?
                        // This assumes each line of code takes has the same cost.
                        // Some lines will cost more in practice like texture loads.
                        let lines_of_code =
                            pixel_source.map(|s| s.lines().count()).unwrap_or_default()
                                + vertex_source.map(|s| s.lines().count()).unwrap_or_default();

                        // Texture15 is always the shadow map texture.
                        // Shaders with Texture15 also have IN_ShadowMap.
                        // Just check if the shadow map is present for now.
                        // Checking shadow map usage requires mapping decompiled texture handles to uniforms.
                        let receives_shadow = pixel_binary_data
                            .as_ref()
                            .map(|p| p.uniforms.iter().any(|u| u.name == "Texture15"))
                            .unwrap_or_default();

                        // Spherical harmonic ambient lighting is passed from the vertex shader.
                        let sh = pixel_binary_data
                            .as_ref()
                            .map(|p| p.inputs.iter().any(|i| i.name == "IN_shLighting"))
                            .unwrap_or_default();

                        // Some models with baked lighting don't use the light set.
                        // A negative offset means that the buffer doesn't contain the uniform.
                        let lighting = pixel_binary_data
                            .as_ref()
                            .map(|p| {
                                p.uniforms.iter().any(|u| {
                                    u.name == "lightDirColor1" && u.uniform_buffer_offset != -1
                                })
                            })
                            .unwrap_or_default();

                        ShaderProgram {
                            name: program.name.to_string_lossy(),
                            discard,
                            premultiplied,
                            receives_shadow,
                            sh,
                            lighting,
                            attrs,
                            params,
                            complexity: lines_of_code as f64,
                        }
                    })
                    .collect(),
            };

            // Normalize shader complexity so the highest complexity is 1.0.
            // Prevent a potential division by zero.
            let total_lines_of_code = database
                .shaders
                .iter()
                .map(|s| s.complexity)
                .reduce(f64::max)
                .unwrap_or_default()
                .max(1.0);

            for s in &mut database.shaders {
                s.complexity /= total_lines_of_code;
            }

            // TODO: Make pretty printing optional.
            let json = serde_json::to_string_pretty(&database).unwrap();
            std::fs::write(output_file, json).unwrap();
        }
        Ok(_) => eprintln!("Only NUFX version 1.1 is supported"),
        Err(e) => eprintln!("Error reading {:?}: {:?}", nufx_file, e),
    }
    0
}

fn shader_binary_data(
    binary_folder: &str,
    shader: String,
) -> Result<MetaData, Box<dyn std::error::Error>> {
    let file = Path::new(binary_folder).join(shader).with_extension("bin");
    MetaData::from_file(file)
}

fn shader_source(source_folder: &str, shader: &String) -> Result<String, std::io::Error> {
    let file = Path::new(source_folder).join(shader).with_extension("glsl");
    std::fs::read_to_string(file)
}

fn material_parameters(
    program: &ssbh_lib::formats::nufx::ShaderProgramV1,
    vertex_binary_data: &Result<MetaData, Box<dyn std::error::Error>>,
    pixel_binary_data: &Result<MetaData, Box<dyn std::error::Error>>,
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
    binary_data: &Result<MetaData, Box<dyn Error>>,
    source: &Result<String, std::io::Error>,
) -> Option<[bool; 4]> {
    let uniform = binary_data
        .as_ref()
        .ok()?
        .uniforms
        .iter()
        .find(|u| u.name == name)?;

    // Assume just fragment textures for now.
    let handle_name = texture_handle_name("fp_tex_tcb", uniform.unk11);

    // Check what color channels are used.
    Some(texture_color_channels_from_source(
        &handle_name,
        source.as_ref().ok()?,
    ))
}

fn texture_color_channels_from_source(handle_name: &str, source: &str) -> [bool; 4] {
    // Assume accesses will be combined like xyzw or xw.
    // TODO: regex?
    let access = format!("({handle_name}");
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
    binary_data: &Result<MetaData, Box<dyn Error>>,
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
    let vec4_index = uniform.uniform_buffer_offset / VEC4_SIZE;
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
    vertex_binary_data: Result<MetaData, Box<dyn std::error::Error>>,
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
            }) {
                if let Ok(vertex) = &vertex_source {
                    let channels = input_attribute_color_channels(location, vertex);
                    if !channels.is_empty() {
                        name = format!("{name}.{channels}")
                    }
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

fn is_premultiplied_alpha(source: &str) -> Option<bool> {
    let source = shader_source_no_extensions(source);
    let graph = xc3_shader::graph::Graph::parse_glsl(source).unwrap();
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
    let result = xc3_shader::graph::query::query_nodes_glsl(&node.input, &graph.nodes, query)?;

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

        assert!(is_premultiplied_alpha(source).unwrap_or_default());
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

        assert!(is_premultiplied_alpha(source).unwrap_or_default());
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

        assert!(is_premultiplied_alpha(source).unwrap_or_default());
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

        assert!(!is_premultiplied_alpha(source).unwrap_or_default());
    }

    #[test]
    fn pixel_source_not_premultiplied_empty() {
        assert!(!is_premultiplied_alpha("").unwrap_or_default());
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
