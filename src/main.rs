///niente di utile
fn func() {
    let mut d = 0;
    loop {
        println!("hello, world!🦀");
        d = d + 1;
        if d >= 5 {
            break;
        }
    }
}

///ancora peggio
fn main() {
    const Y: f32 = 0.2;
    let x: f32 = 0.1;
    let a: f32 = x + Y;
    let b = i32::MAX;
    let c = false;
    println!("{a}");
    if b >= 1 {
        println!("{b}");
    }
    let var = if c { 5 } else { 10 };
    println!("var: {}", var);
    func();
}
