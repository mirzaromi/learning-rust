// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad
    }

    fn fix_incorrect_order () {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

fn deliver_order() {}

pub use crate::front_of_house::hosting;
use back_of_house::Breakfast;
use back_of_house::Appetizer;

mod customer {
    use crate::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    
        // Order a breakfast in the summer with Rye toast
        let mut meal = super::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
    
        println!("I'd like {} toast please", meal.toast);
    
        // The next line won't compile if we uncomment it;
        // we're not allowed to see or modify the seasonal fruit that comes with the meal
    
        // meal.seasonal_fruit = String::from("blueberries");
    
        let order1 = super::Appetizer::Soup;
        let order2 = super::Appetizer::Salad;
    
    }
}


