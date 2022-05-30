use std::{
    thread,
    time::{Duration, Instant},
};

use futures::{future, join, pin_mut, select, FutureExt};

//source: https://rust-lang.github.io/async-book/06_multiple_futures/03_select.html#default---and-complete--
#[derive(Debug)]
pub(crate) struct Book {
    name: String,
    release_year: u16,
}

#[derive(Debug)]
pub(crate) struct Music {
    name: String,
    singer: String,
}

pub(crate) async fn get_book() -> Book {
    thread::sleep(Duration::from_secs(3));
    Book {
        name: "Async Rust".into(),
        release_year: 2018,
    }
}

pub(crate) async fn get_music() -> Music {
    thread::sleep(Duration::from_secs(3));
    Music {
        name: "Face the end".into(),
        singer: "Andre Matos".into(),
    }
}

pub(crate) async fn get_music_and_book() -> (Music, Book) {
    let book = get_book();
    let music = get_music();
    join!(music, book) //join don't run in parallel, its run concurrently
}

pub(crate) async fn race_tasks() {
    let t1 = get_book().fuse();
    let t2 = get_music().fuse();

    pin_mut!(t1, t2);

    select! { //run in parallel,
        book = t1 => println!("task 1 finish first {:#?}", book),
        music = t2 => println!("task 2 finish first {:#?}", music),
    } //after finish the first task (task 1 or task 2), drop the remain tasks
}

async fn count() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break, //complete branches can be used to handle the case where all futures being selected over have completed and will no longer make progress
            default => unreachable!(), // never runs (futures are ready, then complete)
        };
    }
    dbg!(total);
    assert_eq!(total, 10);
}

fn main() {
    let now = Instant::now();
    let (music, book) = futures::executor::block_on(get_music_and_book());
    dbg!(music);
    dbg!(book);
    let finish = Instant::now().duration_since(now);
    println!("{}", finish.as_millis());

    let now = Instant::now();
    futures::executor::block_on(race_tasks());
    let finish = Instant::now().duration_since(now);
    println!("{}", finish.as_millis());
    thread::sleep(Duration::from_millis(500)); //make sleep for check if another task run in 'race_tasks' function
    futures::executor::block_on(count());
}
