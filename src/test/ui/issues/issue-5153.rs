trait Foo {
    fn foo(self: Box<Self>);
}

impl Foo for isize {
    fn foo(self: Box<isize>) { }
}

fn main() {
    (&5isize as &Foo).foo();
    //~^ ERROR: no method named `foo` found for type `&dyn Foo` in the current scope
}
