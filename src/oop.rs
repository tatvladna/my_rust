pub fn oop() {
    println!("============== ООП ===============");
    let tanya = Person::new("Tanya", 23);
    println!("Имя: {}, Возраст: {}", tanya.name, tanya.age);

}


struct Person {
    name: String,
    age: u32,
}

// методы
impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age: age,
        }
    }
}