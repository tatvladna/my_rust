use std::rc::Rc;
use std::sync::Arc;

pub fn smart_pointers () {
    println!("=========== Умные указатели ============");

    let a = 35;
    let b = &a as *const i32 as usize;
    println!("b = {}", b);


    // Большие данные лучше перемещать в кучу, тк стэк ограничен по размеру памяти 
    let my_pie = Box::new(Pie::new(30));
    my_pie.eat();

    // Rc лучше использовать тогда, когда данные должны жить пока, если хотя бы есть 1 владелец
    // Rc нужны если нужно много владельцев
    let heap_pie = Rc::new(Pie::new(80));
    let heap_pie2 = heap_pie.clone();
    let heap_pie3 = heap_pie2.clone();

    heap_pie3.eat();
    heap_pie2.eat();
    heap_pie.eat();


    // Arc для многопоточного кода нужен
    // let data = Arc::new(42);
    // let thread_data = Arc::clone(&data);
}


struct Pie {
    size: usize,
}

impl Pie {
    fn new(size: usize) -> Self {
        Self {
            size,
        }
    }
    fn eat(&self) {
        println!("кушаем пирог, диаметр которого {} см........", self.size);
    }
}