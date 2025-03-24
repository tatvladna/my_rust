pub fn data_structures() {
    println!("========== Структуры данных ===============");
    // Структура - это коллекция полей
    let developer_job = Job {
        title: TitleJob::Developer,
        company: String::from("Google"),
        salary: 120_000,
        position: Position {
            level: Exp::Junior, 
            years_experience: 1}
    };
    println!("{:?}\n{}\n{}\n{:?}", developer_job.title, developer_job.company, developer_job.salary, developer_job.position);

    // ассоциированные методы: статические методы и методы экземпляра.
    let title_job = String::from("Developer");// статические методы через "::"
    println!("{}", title_job);
    println!("{}", title_job.len()); // методы экземпляра через "."


    // tuple
    let person = ("John", "Doe", 30);
    println!("{:?}", person);

    // enum 

    match developer_job.title {
        TitleJob::Manager => println!("Заработная плата у менеджеров: {}", developer_job.salary),
        TitleJob::Developer => println!("Заработная плата у разработчиков: {} ", developer_job.salary),
        TitleJob::Engineer => println!("Заработная плата у инженеров: {}", developer_job.salary)
    }

    let tom_job = Job {
        title: TitleJob::Engineer,
        company: String::from("Apple"),
        salary: 150_000,
        position: Position {
            level: Exp::Senior,
            years_experience: 10,
        }
    };

    let year_exp = match tom_job.position {
        Position { level: Exp::Senior, years_experience } => years_experience,
        _ => 0, 
    };
    println!("Опыт работы Тома: {}", year_exp);


    let bob_job = Job {
        title: TitleJob::Manager,
        company: String::from("Facebook"),
        salary: 130_000,
        position: Position {
            level: Exp::Middle,
            years_experience: 5,
        }
    };
    println!("Боб работает в компании {} на уровне {:?}", bob_job.company, bob_job.position.level);
}

#[derive(Debug)]
struct Job {
    title: TitleJob, // поля
    company: String,
    salary: u32,
    position: Position
}

#[derive(Debug)]
struct Position {
    level: Exp,
    years_experience: u8

}

#[derive(Debug)]
enum Exp {
    Junior,
    Middle,
    Senior,
}

#[derive(Debug)]
enum TitleJob {
    Manager,
    Developer,
    Engineer,
}