
/*
#![allow (warnings)]   // disables all warnings
#![deny (warnings)]    // enables all warnings
*/

#![allow (dead_code)]

use std::collections::HashMap;

fn add (num1: i32, num2: i32) -> i32
{
    num1 + num2
}

fn basics ()
{
    let a;               // variable declared
    let b = 10;     // variable declared and initialized

    let mut c: i32;          // variable declared for a specific type
    let d: i32 = 20;     // variable declared for a specific type

    a = "Atul";

    println! ("{}", a);

    /*
    Notes:
    - if variable is declared but not used then prefix the name with _
    - cannot compile if a variable is declared and its type cannot be detected by compiler
    */

    // If-else

    println! ("sum of {} and {} is {}", b, d, add (b, d));

    if b < 10 {

        println! ("b is less than 10");
    } else {

        println! ("b is greater than or equal to 10")
    }

    // loop

    c = 1;

    loop {

        print! ("{} ", c);

        c += 1;

        if c == 6 {

            break;
        }
    }

    // while loop

    while c < 11 {

        print! ("{} ", c);
        c += 1;
    }

    // for loop

    for n in 11..=15 {

        print! ("{} ", n);
    }

    let arr = [16, 17, 18, 19, 20];

    for n in arr.iter () {

        print! ("{} ", n);
    }

    println! ();

    // printing for debugging, will print whole array is a formatted way
    println! ("{:?}", arr);

    // for loop on strings

    let str1 = "abcde";

    for ch in str1.chars () {

        print! ("{} ", ch);
    }

    let str2 = String::from ("fghij");

    for ch in str2.chars () {

        print! ("{} ", ch);
    }

    println! ();

    // string length

    let emoji = "🤦🏼‍♂️";

    println! ("length of {} is {}", emoji, emoji.len ());

}

fn advance_ds_vec ()
{
    // vectors
    let mut vec: Vec<i32> = Vec::<i32>::new ();
    /*
    // alternative way
    let mut vec: Vec<i32> = vec! ();
    */

    vec.push (10);
    vec.push (20);
    vec.push (30);
    vec.push (40);
    vec.push (50);
    vec.push (60);

    println! ("capacity of vector: {}", vec.capacity ());
    println! ("size of vector: {}", vec.len ());
    println! ("0th element: {}", vec[0]);

    println! ("vector after multiple push: {:?}", vec);

    vec.pop ();

    println! ("vector after pop: {:?}", vec);

    let oth: Vec<i32> = vec! (10, 20, 30, 40, 50);

    if vec == oth {

        println! ("both vectors are equal");
    } else {

        println! ("unequal vectors")
    }

    let result = vec.binary_search (&-10);

    println! ("{:?}", result);

    match result {

        Ok (idx) => println! ("{}", vec[idx]),

        Err (err) => println! ("error {}", err)
    }

    // returns ordering enum: less (-1), equal (0), greater (1)
    println! ("vec.cmp (&oth): {:?}", vec.cmp (&oth));

    println! ("first: {}, last: {}, contains (100): {}", vec.first ().unwrap (), vec.last ().unwrap (), vec.contains (&100));

    vec.rotate_left (2);
    println! ("left rotate (2): {:?}", vec);
    vec.rotate_right (3);
    println! ("right rotate (3): {:?}", vec);
    vec.reverse ();
    println! ("reverse: {:?}", vec);
    vec.sort ();
    println! ("sorted: {:?}", vec);
    vec.sort_by(|a, b| b.cmp (a));
    println! ("rev sorted: {:?}", vec);
    vec.swap (1, 3);
    println! ("swap idx 1 and 3: {:?}", vec);
    vec.insert (2, 60);
    println! ("inserted 60 at idx 2: {:?}", vec);
    vec.remove (2);
    println! ("removed element at idx 2: {:?}", vec);
    vec.clear ();
    println! ("cleared vector: {:?}", vec);

    println! ("done");
}

fn advance_ds_unordered_map ()
{
    let mut map = HashMap::new ();

    map.insert ("a", 1);
    map.insert ("b", 2);
    map.insert ("c", 3);
    map.insert ("d", 4);

    println! ("map: {:?}, len: {}, capacity: {}", map, map.len (), map.capacity ());

    map.remove ("d");
    println! ("removed key: \"d\": {:?}", map);

    if map.contains_key ("b") {

        println! ("key: \"b\" is present: {}", map["b"]);
    } else {

        println! ("key: \"b\" is not present")
    }

    let key = map.keys ();
    println! ("keys: {:?}", key);

    let val = map.values ();
    println! ("values: {:?}", val);

    let result = map.get ("a");

    match result {

        Some (val) => println! ("value for \"a\": {}", val),

        None => println! ("key \"a\" not found")
    }

    print! ("iterating on map:");

    for (key, val) in map.iter () {

        print! (" ({}, {})", key, val);
    }

    println! ();

    map.clear ();
    println! ("cleared map: {:?}", map);
}

fn main ()
{
    advance_ds_unordered_map ();
}
