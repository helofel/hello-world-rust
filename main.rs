
#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]

// Struct Declaration
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

struct ColorT(u8, u8, u8);
struct Kilometers(i32);

struct Electron;

struct Point<T> {
    x: T,
    y: T,
}

/* my own structs to fux bugs */
struct Person2 {

}

/*----------------*/

// 02 - - - - - - - - - - - - - - - - - - - - - -
struct Task {
    title: String,
    assignee: Option<Person2>,
}

// 💭 Instead of assignee: Person, we use Option<Person>
// because the task has not been assigned to a specific person

// - - - - - - - - - - - - - - - - - - - - - - -
// When using Option types as return types on functions
// we can use pattern matching to catch the relevant return type(Some/None) when calling them


struct Player {
    first_name: String,
    last_name: String,
}

impl Player {
    fn full_name_one(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait FullName {
    fn full_name(&self) -> String;
}

impl FullName for Player {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait Foo {
    fn bar(&self);
    fn baz(&self) { println!("We called baz."); }
}

impl Player {
    fn new(first_name: String, last_name: String) -> Player {
        Player {
            first_name : first_name,
            last_name : last_name,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait From<T> {
    //fn from(T) -> Self;
}
/*  impl From<u8> for u16 {
        //...
    }
    impl From<u8> for u32{
        //...
    } */ 

// Should specify after the trait name like generic functions

trait Person {
    fn full_name(&self) -> String;
}

    trait Employee : Person { // Employee inherits from person trait
      fn job_title(&self) -> String;
    }

    trait Expat{

    }

    trait ExpatEmployee : Employee + Expat { // ExpatEmployee inherits from Employee and Expat traits
      fn additional_tax(&self) -> f64;
    }

    trait GetSound {
        fn get_sound(&self) -> String;
    }
    
    struct Cat {
        sound: String,
    }
        impl GetSound for Cat {
            fn get_sound(&self) -> String {
                self.sound.clone()
            }
        }
    
    struct Bell {
        sound: String,
    }
        impl GetSound for Bell {
            fn get_sound(&self) -> String {
                self.sound.clone()
            }
        }
    
    
    fn make_sound<T: GetSound>(t: &T) {
        println!("{}!", t.get_sound())
    }

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

// The `Day` is the enum
// Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday are the variants

enum FlashMessage {
    Success, // A unit variant
    Warning{ category: i32, message: String }, // A struct variant
    Error(String) // A tuple variant
}

fn print_sum(a: i8, b: i8) {
    println!("sum is: {}", a + b);
}

// 01. Without the return keyword. Only the last expression returns.
fn plus_one(a: i32) -> i32 {
    a + 1
    // There is no ending ; in the above line.
    // It means this is an expression which equals to `return a + 1;`.
}

// 02. With the return keyword.
fn plus_two(a: i32) -> i32 {
    return a + 2;
    // Should use return keyword only on conditional/ early returns.
    // Using return keyword in the last expression is a bad practice.
}

fn get_midnightblue_color() -> Color {
    Color {red: 25, green: 25, blue: 112}
}

fn print_flash_message(m : FlashMessage) {
    // Pattern matching with enum
    match m {
      FlashMessage::Success =>
        println!("Form Submitted correctly"),
      FlashMessage::Warning {category, message} => // Destructure, should use same field names
        println!("Warning : {} - {}", category, message),
      FlashMessage::Error(msg) =>
        println!("Error : {}", msg)
    }
}

fn takes_anything<T>(x: T) { // x has type T, T is a generic type
}

fn takes_two_of_the_same_things<T>(x: T, y: T) { // Both x and y has the same type
}

fn takes_two_things<T, U>(x: T, y: U) { // Multiple types
}

/*
// 01 - - - - - - - - - - - - - - - - - - - - - -
fn get_id_by_username(username: &str) -> Option<usize> {
    // if username can be found in the system, set userId
        //return Some(someId);
    // else
        None
}

// 💭 So, on the above function, instead of setting return type as usize
//   set return type as Option<usize>
// Instead of return userId, return Some(userId)
//   else None (💡remember? last return statement no need return keyword and ending ;)

// - - - - - - - - - - - - - - - - - - - - - -
fn get_word_count_from_file(file_name: &str) -> Result<u32, &str> {
    // if the file is not found on the system, return error
      //return Err("File can not be found!");
    // else, count and return the word count
      // let mut word_count: u32; ....
      //Ok(word_count)
      None
}
*/

  // 💭 On the above function,
  // instead panic(break) the app, when the file can not be found; return Err(something)
  // or when it could get the relevant data; return Ok(data)
  
  
  // - - - - - - - - - - - - - - - - - - - - - - -
  // We can use pattern matching to catch the relevant return type(Ok/Err) when calling it


fn main() {
    println!("Hello, world!");
    println!("{0} {1}!", "Hello,", "world");
    println!("{greeting} {name}!", greeting = "Hello,", name = "world");

    println!("{:?}", [1,2,3,4,5]);
    println!("{:#?}", [1,2,3,4,5]);

    let x = format!("{}, {}!", "Hello", "world");

    println!("{}", x);

    print!("Hello, world!");
    println!();

    print!("Hello, world\n");

    /*

    // Line comments
    /* Block comments */

    /// Line comments; document the next item
    /** Block comments; document the next item */
    
    //! Line comments; document the enclosing item
    /*! Block comments; document the enclosing item !*/

    /// Outer comment
    #[doc = "Outer comment"]

    //! Inner comment
    #![doc = "Inner comment"]


    */
     
    let a; // Declaration; without data type
    a = 5; // Assignment
    
    let b: i8; // Declaration; with data type 
    b = 5;
    
    let t = true;        // Declaration + assignment; without data type
    let f: bool = false; // Declaration + assignment; with data type
    
    let (x, y) = (1, 2); // x = 1 and y = 2
    
    let mut z = 5;
    z = 6;
    
    let z = { x + y }; // z = 3
    
    let z = {
        let x = 1;
        let y = 2;
    
        x + y
    }; // z = 3

    const N: i32 = 5;

    //static N: i32 = 5;

    let x: f64 = -20.48; // float
    let x: i64 = x.floor() as i64; // int
    println!("{}", x); // -21

    let s: &str = "hello"; // &str
    let s: String = s.to_uppercase(); // String
    println!("{}", s); // HELLO

    let x = 2;
    let square = |i: i32| -> i32 { // Input parameters are passed inside | | and expression body is wrapped within { }
        i * i
    };
    println!("{}", square(x));

    let x = 2;
    let square = |i| i * i; // { } are optional for single-lined closures
    println!("{}", square(x));

    let x = 2;
    let x_square = |i: i32| -> i32 { i * i }(x); // { } are mandatory while creating and calling same time.
    println!("{}", x_square);

    let x = 2;
    let x_square = |i| -> i32 { i * i }(x); // ⭐️ The return type is mandatory.
    println!("{}", x_square);

    let x = true;
    let y: bool = false;

    // ⭐️ no TRUE, FALSE, 1, 0


    let x = 'x';
    let y: char = '😎';

    // ⭐️ no "x", only single quotes

    let x = 10; // ⭐️ The default integer type in Rust is i32
    let y: i8 = -128;

    let x = 1.5; // ⭐️ The default float type in Rust is f64
    let y: f64 = 2.0;

    //ARRAY
    let a = [1, 2, 3];
    let a: [i32; 3] = [1, 2, 3]; // [Type; NO of elements]

    let b: [i32; 0] = []; // An empty array

    let mut c: [i32; 3] = [1, 2, 3];
    c[0] = 2;
    c[1] = 4;
    c[2] = 6;

    println!("{:?}", c); // [2, 4, 6]
    println!("{:#?}", c);
    //  [
    //      2,
    //      4,
    //      6,
    //  ]

    let d = [0; 5];   // [0, 0, 0, 0, 0]
    let e = ["x"; 5]; // ["x", "x", "x", "x", "x"]

    //TUPLE
    let a = (1, 1.5, true, 'a');
    let a: (i32, f64, bool, char) = (1, 1.5, true, 'a');
    
    let mut b = (1, 1.5);
    b.0 = 2;
    b.1 = 3.0;
    
    println!("{:?}", b); // (2, 3.0)
    println!("{:#?}", b);
    // (
    //   2,
    //   3.0,
    // )
    
    let (c, d) = b; // c = 2, d = 3.0
    let (e, _, _, f) = a; // e = 1, f = 'a'
    
    let g = (0,); // single-element tuple
    let h = (b, (2, 4), 5); // ((2, 3.0), (2, 4), 5)


    //SLICE
    let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array

    let b: &[i32] = &a; // Slicing whole array
    let c = &a[0..4]; // From 0th position to 4th(excluding)
    println!("{:#?}", c);
    let d = &a[..]; // Slicing whole array
    println!("{:#?}", d);
    
    let e = &a[1..3]; // [2, 3]
    println!("{:#?}", e);
    let f = &a[1..]; // [2, 3, 4]
    println!("{:#?}", f);
    let g = &a[..3]; // [1, 2, 3]
    println!("{:#?}", g);

    //STR
    let a = "Hello, world."; // a: &'static str
    let b: &str = "こんにちは, 世界!";
    println!("{}", b);

    //FUNCTION
    let p1: fn(i32) -> i32 = plus_one;
    let x = p1(5); // 6

    //TWO's COMPLIMENT CONVERSION
    let a = !-2; //1
    let b = !-1; //0
    let c = !0; //-1
    let d = !1; //-2

    //TYPECASTING
    let a = 15;
    let b = (a as f64) / 2.0; //7.5

    //STRING CONCATINATION
    let (s1, s2) = ("some", "thing"); // both &str
    // All bellow codes return `String`; something
    
    let s = String::from(s1) + s2; // String + &str
    
    let mut s = String::from(s1); // String
    s.push_str(s2); // + &str
    
    let s = format!("{}{}", s1, s2); // &str/String + &str/String
    
    let s = [s1, s2].concat(); // &str or String array

    //CONTROL FLOW
    // iii. Let's refactor further
    let team_size = 7;
    let team_size = if team_size < 5 {
        "Small" // ⭐️ no ;
    } else if team_size < 10 {
        "Medium"
    } else {
        "Large"
    };

    println!("Current team size : {}", team_size); // Current team size : Medium

    //MATCH
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S", // check 16
        17 | 18 => "M", // check 17 and 18
        19 ..= 21 => "L", // check from 19 to 21 (19,20,21)
        22 => "XL",
        _ => "Not Available",
    };
    
    println!("{}", tshirt_size); // L

    let is_allowed = false;
    let list_type = match is_allowed {
        true => "Full",
        false => "Restricted"
        // no default/ _ condition can be skipped
        // Because data type of is_allowed is boolean and all possibilities checked on conditions
    };
    
    println!("{}", list_type); // Restricted

    let marks_paper_a: u8 = 25;
    let marks_paper_b: u8 = 30;
    
    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard"
    };
    
    println!("{}", output); // Work hard

    //LOOP
    loop {
        println!("Loop forever!");
        break;
    }

    // Usage of break and continue
    let mut a = 0;

    loop {
        if a == 0 {
            println!("Skip Value : {}", a);
            a += 1;
            continue;
        } else if a == 2 {
            println!("Break At : {}", a);
            break;
        }

        println!("Current Value : {}", a);
        a += 1;
    }

    // Outer break
    let mut b1 = 1;

    'outer_loop: loop { //set label outer_loop
      let mut b2 = 1;

      'inner_loop: loop {
        println!("Current Value : [{}][{}]", b1, b2);

        if b1 == 2 && b2 == 2 {
            break 'outer_loop; // kill outer_loop
        } else if b2 == 5 {
            break;
        }

        b2 += 1;
      }

      b1 += 1;
    }

    //WHILE
    let mut a = 1;

    while a <= 10 {
        println!("Current value : {}", a);
        a += 1; //no ++ or -- on Rust
    }

    // Usage of break and continue
    let mut b = 0;

    while b < 5 {
        if b == 0 {
            println!("Skip value : {}", b);
            b += 1;
            continue;
        } else if b == 2 {
            println!("Break At : {}", b);
            break;
        }

        println!("Current value : {}", b);
        b += 1;
    }

    // Outer break
    let mut c1 = 1;

    'outer_while: while c1 < 6 { //set label outer_while
        let mut c2 = 1;

        'inner_while: while c2 < 6 {
            println!("Current Value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
            c2 += 1;
        }

        c1 += 1;
    }

    //FOR
    // 0 to 10 (10 exclusive); In other languages, `for(i = 0; i < 10; i++)`
    for i in 0..10 {
        println!("Current value : {}", i);
    }

    // 1 to 10 (10 inclusive); In other languages, `for(i = 1; i <= 10; i++)`
    for i in 1..=10 {
        println!("Current value : {}", i);
    }

    // Usage of break and continue
    for b in 0..6 {
        if b == 0 {
          println!("Skip Value : {}", b);
          continue;
        } else if b == 2 {
          println!("Break At : {}", b);
          break;
        }
        
       println!("Current value : {}", b);
    }

    // Outer break
    'outer_for: for c1 in 1..6 { //set label outer_for

        'inner_for: for c2 in 1..6 {
          println!("Current Value : [{}][{}]", c1, c2);
          if c1 == 2 && c2 == 2 { break 'outer_for; } //kill outer_for
        }
  
    }

    // Working with arrays/vectors
    let group : [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];

    for n in 0..group.len() { // group.len() = 4 -> 0..4 👎 check group.len()on each iteration
      println!("Current Person : {}", group[n]);
    }

    for person in group.iter() { // 👍 group.iter() turn the array into a simple iterator
      println!("Current Person : {}", person);
    }

    //VECTORS
    //let mut a = Vec::new(); //1.With new() keyword
    //let mut b = vec![]; //2.Using the vec! macro

    let mut a2: Vec<i32> = Vec::new();
    let mut b2: Vec<i32> = vec![];
    let mut b3 = vec![1i32, 2, 3];//Sufixing 1st value with data type
    
    let mut b4 = vec![1, 2, 3];
    let mut b5: Vec<i32> = vec![1, 2, 3];
    let mut b6  = vec![1i32, 2, 3];
    let mut b7 = vec![0; 10]; //Ten zeroes

    //Accessing and changing existing data
    let mut c = vec![5, 4, 3, 2, 1];
    c[0] = 1;
    c[1] = 2;
    //c[6] = 2; Cannot assign values this way, index out of bounds
    println!("{:?}", c); //[1, 2, 3, 2, 1]
    
    //push and pop
    let mut d: Vec<i32> = Vec::new();
    d.push(1); //[1] : Add an element to the end
    d.push(2); //[1, 2]
    d.pop(); //[1] : : Remove an element from the end
    
    
    // 🔎 Capacity and reallocation
    let mut e: Vec<i32> = Vec::with_capacity(10);
    println!("Length: {}, Capacity : {}", e.len(), e.capacity()); //Length: 0, Capacity : 10
    
    // These are all done without reallocating...
    for i in 0..10 {
        e.push(i);
    }
    // ...but this may make the vector reallocate
    e.push(11);

    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("A reference to {}", i);
    }
    
    for i in &mut v {
        println!("A mutable reference to {}", i);
    }
    
    for i in v {
        println!("Take ownership of the vector and its element {}", i);
    }

    //STRUCTS
    // Creating an instance
    let black = Color {red: 0, green: 0, blue: 0};

    // Accessing its fields using dot notation
    println!("Black = rgb({}, {}, {})", black.red, black.green, black.blue); //Black = rgb(0, 0, 0)

    // Structs are immutable by default, use `mut` to make it mutable but doesn't support field level mutability
    let mut link_color = Color {red: 0,green: 0,blue: 255};
    link_color.blue = 238;
    println!("Link Color = rgb({}, {}, {})", link_color.red, link_color.green, link_color.blue); //Link Color = rgb(0, 0, 238)

    // Copy elements from another instance
    let blue = Color {blue: 255, .. link_color};
    println!("Blue = rgb({}, {}, {})", blue.red, blue.green, blue.blue); //Blue = rgb(0, 0, 255)

    // Destructure the instance using a `let` binding, this will not destruct blue instance
    let Color {red: r, green: g, blue: b} = blue;
    println!("Blue = rgb({}, {}, {})", r, g, b); //Blue = rgb(0, 0, 255)

    // Creating an instance via functions & accessing its fields
    let midnightblue = get_midnightblue_color();
    println!("Midnight Blue = rgb({}, {}, {})", midnightblue.red, midnightblue.green, midnightblue.blue); //Midnight Blue = rgb(25, 25, 112)

    // Destructure the instance using a `let` binding
    let Color {red: r, green: g, blue: b} = get_midnightblue_color();
    println!("Midnight Blue = rgb({}, {}, {})", r, g, b); //Midnight Blue = rgb(25, 25, 112)

    //TUPLE STRUCTS
    // Creating an instance
    let black = ColorT(0, 0, 0);

    // Destructure the instance using a `let` binding, this will not destruct black instance
    let ColorT(r, g, b) = black;
    println!("Black = rgb({}, {}, {})", r, g, b); //black = rgb(0, 0, 0);

    // Newtype pattern
    let distance = Kilometers(20);
    // Destructure the instance using a `let` binding
    let Kilometers(distance_in_km) = distance;
    println!("The distance: {} km", distance_in_km); //The distance: 20 km

    //UNIT STRUCTS
    let x = Electron;

    //ENUMS
    let mut form_status = FlashMessage::Success;
    print_flash_message(form_status);
  
    form_status = FlashMessage::Warning {category: 2, message: String::from("Field X is required")};
    print_flash_message(form_status);
  
    form_status = FlashMessage::Error(String::from("Connection Error"));
    print_flash_message(form_status);

    //GENERICS
    let point_a = Point { x: 0, y: 0 }; // T is a int type
    let point_b = Point { x: 0.0, y: 0.0 }; // T is a float type

    /*
    // 🔎 When adding an implementation for a generic struct, the type parameters should be declared after the impl as well
    //   impl<T> Point<T> {
    let username = "anonymous";
    match get_id_by_username(username) {
        //None => println!("User not found"),
        //Some(i) => println!("User Id: {}", i)
    }

    let mut file_name = "file_a";
    match get_word_count_from_file(file_name) {
        //Ok(i) => println!("Word Count: {}", i),
        //Err(e) => println!("Error: {}", e)
    }
    */

    //IMPS & TRAITS
    let player_1 = Player {
        first_name: "Rafael".to_string(),
        last_name: "Nadal".to_string(),
    };

    println!("Player 01: {}", player_1.full_name());
    // ⭐️ Implementation must appear in the same crate as the self type

    // 💡 And also in Rust, new traits can be implemented for existing types even for types like i8, f64 and etc.
    // Same way existing traits can be implemented for new types you are creating.
    // But we can not implement existing traits into existing types.

    let player_2 = Player {
        first_name: "Roger".to_string(),
        last_name: "Federer".to_string(),
    };

    println!("Player 02: {}", player_2.full_name());
    // 🔎 Other than functions, traits can contain constants and types.

    let player_name = Player::new("Serena".to_string(), "Williams".to_string()).full_name();
    println!("Player: {}", player_name);

    // We have used :: notation for `new()` and . notation for `full_name()`

    // 🔎 Also in here, instead of using new() and full_name() separately as two expressions, 
    // we can use Method Chaining. ex. `player.add_points(2).get_point_count();`

    let kitty = Cat { sound: "Meow".to_string() };
    let the_bell = Bell { sound: "Ding Dong".to_string() };

    make_sound(&kitty); // Meow!
    make_sound(&the_bell); // Ding Dong!

}
