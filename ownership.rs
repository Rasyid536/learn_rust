fn main() {
    let s1 = String::from("RST");
    let s2 = s1;

    println!("s1 : {}", s2);
    // println!("s1 : {}", s1); // this will error because the owner now is s2
    // let len = calculate_length(&s1);
    // println!("isi dari s1 : {}, dan length dari s1 adalah : {}", s1, len);
}

/*
fn calculate_length(s: &String) -> usize {
    s.len()
}
*/
