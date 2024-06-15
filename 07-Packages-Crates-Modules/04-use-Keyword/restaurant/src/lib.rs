mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    pub fn eat_at_restaurant() {
        // hosting::add_to_waitlist();
        // not working!!!

        // working:
        super::hosting::add_to_waitlist();
    }
}