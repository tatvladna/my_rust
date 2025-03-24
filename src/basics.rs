const MY_NAME: &str = "Tanya";
const MY_YEAR: i32 = 2001;

pub fn basics() {
    // сокрытие переменной
    println!("Сокрытие данных");
    let x = 3.3;
    println!("x = {:?}", x);

    let mut x = 5;
    println!("x = {:?}", x);

    // изменение значения только через mut
    println!("Изменение значения через mut");
    let mut y: u8 = 5;
    println!("y = {:?}", y);
    y = 10;
    println!("y = {:?}", y);

    // строковый литерал имеет фиксрованный размер
    let str1: &str = "My name is Tanya";
    println!("{:?}", str1);

    // ключевое слово as: нельзя складывать числа разных типов данных
    let z = y as i32 + x;
    println!("z = {}", z);

    // константы: нужно явным образом объявлять их тип (не копируются)
    println!("My name is {}. I was born in {}", MY_NAME, MY_YEAR);

    // массивы
    let array1 = [2000, 3000, 4000, 5000, 6000, 7000];
    println!("{:?}", array1[2]);


    // функции
    let result = my_add(x as u32, y as u32, z as u32);
    println!("x + y + z = {}", result);

    change_number(&mut x);
    println!("x = {}", x);

    let tuple_swap = swap(35, 95);
    println!("tuple_swap = {:?}", tuple_swap);

    // деструкция кортежа
    let (n1_tuple, n2_tuple )= tuple_swap;
    println!("n1_tuple = {}, n2_tuple = {}", n1_tuple, n2_tuple);

    // Возврат пустого значения (unit)
    // В rust () - конкретный тип данных

    let _ = println!("Hello world!"); // вернет ()


}


fn my_add(a:u32, b: u32, c: u32) -> u32 {
    return a + b + c;
}

fn change_number(number: &mut i32 ) {
    *number += 10;
}

// кортежи часто используются для возврата группы  одинаковых значений
fn swap(a:u32, b:u32) -> (u32, u32) {
    return (b, a);
}