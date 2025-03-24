pub fn ownership_borrowing() {
    println!("============ Владение и заимствование ===============");

    {
        let tmp = -70;
        println!("Внутренний scope: tmp = {}", tmp); 
    }
}