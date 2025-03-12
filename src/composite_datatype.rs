/*
    Изучение составных типов данных
*/


pub fn composite_datatype() {

    println!("============================ Составные типы данных =============================");

    // Главное отличие кортежей от массивов: кортежи могут иметь неоднородную структуру, а массивы только однородную.
    // Сходства: Оба имеют фиксированные размеры

    let tuple: (i32, f64, char) = (10, 3.14, 'A');
    println!("Кортеж tuple: {:?}", tuple);

    let tuple2 = (2, 5.2, '2');

    if tuple == tuple2 {
        println!("Кортежи tuple и tuple2 равны");
    } else {
        println!("Кортежи можно сравнивать только, если у них одинаковые типы данных");
        println!("Кортежи tuple и tuple2 не равны");
    }

    // Кортежи можно копировать только, если все значения являются простыми (статическими)
    // к динамическим данным нужно применять операцию клонирования

    let copy_tuple = tuple;
    println!("Копия кортежа tuple: {:?}", copy_tuple);
    println!("Исходный кортеж tuple остался доступным: {:?}", tuple);

    // К динамическим типам данным относят строки, векторы, сеты, мэпы
    // В целях безопасности и рационального использовани япамяти такие данные нельзя копировать (то есть выделять память по еще их одну копию)
    let complex_tuple = ("1234".to_string(), "Hello".to_string(), String::from("Tanya"));
    println!("Кортеж complex_tuple: {:?}", complex_tuple);
    let mut try1 = complex_tuple; // перемещение, complex_tuple перестает существовать
    println!("{:?}", try1);
    
    let try2 = try1.clone(); // чтобы не затирать старую переменную для динамических типов данных нужно использовать клонирование
    println!("{:?}", try1);
    println!("{:?}", try2);


    // ссылка — это просто указатель на исходные данные
    let try3 = &mut try1;
    println!("try3: {:?}", try3);

    try3.0 = String::from("Hello, world!");
    println!("{:?}", try3); // изменяется и try1, и try3

    let tuple3 = modif_tuple(try1.clone());
    println!("tuple3: {:?}", tuple3); 
    println!("try1 сохранился, так как сделали его клоинрование: {:?}", try1);

    let try4 = &mut try1;
    println!("try4: {:?}", try4);
    let try5 = &mut try1;
    println!("try5: {:?}", try5);

    try5.1 = "rrrrrrrrrrrrrrrrrrrrr".to_string();
    println!("try1 после изменения try5: {:?}", try1);


    let array1 = ["Tanya", "Bob", "Vlad", "Roman"];
    for i in 1..array1.len() {
        println!("array1[{}]: {}", i, array1[i]);
    }


    let mut tanya = People{
        name: "Tanya".to_string(), 
        age: 23,
        height: 150};

    println!("name = {}, age = {}", tanya.name, tanya.age);

    tanya.age = 25;
    println!("name = {}, age = {}", tanya.name, tanya.age);

    let bob = People {
        name: "Bob".to_string(),
        ..tanya
    };

    println!("name = {}, age = {}", bob.name, bob.age);
    println!("height = {}", bob.height);

    let animal = create_animal("Lion".to_string(), "Cat".to_string(), 2);
    println!("name = {}, type_animal = {}, age = {}", animal.name, animal.type_animal, animal.age);

    let n = 10;
    match n {
        n if n % 2 == 0 => println!("Even number"),
        n if n % 2!= 0 => println!("Odd number"),
        0 => println!("Zero"),
        _ => println!("Unknown number"),
    }



}

fn modif_tuple(mut tuple: (String, String, String)) -> (String, String, String) {
    tuple.0.push_str(" - 1");
    tuple.1.push_str(" - 2");
    tuple.2.push_str(" - 3");

    return tuple // любительница явно возвращать переменную
}


struct People {
    name: String,
    age: u32, 
    height: u32,
}
struct Animal {
    name: String,
    type_animal: String,
    age: u32,
}

fn create_animal(name: String, type_animal: String, age: u32) -> Animal {
    return Animal {name, type_animal, age}
}  