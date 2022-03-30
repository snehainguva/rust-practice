use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    fn new(major: u16, minor: u16, patch: u16) -> SemVer {
        SemVer {
            major,
            minor,
            patch,
        }
    }

    fn new_short(major: u16) -> SemVer {
        Self::new(major, 0, 0)
    }

    fn info(&self) {
        println!(
            "hi, I'm a semver: {}.{}.{}",
            self.major, self.minor, self.patch
        )
    }
}

// the use of ? operator 
// Result enum is the standard return type  
fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<SemVer, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

// being explicit with error handling 
// trying to bubble up errors by boxing them - this is what I'm having trouble with 
fn read_user_from_file_explicitly<P: AsRef<Path>>(path: P) -> Result<SemVer,  Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let f = File::open(path);

    let f = match f {
        Ok(file) => file, 
        // need to return the boxed error inside of a new error 
        Err(error) => return Err(Box::new(error)),
    };

    let reader = BufReader::new(f);

    // Read the JSON contents of the file as an instance of `User`.
    let result: Result<SemVer, _> = serde_json::from_reader(reader);
     match result {
        Ok(semver) => Ok(semver),  
        // need to return the box inside of a result 
        Err(err) =>  return Err(Box::new(err)),
     }
}


fn main() {
    //Method #2: another way of reading in json file with serde
    let u = read_user_from_file("releases.txt").unwrap();
    println!("deserialized json: {:#?}", u); 
}
