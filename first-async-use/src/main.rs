use futures::executor::block_on;

async fn hello_world() {
    println!("Hello, World!");
}

async fn print() {
    println!("async test");
}

async fn rever_print() {
    println!("!dlroW ,olleH");
}

async fn call_hello_await() {
    hello_world().await; //await block the thread until completition
}

async fn call_asyncs() {
    futures::join!(print(), rever_print()); /*
                                                the macro join! is similar to .await, but await the Futures
                                                concurrently, if the first Futures temporarily blocked, the `second`
                                                future will take over the current thread,
                                                if all Futures are blocked, then function will blockd until the end of blocks
                                            */
}

fn main() {
    let future = call_hello_await(); //nothing happen
    block_on(future);
    block_on(call_asyncs());
}
