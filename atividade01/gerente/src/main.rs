fn main() {
    println!("Starting tests!");
    restaurant::front_of_house::hosting::add_to_wait_list();
    restaurant::front_of_house::hosting::seat_at_table();
    
    restaurant::front_of_house::serving::take_order();
    restaurant::front_of_house::serving::serve_order();
    restaurant::front_of_house::serving::take_payment();

    restaurant::back_of_house::take_care_trash();

    let pi = restaurant::pi::PI;
    println!("This is the value of PI to this restaurant: {pi}")
}
