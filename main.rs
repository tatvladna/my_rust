fn main() {
    let mut my_name = "Tanya";
    println!("Hello, {}!", my_name);
    my_name = "Bob";
    println!("Hello, {}!", my_name);

    let r = '🦀';
    println!("bytes: {} ", std::mem::size_of_val(&r));


    // переопределять нельзя, но можно "затенить"
    // затенение 
    let id: u32 = 555;
    println!("id: {}", id);
    let id: u32 = 1;
    println!("id: {}", id);
    let id: &str = "12345";
    println!("id: {}", id);

}