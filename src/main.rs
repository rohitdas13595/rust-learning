mod classes;
mod prime;
mod enums;
mod fibonacci;
mod match_case;

fn factorial(num: u128)-> u128{
    (1..=num).product()
}

fn main() {
    let animal =  classes::Animal{
        name:String::from("dog"),
        genus: String::from("jnbjdfnbj"),
        species: String::from("jnbjdfnbjdfnbj"),
        kingdom: String::from("kmbkdfbdf"),
    };
    println!("{:#?}", animal);
    animal.sound();
    println!("{}",prime::is_prime(17));
    let home = enums::Ip::V4(127, 0, 0 ,1);
    let office =  enums::Ip::V6(String::from("acd.abc.acc.afa"));
    println!("{:#?} {:#?}",office,home);
    fibonacci::fibonacci(10);
    let fact: u128= factorial(20);
    println!("{}", fact);
    match_case::to_words( 20)
}



#[test]
fn factorial_of_21() {
   assert_eq!(51090942171709440000,factorial(22));
}

#[test]
fn factorial_of_0() {
   assert_eq!(1,factorial(0));
}




