


pub fn is_prime (x:i32) ->i32{
    let mut i = 2;
    let mut res =  0;

    while i< x/2 {
        if x%i == 0 {
            return res;
        }
        i+=1;
    }
    res= 1;
    res 
}


