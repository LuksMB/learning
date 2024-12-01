pub mod front_of_house {
    
    pub mod hosting {
        pub fn add_to_wait_list(){
            println!("Added to wait list!");
        }
        pub fn seat_at_table(){
            println!("Sat at the table!");
        }
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}