pub fn ownership_borrowing() {
    println!("============ Владение и заимствование ===============");

    {
        let tmp = -70;
        println!("Внутренний scope: tmp = {}", tmp); 
    }

    let data = "My name is Tanya".to_string();
    // меняем владельца, data больше недоступна
    change_owner(data);

    let new_data =return_owner();
    println!("Возвращения владения: new data = {}", new_data);


    let mut s = 10;
    let s_ref = &mut s;
    *s_ref = 50;
    println!("s_ref = {}", s_ref);
    // нельзя использовать s, пока жива s_ref
    let copy_s = *s_ref; // s_ref уничтожается
    println!("copy_s = {}", copy_s);
    s += 10;
    println!("s = {}", s);

    println!("Адрес: {:p}", &copy_s); // убедились, что это копия :)
    println!("Адрес: {:p}", &s);

    let new_s_ref = &mut s;
    *new_s_ref = 100;

    // s+=50;
    println!("new_s_ref= {}", *new_s_ref);


    let mut txt = "Hello".to_string();
    let txt_ref = &mut txt;
    *txt_ref = "Hello World!".to_string();
    println!("Адрес: {:p}", txt_ref);

    let txt_ref_2 = txt_ref; // передается владение 
    println!("{}", &txt_ref_2);
    println!("Адрес: {:p}", &txt_ref_2); // убедились, что это копия :)
    println!("Адрес: {:p}", &txt);

    txt.push_str("!!!!");
    println!("{}", txt);


    let mut exp_str = String::from("Home");
    let new_str = time_live(&mut exp_str);
    println!("{}", new_str);


    let mut str1 = Foo {data: "rrrrrrr".to_string()};
    let mut str2 = Foo {data: "uuuuuuuuuuuuuuuuuuuu".to_string()};
    let res_strs = hard_time_live(&mut str1, &mut str2);
    println!("{:?}", res_strs);




}


fn change_owner(data: String) {
    println!("Изменение владельца: data = {}", data);
}

fn return_owner() -> String {
    "owner".to_string()
}


fn time_live<'a>(ex_str: &'a mut String) -> &'a String {
    ex_str.push_str("~~~~~~~~~~");
    return ex_str;
}

#[derive(Debug)]
struct Foo {
    data: String
}

fn hard_time_live<'a, 'b>(data1: &'a mut Foo, data2: &'b mut Foo) -> &'b Foo {
    data1.data.push_str("==================");
    data2.data.push_str("------------------");

    return data2;
}

