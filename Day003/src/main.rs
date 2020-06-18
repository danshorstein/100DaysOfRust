fn main() {
    let f = 82.0;
    let c = f2c(f);

    println!("{} Farenheit is {} Celsius", f, c);
}

fn f2c(x: f32) -> f32 {
    (x - 32.0) * 5.0 / 9.0
}

