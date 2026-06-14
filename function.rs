fn hello_world() {
    println!("this is a call to a function");
}

fn main() {
    hello_world(); // call a function in main function
    num(32, "OG");
    let entity = {
        let price: i32 = 35;
        let qty: i32 = 10;
        price * qty
    };
    println!("res : {}", entity);

    println!("The returning added number : {}", add(1, 3));
}

fn num(number: u32, word: &str) {
    // adding parameter for an number
    println!(
        "this number is : {} dang number, and the word is {} damn word",
        number, word
    );
}

fn add(a: i32, b: i32) -> i32 {
    // function that return value(using -> data type)
    a + b
}
