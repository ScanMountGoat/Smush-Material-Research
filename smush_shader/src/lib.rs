use binrw::{BinReaderExt, BinResult};
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::{collections::BTreeMap, io::Cursor, path::Path};
use strum::FromRepr;

pub use xc3_shader::expr::{Attribute, OutputExpr, Parameter, Texture, Value};

mod io;

#[derive(Debug)]
pub struct ShaderDatabase {
    pub programs: BTreeMap<SmolStr, ShaderProgram>,
}

#[derive(Debug, PartialEq)]
pub struct ShaderProgram {
    /// `true` if the code contains "discard;" and likely has alpha testing.
    pub discard: bool,
    /// `true` if the fragment RGB outputs are multiplied by the alpha output value.
    pub premultiplied: bool,
    /// `true` if the fragment shader has a shadow map texture and will render shadows.
    /// This does not affect casting shadows on other objects.
    pub receives_shadow: bool,
    /// `true` if the fragment shader uses per vertex spherical harmonic ambient lighting.
    pub sh: bool,
    /// `true` if the fragment shader uses the light set directional lighting.
    pub lighting: bool,
    /// `true` if the fragment shader rotates the tangent and bitangent vectors for anisotropic shading.
    pub anisotropic_rotation: bool,
    /// The collection of required mesh vertex attributes and their accessed channels.
    pub attributes: Vec<SmolStr>,
    /// The collection of required material parameters and their accessed channels.
    pub parameters: Vec<SmolStr>,
    /// A heuristic for shader complexity in the range `0.0` to `1.0`.
    pub complexity: f64,
    pub exprs: ShaderExprs,
}

#[derive(Debug, PartialEq, Default)]
pub struct ShaderExprs {
    pub output_dependencies: IndexMap<SmolStr, usize>,
    pub discard_condition: Option<usize>,
    pub exprs: Vec<OutputExpr<Operation>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default, FromRepr)]
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
    Sqrt,
    Floor,
    Trunc,
    Sin,
    Cos,
    Select,
    IntBitsToFloat,
    UintBitsToFloat,
    FloatBitsToInt,
    FloatBitsToUint,
    Int,
    Uint,
    Float,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Not,
    And,
    Or,
    LeftShift,
    RightShift,
    BitAnd,
    Pack2Float16,
    Unpack2Float16,
    IsNaN,
}

impl ShaderDatabase {
    /// Load the database data from `path`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> BinResult<Self> {
        // Store non indexed programs to avoid converting an indexed program more than once.
        let indexed = io::ShaderDatabaseIndexed::from_file(path)?;
        Ok(Self {
            programs: indexed.programs(),
        })
    }

    /// Load the database data from `bytes`.
    pub fn from_bytes(bytes: &[u8]) -> BinResult<Self> {
        // Store non indexed programs to avoid converting an indexed program more than once.
        let mut reader = Cursor::new(bytes);
        let indexed: io::ShaderDatabaseIndexed = reader.read_le()?;
        Ok(Self {
            programs: indexed.programs(),
        })
    }

    /// Serialize and save the database data to `path`.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> BinResult<()> {
        let indexed = io::ShaderDatabaseIndexed::from_programs(&self.programs);
        indexed.save(path)?;
        Ok(())
    }

    pub fn get_shader(&self, shader_id: &str) -> Option<&ShaderProgram> {
        self.programs.get(shader_id)
    }

    /// Create the internal database representation from non indexed data.
    pub fn from_programs(programs: BTreeMap<SmolStr, ShaderProgram>) -> Self {
        Self { programs }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl ShaderProgram {
    /// Returns `true` if `attributes` has all the vertex attributes required by this shader program.
    // TODO: Take an iterator instead?
    pub fn has_required_attributes(&self, attributes: &[String]) -> bool {
        self.attributes
            .iter()
            .map(|a| attribute_name_no_channels(a))
            .filter(|a| *a != "ink_color_set")
            .all(|required| attributes.iter().any(|a| a == required))
    }

    /// Returns the vertex attribute names required by this shader program not present in `attributes`.
    // TODO: Take an iterator instead?
    pub fn missing_required_attributes(&self, attributes: &[String]) -> Vec<String> {
        self.attributes
            .iter()
            .map(|a| attribute_name_no_channels(a))
            .filter(|required| {
                *required != "ink_color_set" && !attributes.iter().any(|a| a == required)
            })
            .map(|a| a.to_string())
            .collect()
    }

    /// Returns the color channels accessed by the shaders as `[x, y, z, w]`.
    pub fn accessed_channels(&self, param_name: &str) -> [bool; 4] {
        let mut channels = [false; 4];
        if let Some(database_param) = self.parameters.iter().find(|p| p.starts_with(param_name)) {
            let (_, components) = split_param(database_param);
            for (i, c) in "xyzw".chars().enumerate() {
                channels[i] = components.contains(c);
            }
        }
        channels
    }

    /// Returns `true` if this program requires `attribute`.
    pub fn has_attribute(&self, attribute: &str) -> bool {
        self.attributes
            .iter()
            .map(|a| attribute_name_no_channels(a))
            .any(|a| a == attribute)
    }
}

fn attribute_name_no_channels(attribute: &str) -> &str {
    // "map1.xy" -> "map1"
    // "map1" -> "map1"
    attribute.split_once('.').map(|a| a.0).unwrap_or(attribute)
}

/// Splits `param` into its parameter name and accessed components.
///
/// # Examples
/**
```rust
use smush_shader::split_param;

assert_eq!(("CustomBoolean3", ""), split_param("CustomBoolean3"));
assert_eq!(("CustomVector0", "x"), split_param("CustomVector0.x"));
assert_eq!(("CustomVector12", ""), split_param("CustomVector12."));
*/
pub fn split_param(param: &str) -> (&str, &str) {
    param
        .find('.')
        .map(|i| {
            (
                param.get(..i).unwrap_or(""),
                param.get(i + 1..).unwrap_or(""),
            )
        })
        .unwrap_or((param, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_program() -> ShaderProgram {
        ShaderProgram {
            discard: false,
            premultiplied: false,
            receives_shadow: false,
            sh: false,
            lighting: false,
            anisotropic_rotation: false,
            attributes: Vec::new(),
            parameters: Vec::new(),
            complexity: 0.0,
            exprs: Default::default(),
        }
    }

    #[test]
    fn has_required_attributes_empty() {
        assert!(
            ShaderProgram {
                attributes: Vec::new(),
                ..default_program()
            }
            .has_required_attributes(&[])
        );
    }

    #[test]
    fn has_required_attributes_extras() {
        assert!(
            ShaderProgram {
                attributes: Vec::new(),
                ..default_program()
            }
            .has_required_attributes(&["abc".to_string()])
        );
    }

    #[test]
    fn has_required_attributes_missing() {
        assert!(
            !ShaderProgram {
                attributes: vec!["a".into(), "b".into()],
                ..default_program()
            }
            .has_required_attributes(&["a".to_string()])
        );
    }

    #[test]
    fn has_required_attributes() {
        // Make sure the channel extensions are ignored.
        assert!(
            ShaderProgram {
                attributes: vec!["a.xz".into(), "b.w".into()],
                ..default_program()
            }
            .has_required_attributes(&["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn has_required_attributes_ink_color_set() {
        // Check that "ink_color_set" is ignored since it isn't part of the mesh
        // TODO: Investigate how this attribute is generated.
        assert!(
            ShaderProgram {
                attributes: vec!["ink_color_set".into(), "map1".into()],
                ..default_program()
            }
            .has_required_attributes(&["map1".to_string()])
        );
    }

    #[test]
    fn missing_required_attributes_empty() {
        // Check that "ink_color_set" is ignored since it isn't part of the mesh
        // TODO: Investigate how this attribute is generated.
        assert!(
            ShaderProgram {
                attributes: Vec::new(),
                ..default_program()
            }
            .missing_required_attributes(&[])
            .is_empty()
        );
    }

    #[test]
    fn missing_required_attributes_ink_color_set() {
        // Check that "ink_color_set" is ignored since it isn't part of the mesh
        // TODO: Investigate how this attribute is generated.
        assert_eq!(
            vec!["map1".to_string()],
            ShaderProgram {
                attributes: vec!["ink_color_set".into(), "map1.xy".into()],
                ..default_program()
            }
            .missing_required_attributes(&[])
        );
    }
}
