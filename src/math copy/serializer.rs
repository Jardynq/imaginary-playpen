pub trait Keyword 
    where Self: Sized + Clone + std::str::FromStr
{
    fn serialize(&self) -> u32;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Empty {  }
impl std::str::FromStr for Empty {
    type Err = String;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Err(format!("Empty enum."))
    }
}
impl Keyword for Empty {
    fn serialize(&self) -> u32 {
        0
    }
}
