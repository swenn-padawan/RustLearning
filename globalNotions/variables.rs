fn main(){
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6; //mut so can be changed
    println!("The value of x is {}", x);

    const COUNT: u32 = 100_000; //a const variables cant be the value of anything at runtime (func, etc...)

    println!("count  = {}", COUNT);
    

    //same but with shadowing
    let y = 5;
    println!("The value of x is {}", y);
    let y = "six"; //mut so can be changed
    println!("The value of x is {}", y);

    //with shadowing we can changed type, an keep the immutability


}
