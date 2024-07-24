// add new library by command: cargo new libname --lib
// in course video 56 command "cargo modules generate tree" is now "cargo-modules structure"

mod list {

    pub struct Tasks {
        pub item: String // member needs to be mark public even if struct is already marked as public
    }

    /*
    // Code A
    pub mod things_todo {
        pub fn add_activity () {}
        fn update_activity () {}
        fn mark_completed () {}
    }
    */

    /*
    pub mod items_completed {
        pub fn remove_task () {}
        fn move_back_todo () {}
    }
    */
}

// Code B
mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;

fn lets_add_tasks ()
{
    let task = list::Tasks {item: "Sample task".to_string ()};

    /*
    // Code A
    // relative path
    list::things_todo::add_activity ();

    // absolute path
    crate::list::things_todo::add_activity ();
    */

    // Code B
    // relative path
    things_todo::add_activity ();

    // absolute path
    crate::things_todo::add_activity ();

    // because of "use" statement
    add_activity ();

    items_completed::remove_task ();
}

/*
// default code when added "todo" library

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
