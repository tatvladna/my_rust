pub fn oop () {
    println!("========================== OOP ============================");
    /* 
        В Rust ООП построено на:
            -структурах
            - методах
            - метод + ассоциированные функции: объект::функция();
            - трейт (аналогия с интерфесами): обобщенная абстракция, которая может быть использована для нескольких структур.
    */

    let phone1 = Phone {
        model: String::from("iPhone 12 Pro Max"),
        year: 2021,
        price: 1200.0,
        color: String::from("синий")
    };
    dbg!(&phone1);

    phone1.buy();

    // вызов ассоциированной функции
    let world = World::create(120000);
    // это уже неасоциированная функуия
    world.hello();

    // trait
    println!("_________________trait_____________________");
    println!("{}", phone1.print());
    println!("{}", world.print());



    let e1 = Employee {
        name: String::from("John Doe"),
        age: 30,
        salary: 5000.0,
    };

    println!("{}, {}, {}", e1.name, e1.age, e1.salary);
}

// метод + структура
// &self передаем исключительно по ссылке! чтобы не было копирования
impl Phone {
    fn buy(&self) {
        println!("Вы купили телефон:\n
                    - {}\n
                    - {} года\n
                    - {} цена\n
                    - {} цвет", self.model, self.year, self.price, self.color);
    }
}

#[derive(Debug)]
struct Phone {
    model: String,
    year: u32,
    price: f64,
    color: String
}


// структура + ассоциированная функция: "функция должна возвращать структуру (либо Self)", как String::from("Hello world")
struct World {
    number: u64,
}

impl World {

    // ассоциированная функция
    fn create(number: u64) -> World {
        World {number: number}
    }

    fn hello(&self) {
        println!("Hello, World! Number: {}", self.number);
    }
}


trait Printer {
    fn print(&self) -> String;
}

impl Printer for World {
    fn print(&self) -> String {
        format!("World number: {}", self.number)
    }
}

impl Printer for Phone {
    fn print(&self) -> String {
        format!("Phone model: {}, Year: {}, Price: {}", self.model, self.year, self.price)
    }
}


struct Employee <T, S> {
    name: String,
    age: T,
    salary: S,
}