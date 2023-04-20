fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let name = "Rohit";
    let add = Box::new(move |a, b| {
        println!("Adding for{:?}",name);
        a + b
    });
    let sub =  Box::new(|i,j| i-j);
    let mul =  Box::new(|i:i32,j:i32| i*j);
    let div =  Box::new(|i:i32,j:i32| i/j);

    println!("{}", math(2,3,add,));
    println!("{:?}",math(3, 2, sub));
    println!("{:?}",math(3, 2, mul));
    println!("{:?}",math(4, 2, div));

}