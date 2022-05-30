use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub(crate) struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    pub(crate) fn new(txt: &str) -> Self {
        Self {
            a: txt.into(),
            b: std::ptr::null(),
            _marker: PhantomPinned
        }
    }


    pub(crate) fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe {self.get_unchecked_mut()};
        this.b = self_ptr;
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
    let mut test_a = Test::new("test a");
    let mut test_a = unsafe { Pin::new_unchecked(&mut test_a) };
    Test::init(test_a.as_mut());

    let mut test_b = Test::new("test a");
    let mut test_b = unsafe { Pin::new_unchecked(&mut test_b) };
    Test::init(test_b.as_mut());

    println!("Test a -> a: {}, b: {}", Test::a(test_a.as_ref()), Test::b(test_a.as_ref()));
    // std::mem::swap(test_a.as_mut(), test_b.as_mut()); /*Pin<T> prevent to move value, if don't use Pin you can move values and has chance to make undefined behavior*/
    //Pin give a stable memory address
    println!("Test b -> a: {}, b: {}", Test::a(test_b.as_ref()), Test::b(test_b.as_ref()));
}
