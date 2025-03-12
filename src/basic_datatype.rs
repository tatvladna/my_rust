/*
    Изучение основных типов данных в Rust
*/

const MY_NAME: &str = "Tanya"; // только константы можно выносить

pub fn basic_datatype() {
    println!("My name is {}!", MY_NAME);
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

    let display = ||{println!("анонимная функция")};
    display();

    let mut age:u32 = 23;
    println!("Сегодня Вам: {}", age);
    let mut hpb = || {
        age += 1;
        println!("С днем рождения!");
    };
    hpb();
    hpb();
    hpb();
    println!("Сегодня Вам: {}", age );



    // Указатель — это адрес памяти.
    // Ссылка — это безопасный способ доступа к данным по адресу.
    let number = 10;
    let p_number = &number; // храним адрес переменной, p_number - это ссылка на переменную
    let b_number = *p_number > 50; // с помощью * получаем значение
    println!("b_number = {}", b_number);


    let k = (1, "3", 6);
    println!("{:?}", k);

    /*
        Изучение составных типов:
            кортежи и массивы ИМЕЮT ФИКСИРОВАННЫЙ РАЗМЕР
    */
    let complex = (3.14, 1.59, "hello");
    let (a, b, _) = complex;
    println!("a = {}, b = {}", a, b);
    let complex2 = &complex;
    println!("{:?}", complex2.1);

    let my_data: (&str, u32) = (MY_NAME, 23);

    disp_compl(my_data);
    let my_data2 = my_data;
    disp_compl(my_data2);
    println!("{:?}", my_data);


    let arr1 = ["123", "4677", "789"];
    println!("{:?}", arr1);
    let arr2 = &arr1;
    println!("{:?}", arr2);
    println!("{:?}", arr1);


    bank::value();

    for i in 1..5 {
        println!("Hello, {}", i);
    }

}

fn s(mut n: u32)  {
    println!("до = {}", n);
    n *= 10;
    println!("после = {}", n);
}


fn q(n:u32) -> u32 {
    return n +
    50;
}


fn disp_compl((name, age): (&str, u32))  {
    println!("Имя: {}, Возраст: {}", name, age);
}


mod bank {
    pub fn value() {
        println!("В имеете 0 рублей");
    }
}