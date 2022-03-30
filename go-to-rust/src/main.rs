#[derive(Debug)]
struct DetailedError {
    x: u8,
    y: u8,
}

/*
Go:

func foo(answer, x, y int) (int, error) {
    if answer == 42 {
        return 100, nil  //!! cannot use nil as type DetailedError in return argument
    }
    return 0, DetailedError{x: x, y: y}
}
*/

// In Rust we only have to return the relevant, not carrying around
// that nil/zero type 
fn foo(answer: u8) -> Result<u8, DetailedError> {
    if answer == 42 {
        Ok(100)
    } else {
        Err(DetailedError { x: 10, y: 20 })
    }
}

// Option allows us to return an optional type 
fn maybe_foo(answer: u8) -> Option<u8> {
    if answer == 42 {
        Some(100)
    } else {
        None
    }
}

fn main() {
    println!("first attempt: {:?}", foo(10));
    println!("first attempt: {:?}", foo(42));

    println!("now with Option:");
    println!("first attempt: {:?}", maybe_foo(10));
    println!("first attempt: {:?}", maybe_foo(42));
}
