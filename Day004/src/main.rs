fn main() {
    let st = "hello world".to_string();
    let st_slc = owning(&st);
    println!("{} to {}", st, st_slc);
}

fn owning(s: &String) -> &str {
    // takes a reference to a string, returns a slice
   &s[..3] 
}
