///niente di utile
fn func() {
    println!("hello, world!🦀");
}

fn main() {
    const Y: f32 = 0.2;
    let x: f32 = 0.1;
    let a: f32 = x + Y;
    let b = i32::MAX;
    let c = false;
    func();
    println!("{a}");
    if b >= 1 {
        println!("{b}");
    }
    let var = if c { 5 } else { 10 };
    println!("var: {}", var);
}
