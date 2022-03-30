pub fn addition(x: u8, y: u8) -> u8 {
    x + y 
}


#[cfg(test)]
mod tests {
    #[test]
    fn testing_addition() {
        let result = 3 + 4;
        assert_eq!(super::addition(3,4), 8);
    }
}