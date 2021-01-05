use async_std::task;
use futures::join;
use std::time::{Instant};
use rand::Rng;

fn main() {
    let start_time = Instant::now();

    demo_waiting_for_two_async_fns();

    println!("Program finished in {} ms", start_time.elapsed().as_millis());
}

fn demo_waiting_for_two_async_fns() {
    // block_on takes a future and waits for it to complete.
    // Notice that this fn is not `async`, and we are not using
    // an async block either (because we are not calling `await`).
    task::block_on(call_both_sleepers());
}

async fn call_both_sleepers() {
    join!(first_sleeper(), second_sleeper());
}

async fn first_sleeper() {
    for i in 1..50000 {
        let num1 = rand::thread_rng().gen_range(1, 10000);
        let num2 = rand::thread_rng().gen_range(1, 10000);
        if i%2==0 {
            let ans = num1 + num2;
            println!("{}", ans);
        } else {
            let ans = num1 - num2;
            println!("{}", ans);
        }
    }
}

async fn second_sleeper() {
    for i in 1..50000 {
        let num1 = rand::thread_rng().gen_range(1, 10000);
        let num2 = rand::thread_rng().gen_range(1, 10000);
        if i%2==0 {
            let ans = num1 + num2;
            println!("{}", ans);
        } else {
            let ans = num1 - num2;
            println!("{}", ans);
        }
    }
}
