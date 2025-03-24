use std::collections::HashMap;

pub fn generic_types () {
    println!("=========== Обобщенные типы данных =====================");

    let first_data = MyGenericData::<i32> { data: 35};
    println!("{:?}", first_data.data);

    let second_data = MyGenericData::<&str> { data: "Hello, World!"};
    println!("{:?}", second_data.data);

    let third_data = MyGenericData { data: HashMap::from([(1, "1".to_string()),  (2,"2".to_string()), (3, "3".to_string())]) };
    println!("{:?}", third_data.data);

    let fourth_data = third_data;
    println!("{:?}", fourth_data.data); // Обобщенный тип не может быть изменен после его создания
    //  благодаря .to_string() нельзя реализовать Copy, только владения (&str превращается в String)
    // println!("{:?}", third_data.data);


    let mut fifth_data = MyAnotherData::<i32> { data: Some(50)};
    println!("{:?}", fifth_data.data);
    fifth_data.data = None;
    println!("{:?}", fifth_data.data);

    fifth_data.data = Some(35);

    match fifth_data.data {
        Some(fifth_data) => println!("{:?}", fifth_data),
        None => println!("None"),
    }

    if fifth_data.data.is_some() {
        println!("Some data: {:?}", fifth_data.data);
    } else {
        println!("No data");
    }

    if let Some(s) = fifth_data.data {
        println!("{:?}", s);
    }

    let a = -10;
    let result1 = find_bag(a);
    println!("{:?}", result1);

    match result1 {
        Ok(value) => println!("Found: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    let my_result = my_main();
    match my_result {
        Ok(()) => println!("My_main() - Ok(())"),
        Err(error) => println!("My_main() - Err({})", error),
    }

    let result3 = main_find_bag();
    println!("{:?}", result3);

    let result4 = my_main().unwrap(); // unwrap - грубый способ обработки ошибок, лучше так не делать, после него программа выключается
    println!("{:?}", result4);


    // Векторы
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);
    println!("{:?}", v[2]); // доступ по индексу

    let mut v2 = Vec::new();
    v2.push(1);
    v2.extend([6, 7, 8].iter());
    println!("{:?}", v2);

    for i in 0..v2.len() {
        println!("{:?}", v2[i]);
    }

    // или
    println!("Другой способ: ");
    for i in v2.iter() {
        println!("{:?}", i);
    }


}

#[derive(Debug)]
struct MyGenericData <T> {
    data: T,
}


// встроенные тип Option
#[derive(Debug)]
struct MyAnotherData <T> {
    data: Option<T>,
}


// встроенный тип Result - обработка ошибок

fn find_bag (num: isize) -> Result<String, String>{
    match num {
        1..1000 => Ok("yes".to_string()),
        _ => Err("Число отрицательно и меньше 1000".to_string()),
    }
}

fn main_find_bag () ->Result <(), String> {
    let a = find_bag(-500)?;
    println!("found: {}", a);
    Ok(())
}


fn my_main () -> Result<(), String> {
    let res = find_bag(10);
    match res {
        Ok(value) => println!("Found: {}", value),
        Err(error) => {
            return Err(error)
        }
    };

    Ok(())
}