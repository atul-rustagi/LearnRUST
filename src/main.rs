
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

#[allow (dead_code)]
fn takes_ownership (s: String)
{
    println! ("takes_ownership func, s: {}", s);
}

#[allow (dead_code)]
fn give_ownership () -> String
{
    "given".to_string ()
}

#[allow (dead_code)]
fn change_string (orig_str: &mut String)
{
    orig_str.push_str (" World!!");
}

#[allow (dead_code)]
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

// Section 4 helper things

// named field struct
#[allow (dead_code)]
struct User {
    name: String,
    age: i32,
    active: bool
}

// tuple struct
#[allow (dead_code)]
struct Coordinates (i32, i32, i32);

// unit struct
#[allow (dead_code)]
struct UnitStruct;

#[allow (dead_code)]
struct Rectangle {
    width: i32,
    height: i32
}

#[allow (dead_code)]
impl Rectangle {
    fn area (& self) -> i32
    {
        self.width * self.height
    }

    fn change_width (&mut self, new_width: i32)
    {
        self.width = new_width;
    }
}

#[allow (dead_code)]
fn build_user (name: String, age: i32) -> User
{
    User {
        name,
        age,
        active: true
    }
}

// lifetime annotations
/*
here 'a 'b are lifetime annotations, this means that lifetime of x is till 'a and of b is 'b

3 rules of using lifetime annotations:
- compiler assigns a lifetime parameter to each parameter that's a reference
- if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
- if there are multiple input lifetime parameter, but of them is "&self" or "&mut self" because this is a method,
  lifetime of self is assigned to all output lifetime parameters
*/
#[allow (dead_code)]
fn lifetime_example <'a, 'b> (_x: &'a str, y: &'b str) -> &'b str
{
    y
}

// lifetime in structs
#[allow (dead_code)]
struct MyString <'a> {
    text: &'a str
}

#[allow (dead_code)]
fn section_4 ()
{
    // structs
    let user = User {name: "Atul".to_string (), age: 27, active: true};

    println! ("name: {}, age: {}, isactive: {}", user.name, user.age, user.active);

    let user_1 = build_user("Deepak".to_string (), 31);

    println! ("name: {}, age: {}, isactive: {}", user_1.name, user_1.age, user_1.active);

    let coord = Coordinates (1, 2, 3);

    println! ("Coordinates: {} {} {}", coord.0, coord.1, coord.2);

    // implement methods
    let mut rec = Rectangle {width: 5, height: 5};

    println! ("area: {}", rec.area ());

    rec.change_width (10);

    println! ("area: {}", rec.area ());

    // lifetime
    /*
    let r;

    {
        let x = 10; // binding 'x' declared here
        r = &x; // value is borrowed here - Error
    } // 'x' will become out of scope and will not be available outside this block

    println! ("r: {}", r); // borrowed value used here
    */

    let str = "some text".to_string ();

    let mystr = MyString {text: str.as_str ()}; // by giving lifetime annotation I am making sure that MyString object lives as long as "str"

    println! ("mystr: {}", mystr.text);

    // static lifetime - string literals have static lifetime, stored in binary itself, available always
    let _s: &'static str = "string literal having static lifetime";

}

// Section 5 helper things

// simple enum
#[allow (dead_code)]
enum Pet {
    Dog,
    Cat,
    Fish
}

// implementing methods for enum
#[allow (dead_code)]
impl Pet {
    fn what_am_i (self) -> &'static str {
        match self {
            Pet::Dog => "I am a dog",
            Pet::Cat => "I am a cat",
            Pet::Fish => "I am a fish"
        }
    }
}

#[allow (dead_code)]
enum IpAddrKind {
    V4 (String), // enum can have different types also
    V6
}

#[allow (dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[allow (dead_code)]
fn plus_one (x: Option <i32>) -> Option <i32>
{
    match x {
        None => None,
        Some (i) => Some (i + 1)
    }
}

#[allow (dead_code)]
fn section_5 ()
{
    // enum
    let pet = Pet::Dog;

    println! ("{}", pet.what_am_i ());

    // enum with some type
    let _home = IpAddrKind::V4 ("127.0.0.1".to_string ());

    // enum inside struct
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: "::1".to_string ()
    };

    // option
    let _some_num = Some (10);
    let _some_str = Some ("a string");
    let _nothing: Option<i32> = None;

    // match
    let five = Some (5);
    let six = plus_one (five);
    let none = plus_one (None);

    println! ("six: {:?}, none: {:?}", six, none);

    // if let
    let another_pet = Some (Pet::Dog);
    if let Some (Pet::Dog) = another_pet {
        println! ("pet is a dog");
    } else {
        println! ("pet is not a dog");
    }

    // while let
    let mut st = Vec::new ();

    st.push (1);
    st.push (2);
    st.push (3);

    while let Some (top) = st.pop () {
        println! ("{}", top);
    }

    // more matches
    let x = 10;

    match x {
        1 | 2 => println! ("x is either 1 or 2"),
        3..=10 => println! ("x is in range 3 to 10"),
        _ => println! ("anything else")
    }

    let y = Some (5);
    let z = 10;

    match y {
        Some (15) => println! ("y is 15"),
        Some (y) if y == z => println! ("y is 10"),
        _ => println! ("default")
    }
}

fn main ()
{
    //section_2 ();

    //section_3 ();

    //section_4 ();

    section_5 ();
}
