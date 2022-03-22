#[derive(Debug)]
struct Square {
    length: i32, 
    width: i32, 
}

impl Square {
    fn new(length: i32, width: i32) -> Self {
        Self {
            length, 
            width, 
        }
    }

    fn area(&self) -> i32 {
        self.length * self.width
    }

    fn is_actual_square(&self) -> bool {
        self.length == self.width
    }
}

fn main() {
    let len: i32 = 5;
    let width: i32 = 7;

    println!("Defining a new square with length: {}, width: {}", len, width);

    let my_square = Square::new(len, width);

    println!("The area of my square is: {}", my_square.area());

    println!("Is my square actually a square? {}", my_square.is_actual_square());
}
