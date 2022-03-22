use std::fmt::Display;

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

    // not the best way to create instance of struct 
    let a_semver = SemVer{
        major: 1, 
        minor: 2, 
        // a way to implement default values for the type 
        patch: Default::default(),
    };

   println!("printing out the semver: {:?}", a_semver);
   println!("printing out the semver: {:#?}", a_semver); // in order to pretty print this

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