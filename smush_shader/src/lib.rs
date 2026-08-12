use binrw::BinResult;
use indexmap::IndexMap;
use smol_str::SmolStr;
use std::{collections::BTreeMap, path::Path};
use strum::FromRepr;
use xc3_shader::expr::OutputExpr;

mod io;

#[derive(Debug)]
pub struct ShaderDatabase {
    pub programs: BTreeMap<SmolStr, ShaderProgram>,
}

#[derive(Debug)]
pub struct ShaderProgram {
    pub discard: bool,
    pub premultiplied: bool,
    pub receives_shadow: bool,
    pub sh: bool,
    pub lighting: bool,
    pub anisotropic_rotation: bool,
    pub attrs: Vec<SmolStr>,
    pub params: Vec<SmolStr>,
    pub complexity: f64,
    pub exprs: ShaderExprs,
}

#[derive(Debug, Default)]
pub struct ShaderExprs {
    pub output_dependencies: IndexMap<SmolStr, usize>,
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
    GreaterEqual,
    LessEqual,
    Not,
    LeftShift,
    RightShift,
    BitAnd,
    Pack2Float16,
    Unpack2Float16,
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
