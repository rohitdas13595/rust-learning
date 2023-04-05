pub fn fibonacci(num: i32){
    println!("{}",0);
    println!("{}",1);
    
    let mut i = 2;
    let mut a = 0;
    let mut b = 1;
    while i<=num{
        println!("{}", a+b);
        a=a+b;
        b=a-b;
        i = i +1;
    }
}