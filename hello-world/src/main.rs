use std::{fmt::Display};

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

// This is similar to the idea of using iota to define types in Golang
// Except we are using a separate type called enumerated types. 
enum CrateKind {
    Binary, 
    Library, 
    Other {description: String },
}

struct Crate {
    version: SemVer, 
    kind: CrateKind, // Rust string always valid utf-8
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
    // match end_of_function {
    //     true => todo!(), // another macro that will panic if this doesn't happen
    //     false => todo!()
    // }

    let crt = Crate { 
        version: SemVer::new(1, 0, 0), 
        kind: CrateKind::Binary,
    };

    let new_Crt = Crate {
        version: SemVer::new(1, 0, 0),
        kind: CrateKind::Other {
            description: "cat".to_string(),
        },
    };

    match crt.kind {
        CrateKind::Binary => println!("found a binary!"),
        CrateKind::Library => println!("found a library"),
        CrateKind::Other { description }=> {
            if description == "alien" {
                println!("wasn't expecting you here!");
            } else {
                println!("found nothing else: {}", description);
            }
        }
    }

    match new_Crt.kind {
        CrateKind::Binary => println!("found a binary!"),
        CrateKind::Library => println!("found a library"),
        CrateKind::Other { description } => {
            if description == "cat" {
                println!("omg we found gatsby!");
            }
        }
    }

    // Can do number matching as well 
    let num: u8 = 5; // Note this is the same as saying let num = 5_u8; 
    
    match num {
        0..=4 => println!("this is a smaller number"),
        5..=8 => println!("a bit larger of a number"), // note .. is for a range
        9..=u8::MAX => println!("found the largest possible number"),
    };

    // Can also pattern match on a struct 
    match a_semver {
        SemVer {
            major,
            .. // ignore what we don't want
        }  if major >= 1 => { 
            println!("yay we are at least at major version 1");
        }

        _ => ()
    };

    let crt = Crate {
        version: SemVer::new(1, 0, 0),
        kind: CrateKind::Other {
            description: "something else!".to_string(),
        },
    };

    match crt {
        Crate {
            version: SemVer { major, .. },
            ..
        } if major == 0 => println!("deploy it ANYWAY"),
        _ => (),
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