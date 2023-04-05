#[derive(Debug)]
pub struct Animal{
    pub name: String,
    pub genus:String,
    pub species: String,
    pub kingdom: String
}
impl Animal{
    pub fn sound(&self){
        println!("baf!");
    }
}

