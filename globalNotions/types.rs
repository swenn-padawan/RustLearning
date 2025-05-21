fn main(){
    //int
    //float
    //bool
    //char

    let a = 98_222; //decimal
    let b = 0xFF; // hexadecimal
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte (u8 only);

    println!("a = {}, b = {}, c = {}, d = {}, e = {}",a,b,c,d,e);

    //i skip simple data type, already knows them.

    //tuples
    //there's 2 ways to get the value desired of a tuple: 1 = index (tuple.index) | 2 = destructured
    let tup = ("Let's get rusty", 100_100);
    let (_channel, _subcount) = tup; // destructured
    let _sub_count = tup.1; // index

    let error_codes = [200, 404, 500];
    let _not_found = error_codes[1];

    let _byte = [0; 8];

    my_function(11);
}

//function
fn my_function(x: i32) {
    println!("Another function = {}", x);
}
