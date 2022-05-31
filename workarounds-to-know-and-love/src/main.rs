use std::rc::Rc;

use futures::{future::{FutureExt, BoxFuture}, executor};


#[derive(Default)]
struct NotSend(Rc<()>);


async fn foo() {
    NotSend::default(); //if set the NotSend to a variable cannot compile, because Rc does not implement the Send trait
                        //but we can do this
                        //{
                        //    let x = NotSend::default();
                        //} //x drop  after this
    bar().await;    
}

async fn bar() {}

fn require_send(_: impl Send) {

}

fn recursion() -> BoxFuture<'static, ()> { //Recursion function cannot called directly
                                            //because of this return a BoxFuture and use async block
    async move {
        recursion().await;
        recursion().await;
    }.boxed()
}

fn main() {
    let value = async {

        Ok::<(), String>(()) //this type is necessary, if remove type get a compile error
    };
    require_send(foo());
    //futures::executor::block_on(recursion()); //stack overflow
}

/*
    async cannot be used in traits
*/
