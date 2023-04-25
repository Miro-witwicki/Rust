mod pizza_order {
    pub struct Pizza {
        pub dough: String, 
        pub cheese: String, 
        pub topping: String
    }

    impl Pizza {
        pub fn new(topping: &str) -> Pizza {
            Pizza {
                dough: String::from("regular dough"),
                cheese: String::from("mozzarela"),
                topping: String::from(topping)
            }
        }
    }

    pub mod help_customer {
        pub fn take_order() {
            seat_at_table();
            
        }
        fn seat_at_table() {
            println!("Customer seated at table");
            let customer_pizza: super::Pizza = super::Pizza::new("Veggies");
            serve_customer(customer_pizza);
        }

        fn serve_customer(cust_pizzza: super::Pizza) {
            println!("The customer is served a regular pizza with specific topping :{}", cust_pizzza.topping)
        } 
     
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}