// edition:2018

#![feature(async_await)]

struct Xyz {
    a: u64,
}

trait Foo {}

impl Xyz {
    async fn do_sth<'a>(
        foo: &dyn Foo, bar: &'a dyn Foo //~ ERROR cannot infer
    ) -> &dyn Foo //~ ERROR missing lifetime specifier
    {
        foo
    }
}

fn main() {}
