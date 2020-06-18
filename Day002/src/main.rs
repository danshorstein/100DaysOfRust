fn main() {
  println!("{:?}", arrays());
}


fn arrays() -> [i32; 6] {
  let a = [1, 2, 3, 4, 5, 6];
  println!("{:?}", a);
  println!("First element is {}", a[0]);
  a
}


