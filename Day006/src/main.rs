fn main() {
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a + b), 18);    
    println!("{:?}", v);
 
    v[0] = 12;

    println!("{:?}", v);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 1260);
}

