pub fn oop() {
    use people:: {People, print_msg, generic_fn};
    println!("============== ООП ===============");
    let tanya = Person::new("Tanya", 23);
    println!("Имя: {}, Возраст: {}", tanya.name, tanya.age);


    let bob = people::Person::new("Bob",  21, 150);
    println!("Имя: {}, Возраст: {}, Деньги: {}", bob.get_name(), bob.get_age(), bob.get_money());

    print_msg(&bob);

    generic_fn(&bob);
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


mod people{

    pub struct Person<'a> {
        name: &'a str,
        age: u32,
        money: u32
    }

    pub trait People {
        fn get_name(&self) -> &str;
        fn get_age(&self) -> u32;
        fn get_money(&self) -> u32;
    }

    impl <'a>Person<'a> {
        pub fn new(name: &'a str, age: u32, money: u32) -> Self {
            Self {
                name,
                age,
                money,
            }
        }
    }

    // реализуем трейт для структуры
    impl <'a> People for Person<'a> {
        fn get_name(&self) -> &str {
            &self.name
            }

        fn get_age(&self) -> u32 {
            self.age
        }

        fn get_money(&self) -> u32 {
            self.money
        }
    }


    pub fn print_msg(msg: &dyn People) {
        println!("Hello, {}!", msg.get_name());
    }

    // обобщенные функции тесно связаны с trait
    pub fn generic_fn<T>(cr: &T) 
    where 
        T: People,
    {
        println!("Зарплата: {}", cr.get_money());
    }
    

}