fn main(){
    //BASIC LOOP
    let mut counter = 0;
    let result = loop{ // infinite loop by default (need to break)
        counter += 1;
        if counter == 10{
            break counter; // can return loop value by break with the value
        }
    };
    println!("{}", result);

    //WHILE LOOP
    let mut number = 3;

    while number != 0{
        println!("{}", number);
        number -= 1;
    }

    //FOR LOOP
    let a = [10, 20, 30, 40 ,50];
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in 1..4{ // 1..4 create a list of number started to 1 and ended to 3 (4 is exclusive)
        println!("{}", number);
    }
}
