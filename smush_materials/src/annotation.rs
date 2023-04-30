use ssbh_data::shdr_data::MetaData;
use ssbh_lib::formats::shdr::ShaderStage;

use crate::VEC4_SIZE;

// Annotate the glsl with input and output names.
// Use string replacement instead of the glsl crate to preserve formatting and comments.
pub fn annotate_glsl(
    glsl: String,
    shader_type: &ShaderStage,
    metadata: &MetaData,
) -> Option<String> {
    let mut annotated_glsl = glsl;

    annotate_input_outputs(&mut annotated_glsl, shader_type, metadata);
    annotate_uniforms(&mut annotated_glsl, metadata, shader_type)?;

    Some(annotated_glsl)
}

fn annotate_input_outputs(
    annotated_glsl: &mut String,
    shader_type: &ShaderStage,
    metadata: &MetaData,
) {
    // It's possible to have overlapping identifiers like in_attr1 and in_attr10.
    // Replace names in reverse order to hopefully fix this.
    // TODO: Investigate a more robust solution.
    match shader_type {
        ShaderStage::Vertex => {
            // Vertex inputs have explicit locations.
            for input in metadata.inputs.iter().rev() {
                let glsl_name = format!("in_attr{}", input.location);
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &input.name);
            }
            // Vertex outputs appear in order.
            // TODO: Skip builtins like gl_Position?
            for (i, output) in metadata.outputs.iter().enumerate().rev() {
                let glsl_name = format!("out_attr{i}");
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &output.name);
            }
        }
        ShaderStage::Fragment => {
            // Fragment inputs appear in order.
            for (i, input) in metadata.inputs.iter().enumerate().rev() {
                let glsl_name = format!("in_attr{i}");
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &input.name);
            }
            // Fragment outputs have explicit locations.
            for output in metadata.outputs.iter().rev() {
                let glsl_name = format!("out_attr{}", output.location);
                *annotated_glsl = annotated_glsl.replace(&glsl_name, &output.name);
            }
        }
        _ => (),
    }
}

fn annotate_uniforms(
    annotated_glsl: &mut String,
    metadata: &MetaData,
    shader_type: &ShaderStage,
) -> Option<()> {
    let buffer_prefix = match shader_type {
        ShaderStage::Vertex => Some("vp"),
        ShaderStage::Geometry => None,
        ShaderStage::Fragment => Some("fp"),
        ShaderStage::Compute => None,
    }?;

    // TODO: tcb is texture constant buffer?
    let texture = match shader_type {
        ShaderStage::Vertex => Some("vp_tex_tcb"),
        ShaderStage::Geometry => None,
        ShaderStage::Fragment => Some("fp_tex_tcb"),
        ShaderStage::Compute => None,
    }?;

    // TODO: Don't create assignments for unused parameters?
    let mut assignments = Vec::new();

    for u in metadata.uniforms.iter() {
        // Negative values  like -1 indicate that the entry is unused.
        if u.buffer_index >= 0 && u.uniform_buffer_offset >= 0 {
            // Uniforms are assigned to a particular entry in the buffer list.
            if let Some(buffer_entry) = metadata.buffers.get(u.buffer_index as usize) {
                annotate_buffer_uniform(
                    annotated_glsl,
                    &mut assignments,
                    buffer_entry,
                    u,
                    buffer_prefix,
                );
            }
        } else {
            match u.data_type {
                ssbh_data::shdr_data::DataType::Sampler2d
                | ssbh_data::shdr_data::DataType::Sampler3d
                | ssbh_data::shdr_data::DataType::SamplerCube
                | ssbh_data::shdr_data::DataType::Sampler2dArray
                | ssbh_data::shdr_data::DataType::Image2d => {
                    annotate_texture(annotated_glsl, u, texture);
                }
                _ => (),
            }
        }
    }

    add_variable_assignments(annotated_glsl, assignments);

    Some(())
}

fn annotate_buffer_uniform(
    annotated_glsl: &mut String,
    assignments: &mut Vec<String>,
    buffer_entry: &ssbh_data::shdr_data::Buffer,
    u: &ssbh_data::shdr_data::Uniform,
    buffer_prefix: &str,
) {
    // TODO: Handle the case where unk5 and unk6 are -1?
    // Buffers are selected using an index in the shader code.
    // This is also the binding in the decompiled code.
    let binding = match buffer_entry.unk4 {
        0 => 1, // TODO: how is fp_c1 handled?
        1 => buffer_entry.unk5 + 3,
        2 => buffer_entry.unk6 + 3,
        _ => todo!(),
    };
    // TODO: Will multiple buffer names ever have the same binding?
    // If not, we can replace the uniform buffer names as well.
    let buffer_name = format!("{buffer_prefix}_c{binding}_data");

    // Assume all uniform buffers are of the form "vec4 data[0x1000];".
    let vec4_index = u.uniform_buffer_offset / VEC4_SIZE;

    match u.data_type {
        ssbh_data::shdr_data::DataType::Boolean => {
            // Booleans offsets are accessed as floats and converted to booleans.
            let assignment = annotate_float(
                annotated_glsl,
                u,
                &buffer_name,
                vec4_index,
                &buffer_entry.name,
            );
            assignments.push(assignment);
        }
        ssbh_data::shdr_data::DataType::Float => {
            // Float offsets point to one of the vec4 components.
            let assignment = annotate_float(
                annotated_glsl,
                u,
                &buffer_name,
                vec4_index,
                &buffer_entry.name,
            );
            assignments.push(assignment);
        }
        ssbh_data::shdr_data::DataType::Vector4 => {
            let assignment = annotate_vector4(
                annotated_glsl,
                u,
                &buffer_name,
                vec4_index,
                &buffer_entry.name,
            );
            assignments.push(assignment);
        }
        _ => (),
    }
}

fn annotate_texture(annotated_glsl: &mut String, u: &ssbh_data::shdr_data::Uniform, texture: &str) {
    // Textures are accessed using integer handles.
    // TODO: Figure out the proper name for unk11.
    // TODO: Why do handles in Ryujinx.ShaderTools not match Ryujinx itself?
    let texture_name = texture_handle_name(texture, u.unk11);
    *annotated_glsl = annotated_glsl.replace(&texture_name, &u.name);
}

pub fn texture_handle_name(base: &str, unk11: i32) -> String {
    let handle = unk11 * 2 + 8;
    let texture_name = format!("{base}_{handle:X}");
    texture_name
}

fn annotate_vector4(
    annotated_glsl: &mut String,
    u: &ssbh_data::shdr_data::Uniform,
    buffer: &str,
    vec4_index: i32,
    buffer_name: &str,
) -> String {
    let pattern = format!("{buffer}[{vec4_index}]");

    let uniform_name = uniform_name(u, buffer_name);

    *annotated_glsl = annotated_glsl.replace(&pattern, &uniform_name);

    format!("vec4 {} = {pattern};", uniform_name)
}

fn uniform_name(u: &ssbh_data::shdr_data::Uniform, buffer_name: &str) -> String {
    // Prevent collisions for uniforms with the same name.
    // Also fix invalid characters like sun_shaft_blur_param[0].
    format!("{buffer_name}_{}", u.name)
        .replace('[', "_")
        .replace(']', "_")
}

fn annotate_float(
    annotated_glsl: &mut String,
    u: &ssbh_data::shdr_data::Uniform,
    buffer: &str,
    vec4_index: i32,
    buffer_name: &str,
) -> String {
    let component_offset = u.uniform_buffer_offset - vec4_index * VEC4_SIZE;
    let component = match component_offset {
        0 => "x",
        4 => "y",
        8 => "z",
        12 => "w",
        _ => todo!(),
    };

    let uniform_name = uniform_name(u, buffer_name);

    let pattern = format!("{buffer}[{vec4_index}].{component}");
    *annotated_glsl = annotated_glsl.replace(&pattern, &uniform_name);

    format!("float {} = {pattern};", uniform_name)
}

fn add_variable_assignments(annotated_glsl: &mut String, assignments: Vec<String>) {
    // TODO: Find a more robust way to add the variable assignments to the start of main.
    let mut lines: Vec<String> = annotated_glsl.lines().map(Into::into).collect();
    let assignment_index = lines
        .iter()
        .position(|l| l.contains("void main()"))
        .unwrap_or_default()
        + 2;
    for assignment in &assignments {
        let indented = format!("    {assignment}");
        lines.insert(assignment_index, indented);
    }
    *annotated_glsl = lines.join("\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;
    use ssbh_data::shdr_data::{Attribute, Buffer, DataType, Uniform};

    #[test]
    fn annotate_glsl_fragment() {
        let glsl = indoc! {"
            layout (location = 0) in vec4 in_attr0;
            layout (location = 1) in vec4 in_attr1;
            layout (location = 0) out vec4 out_attr0;

            layout (binding = 0) uniform sampler2D fp_tex_tcb_8;
            layout (binding = 1) uniform sampler2D fp_tex_tcb_12;

            void main() 
            {
                vec4 varVec4 = fp_c9_data[0];
                float varFloat = fp_c9_data[5].y;
                float varFloat2 = fp_c10_data[0].x;
                float varBool = 0 != floatBitsToInt(fp_c9_data[9].z);
                out_attr0 = in_attr0 + in_attr1;
            }
        "}
        .to_string();

        let metadata = MetaData {
            buffers: vec![
                Buffer {
                    name: "nuPerMaterial".to_string(),
                    used_size_in_bytes: 0,
                    uniform_count: 0,
                    unk4: 2,
                    unk5: 0,
                    unk6: 6,
                    unk7: -1,
                },
                Buffer {
                    name: "PerFrame".to_string(),
                    used_size_in_bytes: 0,
                    uniform_count: 0,
                    unk4: 2,
                    unk5: 0,
                    unk6: 7,
                    unk7: -1,
                },
            ],
            uniforms: vec![
                Uniform {
                    name: "CustomVector0".to_string(),
                    data_type: DataType::Vector4,
                    buffer_index: 0,
                    uniform_buffer_offset: 0,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomFloat0".to_string(),
                    data_type: DataType::Float,
                    buffer_index: 0,
                    uniform_buffer_offset: 5 * VEC4_SIZE + 4,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomBoolean0".to_string(),
                    data_type: DataType::Boolean,
                    buffer_index: 0,
                    uniform_buffer_offset: 9 * VEC4_SIZE + 8,
                    unk11: -1,
                },
                Uniform {
                    name: "sun_shaft_blur_param[0]".to_string(),
                    data_type: DataType::Float,
                    buffer_index: 1,
                    uniform_buffer_offset: 0,
                    unk11: -1,
                },
                Uniform {
                    name: "Texture0".to_string(),
                    data_type: DataType::Sampler2d,
                    buffer_index: -1,
                    uniform_buffer_offset: -1,
                    unk11: 0,
                },
                Uniform {
                    name: "Texture1".to_string(),
                    data_type: DataType::Sampler2d,
                    buffer_index: -1,
                    uniform_buffer_offset: -1,
                    unk11: 5,
                },
            ],
            inputs: vec![
                Attribute {
                    name: "attribute0".to_string(),
                    data_type: DataType::Vector4,
                    location: -1,
                },
                Attribute {
                    name: "attribute1".to_string(),
                    data_type: DataType::Vector4,
                    location: -1,
                },
            ],
            outputs: vec![Attribute {
                name: "outAttribute0".to_string(),
                data_type: DataType::Vector4,
                location: 0,
            }],
        };

        pretty_assertions::assert_eq!(
            indoc! {"
                layout (location = 0) in vec4 attribute0;
                layout (location = 1) in vec4 attribute1;
                layout (location = 0) out vec4 outAttribute0;

                layout (binding = 0) uniform sampler2D Texture0;
                layout (binding = 1) uniform sampler2D Texture1;

                void main() 
                {
                    float PerFrame_sun_shaft_blur_param_0_ = fp_c10_data[0].x;
                    float nuPerMaterial_CustomBoolean0 = fp_c9_data[9].z;
                    float nuPerMaterial_CustomFloat0 = fp_c9_data[5].y;
                    vec4 nuPerMaterial_CustomVector0 = fp_c9_data[0];
                    vec4 varVec4 = nuPerMaterial_CustomVector0;
                    float varFloat = nuPerMaterial_CustomFloat0;
                    float varFloat2 = PerFrame_sun_shaft_blur_param_0_;
                    float varBool = 0 != floatBitsToInt(nuPerMaterial_CustomBoolean0);
                    outAttribute0 = attribute0 + attribute1;
                }"
            },
            annotate_glsl(glsl, &ShaderStage::Fragment, &metadata).unwrap()
        );
    }

    #[test]
    fn annotate_glsl_vertex() {
        let glsl = indoc! {"
            layout (location = 1) in vec4 in_attr1;
            layout (location = 10) in vec4 in_attr10;
            layout (location = 0) out vec4 out_attr0;
            layout (location = 1) out vec4 out_attr1;

            void main() 
            {
                vec4 varVec4 = vp_c15_data[0];
                float varFloat = vp_c15_data[5].y;
                float varBool = 0 != floatBitsToInt(vp_c15_data[9].z);
                out_attr0 = in_attr1;
                out_attr1 = in_attr10;
            }
        "}
        .to_string();

        let metadata = MetaData {
            buffers: vec![Buffer {
                name: "nuPerMaterial".to_string(),
                used_size_in_bytes: 0,
                uniform_count: 0,
                unk4: 1,
                unk5: 12,
                unk6: -1,
                unk7: -1,
            }],
            uniforms: vec![
                Uniform {
                    name: "CustomVector0".to_string(),
                    data_type: DataType::Vector4,
                    buffer_index: 0,
                    uniform_buffer_offset: 0,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomFloat0".to_string(),
                    data_type: DataType::Float,
                    buffer_index: 0,
                    uniform_buffer_offset: 5 * VEC4_SIZE + 4,
                    unk11: -1,
                },
                Uniform {
                    name: "CustomBoolean0".to_string(),
                    data_type: DataType::Boolean,
                    buffer_index: 0,
                    uniform_buffer_offset: 9 * VEC4_SIZE + 8,
                    unk11: -1,
                },
            ],
            inputs: vec![
                Attribute {
                    name: "position".to_string(),
                    data_type: DataType::Vector4,
                    location: 1,
                },
                Attribute {
                    name: "normal".to_string(),
                    data_type: DataType::Vector4,
                    location: 10,
                },
            ],
            outputs: vec![
                Attribute {
                    name: "outAttribute0".to_string(),
                    data_type: DataType::Vector4,
                    location: 0,
                },
                Attribute {
                    name: "outAttribute1".to_string(),
                    data_type: DataType::Vector4,
                    location: 1,
                },
            ],
        };

        pretty_assertions::assert_eq!(
            indoc! {"
                layout (location = 1) in vec4 position;
                layout (location = 10) in vec4 normal;
                layout (location = 0) out vec4 outAttribute0;
                layout (location = 1) out vec4 outAttribute1;

                void main() 
                {
                    float nuPerMaterial_CustomBoolean0 = vp_c15_data[9].z;
                    float nuPerMaterial_CustomFloat0 = vp_c15_data[5].y;
                    vec4 nuPerMaterial_CustomVector0 = vp_c15_data[0];
                    vec4 varVec4 = nuPerMaterial_CustomVector0;
                    float varFloat = nuPerMaterial_CustomFloat0;
                    float varBool = 0 != floatBitsToInt(nuPerMaterial_CustomBoolean0);
                    outAttribute0 = position;
                    outAttribute1 = normal;
                }"
            },
            annotate_glsl(glsl, &ShaderStage::Vertex, &metadata).unwrap()
        );
    }
}
