pub fn text() {
    println!("=========== Текст ============");
    let my_text: &'static str = "Привет, мир!";
    println!("Текст: {}", my_text);


    let a: &'static str = r#"
    <div class="advice">
        Raw strings are useful for some situations.
    </div>
    "#;
    println!("{}", a);

    let hello_html = include_str!("hello.html");
    println!("{}", hello_html);


    // cтроковый слайс
    let s = "Hello, world!";
    let hello_slice = &s[0..5];
    println!("Hello slice: {}", hello_slice);

    let chars = "Hello, world!".chars().collect::<Vec<char>>();
    println!("Chars: {:?}", chars[4]);

    // управляем памятью
    let mut s = String::from("Hello, ");
    s.push_str("Tanya");
    s = s+ "!";
    println!("{}", s);


    print_msg(&s);
    println!("{}", s); // убедились, что изменения внутри print_msg не повлияли на s

    let random_word = ["apple", "banana", "shop"];
    println!("{}", random_word.concat());
    println!("{:?}", random_word.join("~"));

    let format_word = format!("MyShop: {}", random_word[0]);
    println!("{:?}", format_word);

    let n = 55;
    let number_str = n.to_string();
    // parse возвращает тип Result
    let number_dig = number_str.parse::<u8>();

    match number_dig {
        Ok(n) => println!("Number {}", n),
        Err(e) => println!("Error: {}", e),
    }


}


fn print_msg (txt: &str) {
    println!("Сообщение: {}", txt);
}