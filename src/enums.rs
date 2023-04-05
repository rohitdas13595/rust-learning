#[derive(Debug)]
pub enum Ip{
    V4(i8,i8,i8,i8),
    V6(String)
}