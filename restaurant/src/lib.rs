// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     front_of_house::hosting::add_to_waitlist();
// }
//
//
// fn deliver_order() {}
//
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }
//
//     fn cook_order() {}
// }

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::back_of_house::Breakfast;
use crate::back_of_house::Appetizer::Salad;

mod customer {
    pub fn eat_at_restaurant() {
        let mut meal = super::Breakfast::summer("Rye");

        meal.toast = String::from("Wheat");

        println!("I'd like {} toast please", meal.toast);

        let order1 = super::Salad;
        let order2 = super::back_of_house::Appetizer::Soup;
    }
}

//

mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}