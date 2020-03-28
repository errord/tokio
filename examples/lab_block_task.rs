//! A "tiny" example of HTTP request/response handling using transports.
//!
//! This example is intended for *learning purposes* to see how various pieces
//! hook up together and how HTTP can get up and running. Note that this example
//! is written with the restriction that it *can't* use any "big" library other
//! than Tokio, if you'd like a "real world" HTTP library you likely want a
//! crate like Hyper.
//!
//! Code here is based on the `echo-threads` example and implements two paths,
//! the `/plaintext` and `/json` routes to respond with some text and json,
//! respectively. By default this will run I/O on all the cores your system has
//! available, and it doesn't support HTTP request bodies.

#![warn(rust_2018_idioms)]
#![feature(rustc_private)]


#[macro_use]
extern crate log;

use std::{error::Error};

#[tokio::main(basic_scheduler)]
//#[tokio::main(threaded_scheduler)]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    block_task_lab().await
}

async fn block_task_lab() -> Result<(), Box<dyn Error>> {
    println!("block_task_lab entry");

    tokio::spawn(async move {
        println!("test task inner yield_now start");
        for i in 0..100 {
            println!("yield_now start {}", i);
            tokio::task::yield_now().await;
            println!("yield_now end {}", i);
        }
        println!("test task inner yield_now exit");
    });

    // // e: block scheduler loop
    // tokio::spawn(async move {
    //     println!("tokio::task sleep start");
    //     std::thread::sleep(std::time::Duration::from_millis(5_000));
    //     println!("tokio::task sleep end");
    // });

    // e: no block scheduler loop
    tokio::task::spawn_blocking(move || {
        println!("tokio::block_task sleep start");
        std::thread::sleep(std::time::Duration::from_millis(5_000));
        println!("tokio::block_task sleep end");
    }).await?; // e: wait spawn_blocking done

    // // e: panicked at 'can call blocking only when running in a spawned task'
    // tokio::task::block_in_place(move || {
    //     println!("block_in_place sleep start");
    //     std::thread::sleep(std::time::Duration::from_millis(5_000));
    //     println!("block_in_place sleep end");
    // });

    println!("block_task_lab exit");
    Ok(())
}
