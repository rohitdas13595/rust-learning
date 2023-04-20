mod greet {
    pub fn hello() {
        println!("Helllo there");
    }
}

mod math{
    pub fn add(x: i32,y: i32) -> i32{
        x+y
    }
}

fn main() {
    use greet::hello;
    hello();
    let val = math::add(5,6);
    print!("{:?}",val );

}