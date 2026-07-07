use std::f32::consts::PI;

fn main() {
    // cheatsheet
    // i8, i16, i32, i64, i128 : signed integer
    // u8, u16, u32, u64, u128 : unsigned integer

    let x: i32 = -56979;
    // let y: u32 = 32;

    println!("berak {}", x);

    //float number
    let phi: f32 = PI;
    println!("Value of phi {}", phi);

    let jump: bool = true; // boolean variable
    println!("is jump true{}", jump);

    // array declaration
    let num: [i32; 4] = [1, 2, 3, 4]; 
    println!("num at {}", num[3]);

    //tupl
    let tupl: (String, i32, bool) = ("num".to_string(), 32, true); // you must convert a string
    //_slices to String before use it as a String(dynamic)
    println!("tupl = {:?}", tupl);

    //slices
    let slic: &[&str] = &["tiger", "monkey", "crocs"];
    println!("num slice : {:?}", slic);

    //string
    let strs: String = String::from("say hello world");
    println!("he said : {}", strs);

    let string_slices : &str = "this is an immutable string string_slices";
    println!("{}", string_slices);

    let mut words: String = String::from("this");
    words.push_str(" is a dynamic string"); // editing a string

    println!("var word said : {}", words);

    const NUM : u32 = 3;
    println!("a constant{}", NUM);
    
    // Vector 
    let mut v : Vec<_> = vec![1, 2, 3];
    println!("a Vector{:?}",v);

    let str_this: String = String::from("Hello, Dunia");
    let sliceof_str : &str = &str_this;
    println!("this is slice value : {}", sliceof_str);
}

