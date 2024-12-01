pub mod hosting {
    pub fn add_to_wait_list() {
        println!("Added to wait list!");
    }
    pub fn seat_at_table() {
        println!("Sat at the table!");
    }
}

pub mod serving {
    pub fn take_order() {
        println!("Order taken!");
    }
    pub fn serve_order() {
        println!("Order served!");
    }

    pub fn take_payment() {
        println!("Payment received!");
    }
}
