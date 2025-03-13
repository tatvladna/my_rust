// Время жизни ссылок

pub fn link_lifetime() {
    println!("============================== Время жизни ссылок ================================");
    println!("{}", print_mes());
}


fn print_mes<'a>() -> &'a str {
    "hello"
}