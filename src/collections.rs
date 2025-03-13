// Коллекции - динамические типы данных
use std::collections::HashMap;
use std::collections::HashSet;

pub fn collections() {
    println!("============================ Коллекции =============================");

    let mut v1: Vec<u32> = Vec::new();
    let v2 = vec!["a", "b", "c", "d", "e"];
    let v3 = vec![999; 5];

    v1.push(105);
    v1.push(200);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);


    let mut my_str = String::from("Hello");
    my_str.push(',');
    my_str.push_str(" Tanya!");
    println!("{}", my_str);

    let mut my_map:HashMap<i32, i32> = HashMap::new();
    my_map.insert(1, 100);
    my_map.insert(2, 200);
    println!("{:?}", my_map);

    let mut my_set: HashSet <&str> = HashSet::new();
    my_set.insert("Apple");
    my_set.insert("Banana");
    println!("{:?}", my_set);


    // слайс - часть массива
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let slice: &[i32] = &numbers[1..4];
    println!("{:?}", slice);
    println!("{:?}", &my_str[..6]);



}