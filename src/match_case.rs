pub fn to_words(num : u32){
    match num{
        1=> println!("One"),
        2..=9 => println!("Two to Nine"),
        _ => println!("More than Nine")
    }
}