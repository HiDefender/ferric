#[derive(PartialEq, Eq)]
enum Foo {
    A,
    B(usize),
}

fn bar(foo: &Foo, baz: bool) {
    if foo == &Foo::B(3) || baz {
        println!("Do stuff")
    }
    // if let &Foo::B(_) = foo || baz {

    // }
}

fn foo(s: &str) {
    let s = s.trim();
}

fn foo1(mut s: &str) {
    s = s.trim();
}