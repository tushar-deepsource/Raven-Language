use std::fmt::{Display, Formatter};

pub mod blocks;
pub mod r#struct;
pub mod code;
pub mod function;
pub mod types;
pub mod type_resolver;

pub static MODIFIERS: [Modifier; 5] = [Modifier::Public, Modifier::Protected, Modifier::Extern,
    Modifier::Internal, Modifier::Operation];
#[derive(Clone, Copy)]
pub enum Modifier {
    Public = 0b1,
    Protected = 0b10,
    Extern = 0b100,
    Internal = 0b1000,
    Operation = 0b1_0000,
}

impl Display for Modifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Modifier::Public => write!(f, "pub"),
            Modifier::Protected => write!(f, "pub(proj)"),
            Modifier::Extern => write!(f, "extern"),
            Modifier::Internal => write!(f, "internal"),
            Modifier::Operation => write!(f, "operation")
        }
    }
}

pub fn get_modifier(modifiers: &[Modifier]) -> u8 {
    let mut sum = 0;
    for modifier in modifiers {
        sum += modifier.clone() as u8;
    }

    return sum;
}

#[inline]
pub fn is_modifier(modifiers: u8, target: Modifier) -> bool {
    let target = target as u8;
    return modifiers & target != 0;
}

pub fn to_modifiers(from: u8) -> Vec<Modifier> {
    let mut modifiers = Vec::new();
    for modifier in MODIFIERS {
        if from & (modifier as u8) != 0 {
            modifiers.push(modifier)
        }
    }

    return modifiers;
}

pub trait DisplayIndented {
    fn format(&self, parsing: &str, f: &mut Formatter<'_>) -> std::fmt::Result;
}

pub struct Attribute {
    pub value: String
}

impl Attribute {
    pub fn new(value: String) -> Self {
        return Self {
            value
        }
    }
}