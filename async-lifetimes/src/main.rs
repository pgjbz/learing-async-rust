use futures::Future;

// source: https://rust-lang.github.io/async-book/03_async_await/01_chapter.html#async-lifetimes

// fn bad() -> impl Future<Output = u8> {
//     let x: u8 = 5;
//     borrow_x(&x) // ERROR: `x` does not live long enough
// }

// workaround
fn good() -> impl Future<Output = u8> {
    async {
        let x: u8 = 5;
        borrow_x(&x).await
    }
}


fn borrow_x<'a>(x: &'a u8) -> impl Future<Output = u8> + 'a {
    async move {
        *x
    }
}

async fn blocks() {
    let my_string = "foo".to_string();

    let future_one = async {
        println!("{my_string}");
    };

    let future_two = async {
        println!("{my_string}");
    };

    // Run both futures to completion, printing "foo" twice:
    let ((), ()) = futures::join!(future_one, future_two);
}

fn move_block() -> impl Future<Output = ()> {
    let my_string = "foo".to_string();
    async move {
        // like a closure 'move' keyword move a value to inside a block, and the value
        // cannot called outside block anymore
        println!("{my_string}");
    }
}

fn main() {
    futures::executor::block_on(async {
        futures::join!(blocks(), move_block());
    });
}
