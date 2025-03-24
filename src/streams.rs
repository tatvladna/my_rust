pub fn streams() {
    println!("============== Потоки исполнения ============");
    let mut my_age = 14;

    if my_age < 18 {
        println!("Вы не можете проголосовать на выборах");
    } else {
        println!("Вы можете проголосовать на выборах");
    }

    // бесконечный цикл
    loop {
        println!("Переменная my_age: {}", my_age);
        my_age += 1;
        if my_age > 23 {
            println!("Теперь Вы можете проголосовать");
            break;
        }
    }

    while my_age > 14 {
        println!("Переменная my_age: {}", my_age);
        my_age -= 1;
    }
    println!("Вам снова {}", my_age);

    for i in  0..=5 {
        println!("{}", i);
    }
    println!("---------");
    for i in 0..5 {
        println!("{}", i);
    }


    // match: Обычно конструкция match очень большая. И поэтому ее выносят в другую функцию

    match my_age {
        0 => println!("Вы еще не родились"),
        1..18 => println!("Junior"),
        18..=35 => println!("Middle"),
        _ => println!("Senior"),
    }

    // прерывая loop, можно вернуть из него значения

    let result1 = loop {
        my_age += 1;
        if my_age == 23 {
            println!("Увеличи возраст до {}", my_age);
            break "y.o 23";
        }
    };
    println!("Результат первого loop: {}", result1);


    // тернарное выражение
    let result2 = if my_age >= 23 {
        "Вам сейчас 23 лет"
    } else {
        "Вам меньше 23 лет"
    };
    println!("Результат второго loop: {}", result2);

    let my_secrets = {
        let secret = "Это секретное сообщение.  Ваш возраст: ";
        let age = 18;
        format!("{}{}", secret, age)
    };

    println!("Сообщение: {:?}", my_secrets);


    let res_age = age_category(my_age);

    println!("Ваш background: {}", res_age);
}



fn age_category (age: u32) -> &'static str {
    match age {
        0 =>"Вы еще не родились",
        1..18 => "Junior",
        18..=35 => "Middle",
        _ => "Senior",
    }
}