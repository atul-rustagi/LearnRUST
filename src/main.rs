
// Course link: https://www.udemy.com/course/rust-programming-the-complete-guide/?couponCode=IND21PM

#[allow (dead_code)]

fn section_2 ()
{
    // tuple
    let tup = (500, "Atul", true);

    println! ("tup: {} {} {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println! ("x: {}", x);
    println! ("y: {}", y);
    println! ("z: {}", z);

    // array
    let arr = [1, 2, 3];

    println! ("arr[0]: {}", arr[0]);

    // vector
    let mut vec = vec! [1, 2, 3]; // vec! => Vec::new ()
    //let vec = Vec::<i32>::with_capacity (3); // vector of sie 3

    vec.push (4);

    println! ("vec[3]: {}", vec[3]);

    // slice, not sure what it is but it is "non owning reference"
    let v: Vec<i32> = (11..=20).collect ();

    let sv = &v[2..=4]; // sv is slice from vector v from index 2 to 4

    println! ("sv: {:?}", sv);

    // String, allocates memory on heap
    let str = String::from ("Atul"); // "Atul".to_string ();

    println! ("str: {}", str);

    // &str => string slice, does not allocate memory on heap
    let ss = "Rustagi";

    println! ("ss: {}", ss);

    // breaking out of a specific loop by giving loops some name
    let mut count = 0;
    'counter: loop { // loop is named as 'counter
        println! ("count: {}", count);
        let mut decrease = 5;
        loop {
            println! ("decrease: {}", decrease);
            if decrease == 4 {
                break; // this will break immediate loop
            }
            if count == 2 {
                break 'counter; // this will break loop named as 'counter
            }
            decrease -= 1;
        }
        count += 1;
    }
}

// Section 3 helper functions

fn takes_ownership (s: String)
{
    println! ("takes_ownership func, s: {}", s);
}

fn give_ownership () -> String
{
    "given".to_string ()
}

fn change_string (orig_str: &mut String)
{
    orig_str.push_str (" World!!");
}

fn section_3 ()
{
    // ownership
    let x = "Atul".to_string ();
    let a = 10;

    // copy
    let b = a;
    println! ("a: {}, b: {}", a, b);

    // move
    let y = x;

    //println! ("x: {}", x); // will not compile as value of x is moved to y and x no longer owns that value
    println! ("y: {}", y);

    // clone
    let z = y.clone (); // this will do deep copy
    println! ("y: {}, z: {}", y, z);

    /*
    datatypes which are on stack implements copy, whereas which are on heap implements move
    tuple will implement copy if all of its elements implement copy
    */

    // move examples
    let s = "takes".to_string ();

    takes_ownership (s); // gives ownership of s to function

    //println! ("s: {}", s); // will not compile as value of s is moved to "take_ownership" parameter

    let t = give_ownership ();
    println! ("t: {}", t);

    // reference and borrowing
    let mut str = "Hello".to_string ();

    change_string (&mut str);

    println! ("str: {}", str);
}

fn main ()
{
    //section_2 ();

    section_3 ();
}
