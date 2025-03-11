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
    let id: &str = "12345";
    println!("id: {}", id);
    let id: u32 = 1;
    println!("id: {}", id);

    let num = if id == 5 {10} else {0};
    println!("num: {}", num);


    let res: &str = match num {
        0 => "Zero",
        5 => "Five",
        10 => "Ten",
        _ => "непонятно",
    };
    println!("res = {}", res);

    s(id);
    println!("Рузультат все равно не поменялся = {}", id);

    let res2 = q(id);
    println!("res2 = {}", res2);
}

fn s(mut n: u32)  {
    println!("до = {}", n);
    n *= 10;
    println!("после = {}", n);
}


fn q(n:u32) -> u32 {
    return n + 50;
}