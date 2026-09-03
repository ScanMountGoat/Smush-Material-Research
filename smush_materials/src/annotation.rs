use std::collections::BTreeMap;

use ssbh_data::shdr_data::Metadata;
use ssbh_lib::formats::shdr::ShaderStage;

use glsl_lang::{
    ast::{
        ArraySpecifier, ArraySpecifierData, ArraySpecifierDimensionData, ArrayedIdentifierData,
        Block, Expr, ExprData, Identifier, Node, StructFieldSpecifierData, TranslationUnit,
        TypeSpecifierData, TypeSpecifierNonArrayData,
    },
    parse::DefaultParse,
    transpiler::glsl::{FormattingState, show_translation_unit},
    visitor::{HostMut, Visit, VisitorMut},
};

pub const VEC4_SIZE: u32 = 16;

// TODO: Replace this with xc3_shader eventually.
// Annotate the glsl with input and output names.
// Use string replacement instead of the glsl crate to preserve formatting and comments.
pub fn annotate_glsl(
    glsl: String,
    shader_type: &ShaderStage,
    metadata: &Metadata,
) -> Option<String> {
    let mut replacements = BTreeMap::new();
    let mut struct_fields = BTreeMap::new();
    let mut constant_values = BTreeMap::new();

    annotate_input_outputs(&mut replacements, shader_type, metadata);
    annotate_uniforms(&mut replacements, &mut struct_fields, metadata, shader_type)?;
    annotate_constants(&mut constant_values, metadata)?;

    let mut visitor = Annotator {
        replacements,
        struct_fields,
        constant_values,
    };

    let modified_source = shader_source_no_extensions(&glsl);
    let mut translation_unit = TranslationUnit::parse(modified_source).unwrap();
    translation_unit.visit_mut(&mut visitor);

    let mut new_glsl = String::new();
    show_translation_unit(&mut new_glsl, &translation_unit, FormattingState::default()).unwrap();

    Some(new_glsl)
}

fn shader_source_no_extensions(glsl: &str) -> &str {
    // TODO: Find a better way to skip unsupported extensions.
    glsl.find("#pragma").map(|i| &glsl[i..]).unwrap_or(glsl)
}

fn annotate_input_outputs(
    replacements: &mut BTreeMap<String, String>,
    shader_type: &ShaderStage,
    metadata: &Metadata,
) {
    // It's possible to have overlapping identifiers like in_attr1 and in_attr10.
    // Replace names in reverse order to hopefully fix this.
    // TODO: Investigate a more robust solution.
    match shader_type {
        ShaderStage::Vertex => {
            // Vertex inputs have explicit locations.
            for input in metadata.inputs.iter().rev() {
                let glsl_name = format!("in_attr{}", input.location);
                replacements.insert(glsl_name, input.name.clone());
            }
            // Vertex outputs appear in order.
            // TODO: Skip builtins like gl_Position?
            for (i, output) in metadata.outputs.iter().enumerate().rev() {
                let glsl_name = format!("out_attr{i}");
                replacements.insert(glsl_name, output.name.clone());
            }
        }
        ShaderStage::Fragment => {
            // Fragment inputs appear in order.
            for (i, input) in metadata.inputs.iter().enumerate().rev() {
                let glsl_name = format!("in_attr{i}");
                replacements.insert(glsl_name, input.name.clone());
            }
            // Fragment outputs have explicit locations.
            for output in metadata.outputs.iter().rev() {
                let glsl_name = format!("out_attr{}", output.location);
                replacements.insert(glsl_name, output.name.clone());
            }
        }
        _ => (),
    }
}

fn annotate_uniforms(
    replacements: &mut BTreeMap<String, String>,
    struct_fields: &mut BTreeMap<String, Vec<Field>>,
    metadata: &Metadata,
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
        ShaderStage::Vertex => Some("vp_t_tcb"),
        ShaderStage::Geometry => None,
        ShaderStage::Fragment => Some("fp_t_tcb"),
        ShaderStage::Compute => None,
    }?;

    for (buffer_index, buffer) in metadata.buffers.iter().enumerate() {
        // TODO: Handle the case where unk5 and unk6 are -1?
        // Buffers are selected using an index in the shader code.
        // This is also the binding in the decompiled code.
        let binding = match buffer.unk4 {
            0 => 1, // The constant buffer is handled separately
            1 => buffer.unk5 + 3,
            2 => buffer.unk6 + 3,
            _ => todo!(),
        };
        // TODO: Will multiple buffer names ever have the same binding?
        // If not, we can replace the uniform buffer names as well.
        let buffer_name = format!("{buffer_prefix}_c{binding}");
        let buffer_name_prefixed = format!("_{buffer_prefix}_c{binding}");

        replacements.insert(buffer_name.clone(), buffer.name.clone());
        replacements.insert(buffer_name_prefixed, format!("_{}", buffer.name));

        // Sort to make it easier to convert offsets to sizes.
        // TODO: Why are some offsets negative?
        let mut uniforms: Vec<_> = metadata
            .uniforms
            .iter()
            .filter(|u| u.buffer_index == buffer_index as i32 && u.uniform_buffer_offset >= 0)
            .collect();
        uniforms.sort_by_key(|u| u.uniform_buffer_offset);

        for (uniform_index, uniform) in uniforms.iter().enumerate() {
            // "array[0]" -> "array"
            let uniform_name = uniform
                .name
                .find('[')
                .map(|bracket_index| uniform.name[..bracket_index].to_string())
                .unwrap_or_else(|| uniform.name.to_string());

            // TODO: Is there a dedicated length field on the uniform?
            // The array has elements until the next uniform.
            // Assume uniform buffers have explicit padding fields to follow the std140 layout.
            // Assume the final uniform extends to the end of the buffer.
            let next_offset = uniforms
                .get(uniform_index + 1)
                .map(|u| u.uniform_buffer_offset as u32)
                .unwrap_or(buffer.used_size_in_bytes);

            let length = (next_offset.saturating_sub(uniform.uniform_buffer_offset as u32))
                / uniform_size(uniform.data_type);
            let array_length = if length > 1 { Some(length) } else { None };

            struct_fields
                .entry(buffer_name.clone())
                .or_default()
                .push(Field {
                    name: uniform_name,
                    offset: uniform.uniform_buffer_offset as u32,
                    ty: uniform.data_type,
                    array_length,
                });
        }
    }

    for u in metadata.uniforms.iter() {
        match u.data_type {
            ssbh_data::shdr_data::DataType::Sampler2d
            | ssbh_data::shdr_data::DataType::Sampler3d
            | ssbh_data::shdr_data::DataType::SamplerCube
            | ssbh_data::shdr_data::DataType::Sampler2dArray
            | ssbh_data::shdr_data::DataType::Image2d => {
                annotate_texture(replacements, u, texture);
            }
            _ => (),
        }
    }

    Some(())
}

fn glsl_type(ty: ssbh_data::shdr_data::DataType) -> TypeSpecifierNonArrayData {
    match ty {
        ssbh_data::shdr_data::DataType::Boolean => TypeSpecifierNonArrayData::Bool,
        ssbh_data::shdr_data::DataType::Int => TypeSpecifierNonArrayData::Int,
        ssbh_data::shdr_data::DataType::Unk7 => TypeSpecifierNonArrayData::Void, // TODO: What is this data type?
        ssbh_data::shdr_data::DataType::UnsignedInt => TypeSpecifierNonArrayData::UInt,
        ssbh_data::shdr_data::DataType::UVec3 => TypeSpecifierNonArrayData::UVec3,
        ssbh_data::shdr_data::DataType::Float => TypeSpecifierNonArrayData::Float,
        ssbh_data::shdr_data::DataType::Vector2 => TypeSpecifierNonArrayData::Vec2,
        ssbh_data::shdr_data::DataType::Vector3 => TypeSpecifierNonArrayData::Vec3,
        ssbh_data::shdr_data::DataType::Vector4 => TypeSpecifierNonArrayData::Vec4,
        ssbh_data::shdr_data::DataType::Matrix4x4 => TypeSpecifierNonArrayData::Mat44,
        ssbh_data::shdr_data::DataType::Sampler2d => TypeSpecifierNonArrayData::Sampler2D,
        ssbh_data::shdr_data::DataType::Sampler3d => TypeSpecifierNonArrayData::Sampler3D,
        ssbh_data::shdr_data::DataType::SamplerCube => TypeSpecifierNonArrayData::SamplerCube,
        ssbh_data::shdr_data::DataType::Sampler2dArray => TypeSpecifierNonArrayData::Sampler2DArray,
        ssbh_data::shdr_data::DataType::Image2d => TypeSpecifierNonArrayData::Image2D,
    }
}

fn uniform_size(ty: ssbh_data::shdr_data::DataType) -> u32 {
    match ty {
        ssbh_data::shdr_data::DataType::Boolean => 4,
        ssbh_data::shdr_data::DataType::Int => 4,
        ssbh_data::shdr_data::DataType::Unk7 => 1, // TODO: What is this data type?
        ssbh_data::shdr_data::DataType::UnsignedInt => 4,
        ssbh_data::shdr_data::DataType::UVec3 => 4 * 3,
        ssbh_data::shdr_data::DataType::Float => 4,
        ssbh_data::shdr_data::DataType::Vector2 => 4 * 2,
        ssbh_data::shdr_data::DataType::Vector3 => 4 * 3,
        ssbh_data::shdr_data::DataType::Vector4 => 4 * 4,
        ssbh_data::shdr_data::DataType::Matrix4x4 => 4 * 4 * 4,
        ssbh_data::shdr_data::DataType::Sampler2d => 1,
        ssbh_data::shdr_data::DataType::Sampler3d => 1,
        ssbh_data::shdr_data::DataType::SamplerCube => 1,
        ssbh_data::shdr_data::DataType::Sampler2dArray => 1,
        ssbh_data::shdr_data::DataType::Image2d => 1,
    }
}

fn uniform_item_size(ty: ssbh_data::shdr_data::DataType) -> u32 {
    // The size of an indexed element like "var[3]".
    match ty {
        ssbh_data::shdr_data::DataType::Boolean => 4,
        ssbh_data::shdr_data::DataType::Int => 4,
        ssbh_data::shdr_data::DataType::Unk7 => 1, // TODO: What is this data type?
        ssbh_data::shdr_data::DataType::UnsignedInt => 4,
        ssbh_data::shdr_data::DataType::UVec3 => 4,
        ssbh_data::shdr_data::DataType::Float => 4,
        ssbh_data::shdr_data::DataType::Vector2 => 4,
        ssbh_data::shdr_data::DataType::Vector3 => 4,
        ssbh_data::shdr_data::DataType::Vector4 => 4,
        ssbh_data::shdr_data::DataType::Matrix4x4 => 4 * 4,
        ssbh_data::shdr_data::DataType::Sampler2d => 1,
        ssbh_data::shdr_data::DataType::Sampler3d => 1,
        ssbh_data::shdr_data::DataType::SamplerCube => 1,
        ssbh_data::shdr_data::DataType::Sampler2dArray => 1,
        ssbh_data::shdr_data::DataType::Image2d => 1,
    }
}

fn is_vector(ty: ssbh_data::shdr_data::DataType) -> bool {
    match ty {
        ssbh_data::shdr_data::DataType::Boolean => false,
        ssbh_data::shdr_data::DataType::Int => false,
        ssbh_data::shdr_data::DataType::Unk7 => false,
        ssbh_data::shdr_data::DataType::UnsignedInt => false,
        ssbh_data::shdr_data::DataType::UVec3 => true,
        ssbh_data::shdr_data::DataType::Float => false,
        ssbh_data::shdr_data::DataType::Vector2 => true,
        ssbh_data::shdr_data::DataType::Vector3 => true,
        ssbh_data::shdr_data::DataType::Vector4 => true,
        ssbh_data::shdr_data::DataType::Matrix4x4 => false,
        ssbh_data::shdr_data::DataType::Sampler2d => false,
        ssbh_data::shdr_data::DataType::Sampler3d => false,
        ssbh_data::shdr_data::DataType::SamplerCube => false,
        ssbh_data::shdr_data::DataType::Sampler2dArray => false,
        ssbh_data::shdr_data::DataType::Image2d => false,
    }
}

fn annotate_texture(
    replacements: &mut BTreeMap<String, String>,
    u: &ssbh_data::shdr_data::Uniform,
    base: &str,
) {
    // Textures are accessed using integer handles.
    // TODO: Figure out the proper name for unk11.
    // TODO: Why do handles in Ryujinx.ShaderTools not match Ryujinx itself?
    if u.unk11 >= 0 {
        let texture_name = texture_handle_name(base, u.unk11);
        replacements.insert(texture_name, u.name.clone());
    } else if u.unk10 >= 0 {
        // TODO: is this value always used for vertex textures?
        let texture_name = texture_handle_name(base, u.unk10);
        replacements.insert(texture_name, u.name.clone());
    }
}

pub fn texture_handle_name(base: &str, unk11: i32) -> String {
    let handle = unk11 * 2 + 8;
    format!("{base}_{handle:X}")
}

fn annotate_constants(
    constants: &mut BTreeMap<(usize, char), f32>,
    metadata: &Metadata,
) -> Option<()> {
    for (i, value) in metadata.constant_buffer.iter().enumerate() {
        let vec4_index = i / 4;
        let component_index = i % 4;
        let c = ['x', 'y', 'z', 'w'][component_index];
        constants.insert((vec4_index, c), *value);
    }

    Some(())
}

struct Annotator {
    replacements: BTreeMap<String, String>,
    struct_fields: BTreeMap<String, Vec<Field>>,
    constant_values: BTreeMap<(usize, char), f32>,
}

struct Field {
    name: String,
    offset: u32,
    ty: ssbh_data::shdr_data::DataType,
    array_length: Option<u32>,
}

// TODO: Clean up usage of AST.
impl VisitorMut for Annotator {
    fn visit_identifier(&mut self, ident: &mut Identifier) -> Visit {
        if let Some(name) = self.replacements.get(ident.as_str()) {
            ident.0 = name.into();
        }
        Visit::Children
    }

    fn visit_block(&mut self, block: &mut Block) -> Visit {
        if let Some(fields) = block
            .identifier
            .as_ref()
            .map(|ident| &ident.ident.0)
            .and_then(|i| self.struct_fields.get(i.as_str()))
            && !fields.is_empty()
        {
            block.fields = fields.iter().map(field).collect();
        }

        Visit::Children
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Visit {
        if let ExprData::Dot(e1, c) = &mut expr.content {
            if let ExprData::Bracket(var, specifier) = &mut e1.content
                && let ExprData::IntConst(index) = &specifier.content
                && let ExprData::Dot(id, field) = &var.content
                && let ExprData::Variable(id) = &id.content
                && matches!(id.as_str(), "fp_c1" | "vp_c1")
                && field.as_str() == "data"
                && let Some(constant) = self.constant_values.get(&(
                    (*index).try_into().unwrap(),
                    c.as_str().chars().next().unwrap(),
                ))
            {
                // TODO: Don't hard code the constant buffer name and field?
                *expr = Expr::new(ExprData::FloatConst(*constant), None);
            } else {
                if let ExprData::Bracket(var, specifier) = &mut e1.content
                    && let ExprData::IntConst(index) = &mut specifier.content
                {
                    match &mut var.content {
                        ExprData::Variable(_id) => {
                            // buffer[index].x
                            // TODO: How to handle this case?
                        }
                        ExprData::Dot(e, _c) => {
                            // buffer.field[index].x
                            if let ExprData::Variable(id) = &e.content
                                && let Some(buffer_name) = self.replacements.get(id.as_str())
                                && let Some(fields) = self.struct_fields.get(id.as_str())
                                && let Some(parameter) =
                                    find_glsl_parameter(fields, *index as u32, c.as_str())
                            {
                                // Assume the field is always "data" for now to match Ryujinx.
                                let variable = ExprData::Variable(Identifier::new(
                                    buffer_name.as_str().into(),
                                    None,
                                ));

                                // buffer.uniform
                                let new_expr = Expr::new(
                                    ExprData::Dot(
                                        Box::new(Expr::new(variable, None)),
                                        Identifier::new(parameter.name.as_str().into(), None),
                                    ),
                                    None,
                                );

                                let new_expr = match parameter.array_index {
                                    // buffer.uniform[array_index].x
                                    Some(array_index) => Expr::new(
                                        ExprData::Bracket(
                                            Box::new(new_expr),
                                            Box::new(Node::new(
                                                ExprData::IntConst(array_index as i32),
                                                None,
                                            )),
                                        ),
                                        None,
                                    ),
                                    // buffer.uniform.x
                                    None => new_expr,
                                };

                                *expr = match parameter.channel {
                                    Some(c) => {
                                        Expr::new(ExprData::Dot(Box::new(new_expr), c), None)
                                    }
                                    None => new_expr,
                                };
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        Visit::Children
    }
}

struct GlslParameter {
    name: String,
    array_index: Option<u32>,
    channel: Option<Identifier>,
}

fn find_glsl_parameter(fields: &[Field], vec4_index: u32, channel: &str) -> Option<GlslParameter> {
    // Uniforms in the original shader are always vec4 arrays.
    // Convert accesses like "buffer.data[3].y" to a uniform struct field access.
    fields.iter().find_map(|f| {
        let field_vec4_index = f.offset / VEC4_SIZE;
        // Figure out the index like 1 for y.
        let field_component_offset = match channel {
            "x" => 0,
            "y" => 4,
            "z" => 8,
            "w" => 12,
            _ => todo!(),
        };

        // TODO: Fix array handling for vectors and matrices.
        match f.array_length {
            Some(length) => {
                // Check if the vec4 index falls within this array field.
                if vec4_index >= field_vec4_index {
                    let item_size = uniform_item_size(f.ty);
                    let new_index = (vec4_index - field_vec4_index) * VEC4_SIZE / item_size;

                    if new_index < length {
                        Some(GlslParameter {
                            name: f.name.clone(),
                            array_index: Some(new_index),
                            channel: Some(Identifier::new(channel.into(), None)),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            None => {
                match f.ty {
                    ssbh_data::shdr_data::DataType::Matrix4x4 => {
                        // Treat mat4x4 as a vec4 array.
                        if vec4_index >= field_vec4_index && vec4_index < field_vec4_index + 4 {
                            Some(GlslParameter {
                                name: f.name.clone(),
                                array_index: Some(vec4_index - field_vec4_index),
                                channel: Some(Identifier::new(channel.into(), None)),
                            })
                        } else {
                            None
                        }
                    }

                    ssbh_data::shdr_data::DataType::Vector4 => {
                        if vec4_index == field_vec4_index {
                            Some(GlslParameter {
                                name: f.name.clone(),
                                array_index: None,
                                channel: Some(Identifier::new(channel.into(), None)),
                            })
                        } else {
                            None
                        }
                    }
                    _ => {
                        // Check if the offset is within the range of this uniform.
                        let accessed_offset = vec4_index * VEC4_SIZE + field_component_offset;
                        if (f.offset..f.offset + uniform_size(f.ty)).contains(&accessed_offset) {
                            let channel = if is_vector(f.ty) {
                                let channel_index =
                                    (accessed_offset - f.offset) / uniform_item_size(f.ty);
                                match channel_index {
                                    0 => Some(Identifier::new("x".into(), None)),
                                    1 => Some(Identifier::new("y".into(), None)),
                                    2 => Some(Identifier::new("z".into(), None)),
                                    3 => Some(Identifier::new("w".into(), None)),
                                    _ => todo!(),
                                }
                            } else {
                                // Scalar values like floats only use one of the vec4 channels.
                                None
                            };

                            Some(GlslParameter {
                                name: f.name.clone(),
                                array_index: None,
                                channel,
                            })
                        } else {
                            None
                        }
                    }
                }
            }
        }
    })
}

fn field(field: &Field) -> Node<StructFieldSpecifierData> {
    Node::new(
        StructFieldSpecifierData {
            qualifier: None,
            ty: Node::new(
                TypeSpecifierData {
                    ty: Node::new(glsl_type(field.ty), None),
                    array_specifier: None,
                },
                None,
            ),
            identifiers: vec![Node::new(
                ArrayedIdentifierData {
                    ident: Identifier::new(field.name.as_str().into(), None),
                    array_spec: field.array_length.map(|i| {
                        ArraySpecifier::new(
                            ArraySpecifierData {
                                dimensions: vec![Node::new(
                                    ArraySpecifierDimensionData::ExplicitlySized(Box::new(
                                        Node::new(ExprData::IntConst(i as i32), None),
                                    )),
                                    None,
                                )],
                            },
                            None,
                        )
                    }),
                },
                None,
            )],
        },
        None,
    )
}
