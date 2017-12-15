fn main() {
    eprintln!("input-language Rust\ndecl-version 2.0\nvar-comparability implicit\n");
    let mut x: i32 = 5;
    let mut y: bool = true;
    x = squar(x, y);
    y = false;
    x = squar(x, y);
}

fn squar(x: i32, y: bool) -> i32 {
    eprintln!("..square(int,\\_bool):::ENTER\nx\n{}\n1\n\ny\n{}\n1\n", x ,y);
    let return_daikon_unique = x*x;
    eprintln!("..square(int,\\_bool):::EXIT0\nx\n{}\n1\n\ny\n{}\n1\nreturn\n{}\n1\n", x, y, return_daikon_unique);
    return_daikon_unique
}