use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub(crate) struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    pub(crate) fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Self {
            a: txt.into(),
            b: std::ptr::null(),
            _marker: PhantomPinned
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }



    pub(crate) fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub(crate) fn b(self: Pin<&Self>) -> &str {
        assert!(!self.b.is_null(), "Test::b called without Test::init being called first");
        unsafe { &(*self.b) }
    }
}

fn main() {
    let test_a = Test::new("test a");
    let test_b = Test::new("test a");

    println!("Test a -> a: {}, b: {}",test_a.as_ref().a(), test_a.as_ref().b());
    println!("Test b -> a: {}, b: {}",test_b.as_ref().a(), test_b.as_ref().b());
}
