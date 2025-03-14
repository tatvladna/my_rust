use std::thread;
use std::sync::{Arc, Mutex};

pub fn thread() {
    println!("============================ Потоки =============================");

    let mut vec_threads = vec![];

    for i in 1..10{

        // так как используем i, то нужно обязательно использовать move 
        let thread = thread::spawn (move || {
            println!("№{} - Thread ", i);
        });
        vec_threads.push(thread); // добавляем поток в вектор
    }

    // обязательно ждем завершение всех потоков
    // то есть ждем когда vec_threads заполнится
    for th in vec_threads {
        th.join().unwrap();
    }
    println!("End");


    // Воспользуемся безопасным Arc
    let data = Arc::new(vec![1010, 3030, 322, 223]);
    let clone1 = Arc::clone(&data);
    let clone2 = Arc::clone(&data);

    let thread1 = thread::spawn(move || {
        for (i, val) in clone1.iter().enumerate(){
            println!("thread-1 №{} - {}", i, val);
        }
    });

    let thread2 = thread::spawn(move || {
        for (i, val) in clone2.iter().enumerate(){
            println!("thread-2 №{} - {}", i, val);
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("End");

}