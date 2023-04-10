use std::io;

fn get_input()-> io::Result<String>{
    let mut buffer:String  = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}


fn main (){
    let mut vector = vec![];
    let mut times  =0;
    while times <2{
        match get_input() {
            Ok(val)=> {
                vector.push(val);
                times+=1;
            } ,
            Err(e)=> print!("Error: {:?}",e),
        }
    }
    


}