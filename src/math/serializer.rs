use super::{ Node, Expression, Syntax };

pub trait KeywordType 
    where Self: Sized + Clone
{
    fn serialize(name: &str) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Empty {  }
impl KeywordType for Empty {
    fn serialize(_: &str) -> u32 {
        0
    }
}




#[derive(Debug, Clone, Copy, Default)]
#[repr(packed)]
pub struct NodeData {
    // The std140 layout is weird so i make sure that the struct is only 16 bytes in size.
    // Also, for some reason the lo-hi order is reversed.

    // Pack left child index into lo
    pub left_index: u16,                // offset 2, size 2
    // Pack right child index into hi
    pub right_index: u16,               // offset 0, size 2
    
    // Pack syntax identifier into lo
    pub syntax: u16,                    // offset 6, size 2
    // Pack keyword identifier into hi
    pub keyword: u16,                   // offset 4, size 2

    pub value: (f32, f32),              // offset 8, size 8
}

impl Node {
    // Array is sorted in pre-order
    pub fn serialize<V: KeywordType, F: KeywordType>(&self) -> Vec<NodeData> {
        let mut result = Vec::new();
        self.serialize_next::<V, F>(&mut result);
        result
    }
    fn serialize_next<V: KeywordType, F: KeywordType>(&self, result: &mut Vec<NodeData>) {
        let index = result.len();
        result.push(NodeData{
            right_index: 0,
            left_index: 0,
            keyword: self.syntax.serialize_keyword::<V, F>() as u16,
            syntax: self.syntax.serialize_syntax() as u16,
            value: self.syntax.serialize_value(),
        });

        let mut do_left = true;
        let mut do_right = true;
        match self.syntax {
            Syntax::Imaginary(_) |Syntax::Real(_) => {
                do_left = false;
                do_right = false;
            }

            Syntax::Parenthesis | Syntax::Absolute | Syntax::Function(_) => {
                do_right = false;
            }

            _ => (),
        }
        
        
        let mut left = 0;
        if do_left {
            if let Some(node) = &self.left {
                left = result.len();
                node.serialize_next::<V, F>(result);
            }
        }
        let mut right = 0;
        if do_right {
            if let Some(node) = &self.right {
                right = result.len();
                node.serialize_next::<V, F>(result);
            }
        }

        if let Some(parent) = result.get_mut(index) {
            parent.left_index = left as u16;
            parent.right_index = right as u16;
        }
    }
}
impl Syntax {
    pub fn serialize_syntax(&self) -> u32 {
        match self {
            Self::Real(_)       => 0,
            Self::Imaginary(_)  => 1,
            Self::Variable(_)   => 2,
            Self::Function(_)   => 3,

            Self::Parenthesis       => 4,
            Self::Addition          => 5,
            Self::Subtraction       => 6, 
            Self::Multiplication    => 7,
            Self::Division          => 8,
            Self::Exponent          => 9,
            Self::Absolute          => 10,

            _ => 0,
        }
    }
    pub fn serialize_value(&self) -> (f32, f32) {
        match self {
            Self::Real(value) => (*value as f32, 0.0),
            Self::Imaginary(value) => (0.0, *value as f32),
            _ => (0.0, 0.0),
        }
    }
    pub fn serialize_keyword<V: KeywordType, F: KeywordType>(&self) -> u32 {
        match self {
            Self::Variable(variable) => V::serialize(variable),
            Self::Function(function) => F::serialize(function),
            _ => 0,
        }
    }
}

impl Expression {
    pub fn serialize<V: KeywordType, F: KeywordType>(&self) -> Vec<NodeData> {
        match &self.tree {
            Some(tree) => tree.serialize::<V, F>(),
            None => vec![],
        }
    }
}