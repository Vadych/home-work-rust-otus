pub trait Subscriber {
    fn on_add(&mut self, name: &str);
}

impl<F> Subscriber for F
where
    F: for<'a> FnMut(&'a str),
{
    fn on_add(&mut self, name: &str) {
        self(name);
    }
}
