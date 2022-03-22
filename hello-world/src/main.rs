use std::fmt::Display;

// for simple value-only, no-functionality structs, exposing fields can be OK
pub struct Point {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug)]
struct SemVer {
    major: u16, // unsigned 16-bit integer
    minor: u16, 
    patch: u16,
}

// impl lock for a struct
impl SemVer{
    fn new(major: u16, minor:u16, patch: u16) -> Self {
        // function parameters are positional not named
        Self {
            major, 
            minor, 
            // a way to implement default values for the type 
            patch,
        }
    }

    fn info(&self) {
        println!("I am a SemVer! major number: {}", self.major)
    }
}

fn main() {
    println!("Hello, world!");

    let an_int: i32 = 123; // default type: signed 32-bit integer
    let a_big_int: i32 = 1_000_000; 
    let a_float: f64 = 3.14;  // default type: 64 bit float 
    let less_precise_float: f32 = 3.1416; // new type: 32-bit float 

    let a_string_slice: &str = "hello, string!"; // a string slice - a pointer and length 
    let owned_string: String = "hello, owned String!".to_string(); // owned string 

    println!("these are a few of my favorite things: {} {} {}", 
    a_big_int, less_precise_float, owned_string);

    let generous_pi = 4.0;
    say_hi_to_float(generous_pi); // note if this type doesn't match the function definition will not compile

    // not the best way to create instance of struct 
    let a_semver = SemVer{
        major: 1, 
        minor: 2, 
        // a way to implement default values for the type 
        patch: Default::default(),
    };

    // note that the println is a type of macro invocation 
   println!("printing out the semver: {:?}", a_semver);
   println!("printing out the semver: {:#?}", a_semver); // in order to pretty print this

   a_semver.info();

   let end_of_function: bool = true; 

   if end_of_function {
       println!("hurray!")
   }

   // this does pattern matching 
    match end_of_function {
        true => todo!(), // another macro that will panic if this doesn't happen
        false => todo!()
    }
}

// this is more natural 
// no & = no reference = you own that
struct ContainsString {
    member: String,
}

// This is more anti-pattern
// References more common only with complicated structures
// explicit about lifetime of the reference 
// a means the lifetime a 
struct UnergonomicWithLifetimes<'a> {
    member: &'a str,
    another_ref: &'a i32, 
}

fn say_hi_to_number(num: u8) {
    println!("hi! {}", num)
}

fn say_hi_to_float(f: f32) {
    println!("hi! {}", f)
}

fn say_hi_to_big_float(f: f64) {
    println!("hi! {}", f)
}