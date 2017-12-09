fn main() {
    let mut x: i32 = 5;
    let mut y: bool = true;
    x = squar(x, y);
    y = false;
    x = squar(x, y);
}

fn squar(x: i32, y: bool) -> i32 {
    let return_daikon = x*x;
    return_daikon
}