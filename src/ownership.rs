pub fn ownership() {
    println!("============================ Владение =============================");

    // обычная область видимость
    {
        let s = 100;
        println!("Внутри блока: {}", s);
    }

    let tmp = "time".to_string();
    drop(tmp);  // elаление значения

    let hello1 = create_hello();
    println!("Привет, {}", hello1);

    let mut hello2 = "hello".to_string();
    let_str(&mut hello2);
    println!("{}", hello2);


    let num = 55;
    let p_num = &num;
    println!("Получаем доступ к переменной через ссылку: {}", p_num);
    let res = *p_num > 100; // в сложных операциях нужно разыменовать *
    println!("Больше ли 100: {}", res);

    {
        println!("Если хотим изменять ссылки, то нужно передавать их с параметром mut");
        let mut number = 100;
            { 
            let p_number = &mut number;
            *p_number = 999;
            println!("p_number = {}", p_number);
            }
        println!("number = {}", number);
    }

}


fn create_hello() -> String {
    String::from("Hello")
}

fn let_str (mes: &mut String){
    mes.push_str("!!!!");
}

