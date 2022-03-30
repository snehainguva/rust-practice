use using_libs::{
    subtraction,
    addition,
};

#[test]
fn test_add() {
    let x = 3;
    let y = 4; 
    let result = 6;
    assert_eq!(addition(addition(x,y),subtraction(x,y)), result)
}