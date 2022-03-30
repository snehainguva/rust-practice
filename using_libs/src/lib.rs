pub fn addition(x: i8, y: i8) -> i8 {
    x + y 
}

pub fn subtraction(x: i8, y: i8) -> i8 {
    x - y 
}

#[cfg(test)]
mod tests {
    #[test]
    fn testing_addition() {
        let result = 3 + 4;
        assert_eq!(super::addition(3,4), result);
    }
    fn testing_subtraction() {
        let result = -1;
        assert_eq!(super::subtraction(3,4), result);
    }
}