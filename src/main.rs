
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
struct User {
    name: String,
    age: i32,
    active: bool
}

// tuple struct
struct Coordinates (i32, i32, i32);

// unit struct
#[allow (dead_code)]
struct UnitStruct;

struct Rectangle {
    width: i32,
    height: i32
}

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

// Section 6 helper things

// generic struct
#[derive (Debug)]
struct Point <T, U> {
    x: T,
    y: U
}

// operator overloading
use std::ops::Add;

impl <T, U> Add for Point <T, U>
    where
    T: Add <Output = T>,
    U: Add <Output = U> {
        type Output = Self;
        fn add (self, rhs: Self) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y
            }
        }
    }

// trait
trait Overview {
    fn overview (&self) -> String {
        "This is default implementation of Overview trait".to_string ()
    }
}

struct Course {
    title: String,
    author: String
}

struct Book {
    title: String,
    author: String,
    pages: i32
}

impl Overview for Course {
    fn overview (&self) -> String { // if this implementation is removed then course.overview () will return from default implementation
        format! ("{}, {}", self.title, self.author)
    }
}

impl Overview for Book {
    fn overview (&self) -> String {
        format! ("{}, {}, {}", self.title, self.author, self.pages)
    }
}

// passing trait as parameter
//fn call_overview (item: &impl Overview) { // this will also work
fn call_overview <T: Overview> (item: &T) {
    println! ("call_overview: {}", item.overview ());
}

/*
different trait patterns: ***Have to read them again, not able to understand***
- fn call_overview (item1: &impl Overview, item2: &impl Overview)
- fn call_overview <T: Overview> (item1: &T, item2: &T)
- fn call_overview (item: &impl Overview + AnotherTrait)
- fn call_overview <T: Overview + AnotherTrait> (item: &T)
*/

// drop trait
impl Drop for Course {
    fn drop (&mut self) {
        println! ("Dropping Course: {}", self.title);
    }
}

// "clone", "copy", "from", "into" trait not explained completely

#[allow (dead_code)]
fn section_6 ()
{
    let p1 = Point {x: 5.0, y: 10.0};
    let _p2 = Point {x: "5.0", y: "10.0"};
    let _p3 = Point {x: 'x', y: 10};
    let p4 = Point {x: 1.0, y: 2.0};

    let course = Course {title: "Rust Course".to_string (), author: "Atul".to_string ()};
    let book = Book {title: "Dune".to_string (), author: "Frank Herbert".to_string (), pages: 500};

    println! ("Course: {}", course.overview ());
    println! ("Book: {}", book.overview ());

    call_overview (&course);
    call_overview (&book);

    println! ("{}", Overview::overview (&course));

    drop (course); // even if this is line is commented then also "Dropping Course: course name" will get print
                   // this is because at then end of function section_6, course is getting dropped

    println! ("Sum: {:?}", p1 + p4);
}

// Section 9 helper things

use std::fs::File;          // for file operations
use std::fs::rename;        // for renaming the file
use std::io::Error;         // Errors
use std::io::ErrorKind;     // to identify error from error code

#[allow (dead_code)]
fn open_file () -> Result <File, Error>
{
    // `?` operator to extract the `Result<File, std::io::Error>` value, propagating a `Result::Err` value to the caller
    let file = File::open ("target\\xyz.txt")?;

    Ok (file)
}

#[allow (dead_code)]
fn rename_file () -> Result <(), Error>
{
    let file = rename("target\\xyz.txt", "target\\pqr.txt")?;

    Ok (file)
}

#[allow (dead_code)]
fn section_9 ()
{
    /*
    // invoke panic
    // this will unwind the stack before exiting, whereas "abort" will not
    panic! ("panicked here");
    */

    /*
    let v = vec! [1, 2, 3];

    println! ("v[10]: {}", v[10]); // index out of bound panic
    */

    let file = File::open ("target\\sample.txt");

    let _file = match file {
        Ok (f) => f,
        Err (e) => match e.kind () {
            ErrorKind::NotFound => match File::create ("target\\sample.txt") {
                Ok (f) => f,
                Err (_e) => panic! ("cannot create file")
            }
            _ => panic! ("it is some other error")
        }
    };

    /*
    // using unwrap
    let file = File.open ("target\\sample.txt").unwrap (); // return file if Ok, panic if Err

    // customize error msg
    let file = File.open ("target\\sample.txt").expect ("error opening the file"); // will print out error msg during failure
    */

    // error propagation

    // commenting as it will cause panic
    //open_file ().unwrap ();

    // commenting as it will cause panic
    //rename_file ().unwrap ();
}

// Section 12 helper things

#[derive (Debug)]
struct City {
    _name: String,
    population: u64
}

#[allow (dead_code)]
fn sort_cities (cities: &mut Vec<City>)
{
    cities.sort_by_key (sort_cities_helper);
}

#[allow (dead_code)]
fn sort_cities_helper (city: &City) -> u64
{
    city.population
}

fn sort_cities_closure (cities: &mut Vec<City>)
{
    cities.sort_by_key (|p| p.population);
}

#[derive (Debug)]
struct Item {
    name: String
}

fn check_item (items: Vec<Item>, product: String) -> Vec<Item>
{
    items.into_iter ().filter (|i| i.name == product).collect ()
}

#[derive (Debug)]
struct Range {
    start: u32,
    end: u32
}

impl Iterator for Range {
    type Item = u32;

    fn next (&mut self) -> Option<Self::Item> {

        if self.start >= self.end {
            return None;
        }

        let result = Some (self.start);

        self.start += 1;

        result
    }
}

#[allow (dead_code)]
fn section_12 ()
{
    // Closures and Iterators

    // closure are a function like construct that you can store in a variable

    let a = City {_name: "A".to_string (), population: 100};
    let b = City {_name: "B".to_string (), population: 57};
    let c = City {_name: "C".to_string (), population: 140};
    let d = City {_name: "D".to_string (), population: 15};
    let e = City {_name: "E".to_string (), population: 70};

    let mut cities = Vec::new ();

    cities.push (a);
    cities.push (b);
    cities.push (c);
    cities.push (d);
    cities.push (e);

    //sort_cities (&mut cities);

    sort_cities_closure(&mut cities);

    println! ("{:?}", cities);

    let add_v1 = |x: i32| -> i32 {x + 1}; // with type explicitly specified
    let add_v2 = |x| x + 1; // type identified by compiler

    let x = 1;
    println! ("x: {}", add_v1 (x));
    println! ("x: {}", add_v2 (x));

    let example = |v| v;

    let str = example ("example".to_string ());
    println! ("str: {}", str);

    // below code will give compilation error
    //let val = example (10); // once type is identified by compiler, it can't change

    // Fn Traits: Fn, FnMut, FnOnce
    // || drop (v); // FnOnce as it can be run only once
    // |args| v.contains (args); // Fn as it can be run any number of times and not changing v
    // |args| v.push (args); // FnMut as it can be run any number of times and it changes v

    /*
    // not able to run below code because of falcon crowd strike
    let y = 5;
    let add_y = |x| x + y;
    let copy = add_y; // this is closure being copied
    println! ("calling add_y (copy (10)): {}", add_y (copy (10)));

    // below code will give compilation error
    // note: closure cannot be moved more than once as it is not `Copy` due to moving the variable `y` out of its environment
    // help: consider mutably borrowing `add_y`
    let mut y = 5;
    let mut add_y = |x| {y += x; y};
    println! ("calling add_y(10): {}", add_y (10));
    let mut copy = add_y;
    println! ("calling add_y (copy (10): {}", add_y (copy (10)));
    */

    // Iterators

    /*
    pub trait Iterator {
        type Item;
        fn next (&mut self) -> Option <Self::Item>;
        // other default methods
    }
    */

    // https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter

    let vec_1 = vec![1, 2, 3];

    for val in vec_1.iter () {
        print! ("{}", val);
    }

    println! ("");

    let vec_2 = vec![1, 2, 3];
    let mut iter = vec_2.into_iter ();

    while let Some (v) = iter.next () {
        print! ("{}", v);
    }

    println! ("");

    let mut items = Vec::new ();

    items.push (Item {
        name: "shirt".to_string ()
    });

    items.push (Item {
        name: "coat".to_string ()
    });

    items.push (Item {
        name: "shorts".to_string ()
    });

    items.push (Item {
        name: "shoes".to_string ()
    });

    let checked = check_item (items, "shirt".to_string ());

    println! ("shirts: {:?}", checked);

    let range = Range {start: 0, end: 10};
/*
    // commented this as copy trait is not implemented for our Range struct
    // so range is getting moved in for loop, so next use of range will be "used after move" compilation error
    for r in range {
        print! ("{}", r);
    }

    println! ("");
*/

    let even_val: Vec<u32> = range.filter (|x| x % 2 == 0).collect ();

    println! ("even values: {:?}", even_val);
}

use std::rc::Rc;
use std::cell::RefCell;

struct Flagger {
    is_true: Rc<RefCell <bool>>
}

#[allow (dead_code)]
fn section_13 ()
{
    // Box and Dereferencing
    let t = (12, "eggs"); // created on the stack

    let b = Box::new (t); // created on the heap but b was stored on the stack

    println! ("{:?}", b);

    let x = 5;
    let y = &x;
    let z = Box::new (x);

    assert_eq! (5, x);
    assert_eq! (5, *y); // cannot compare integer with reference, so dereference before comparing
    assert_eq! (5, *z);

    println! ("{}", z);

    // Rc and Arc, here Arc is nothing but thread safe Rc or Atomic Rc

    let s1 = Rc::new ("pointer".to_string ());
    let s2 = s1.clone (); // cloning does not copy the value, it creates another pointer and increases the reference count
    let s3 = s2.clone ();

    // methods of underlying objects can be directly used
    println! ("s1 contains \"point\": {}, s2: {}, len of s3: {}", s1.contains ("point"), s2, s3.len ());

    // RefCell
    // it allows us to follow the interior mutability pattern
    // it is a design pattern that allows us to mutate data even when they are immutable references to the data
    // it is enforced at runtime so compiler will typically won't catch any errors for us

    let flag = Flagger {is_true: Rc::new (RefCell::new (true))};

    // borrow returns Ref <T>
    // borrow_mut returns RefMut <T>

    // only one can be borrowed either borrow or borrow_mut until we use Rc
    let reference = flag.is_true.clone ();
    println! ("flag.is_true.borrow (): {:?}", reference);

    let mut mut_reference = flag.is_true.borrow_mut ();
    *mut_reference = false; // dereference first to access
    println! ("flag.is_true.borrow_mut (), changed true to false: {}", mut_reference);
}

fn main ()
{
    //section_2 ();

    //section_3 ();

    //section_4 ();

    //section_5 ();

    //section_6 ();

    //section 7 => cargo, crate, and modules => no programming but folder structuring, check secion 7 commit

    //section 8 => common collections => vectors, binary heap, maps, and sets => learn by using them

    //section_9 ();

    //section 10 => testing => will explore it at then end, basics are very easy

    //section 11 => Find and Replace CLI Program => small coding assignment, do it at then end

    //section_12 ();

    //section_13 ();
}
