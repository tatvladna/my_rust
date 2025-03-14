pub fn error() {
    println!("=========================== Ошибки ============================");
    let n = 10;
    // if n < 100 {
    //     panic!("Error");
    // }

    // println!("Продолжение выполнения");

    let result = if n < 100 {
        Result::Err("Число слишком маленькое".to_string())
    } else {
        Result::Ok(n)
    };

    match result {
        Result::Ok(value) => println!("Продолжение выполнения: {}", value),
        Result::Err(err) => println!("Ошибка: {}", err),
    }

}


// enum часто используется для обработки ошибок
enum Result <T, S> {
    Ok(T),
    Err(S),
}