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

use std::{env, error::Error, fmt, io};

#[tokio::main(basic_scheduler)]
//#[tokio::main(threaded_scheduler)]
async fn main() -> Result<(), Box<dyn Error>> {
    for i in 0..10 {
        println!("tokio::spawn start");
        tokio::spawn(async move {
            let dur = tokio::time::Duration::from_millis(5_000 + i * 1000);
            println!("process delay time: {:?} start", dur);
            let s = std::time::Instant::now();
            tokio::time::delay_until(
                tokio::time::Instant::now() + dur).await;
            println!("process delay stop time: {:?}", std::time::Instant::now() - s);
        });
        println!("tokio::spawn start exit");
    }

    // tokio::spawn(async move {
    //     let dur = std::time::Duration::from_millis(20_000);
    //     println!("process sleep delay time: {:?} start", dur);
    //     let s = std::time::Instant::now();
    //     std::thread::sleep(dur);
    //     println!("process sleep delay stop time: {:?}", std::time::Instant::now() - s);
    // });

    // let dur = std::time::Duration::from_millis(20_000);
    // println!("process sleep delay time: {:?} start", dur);
    // let s = std::time::Instant::now();
    // std::thread::sleep(dur);
    // println!("process sleep delay stop time: {:?}", std::time::Instant::now() - s);

    tokio::time::delay_until(tokio::time::Instant::now() + tokio::time::Duration::from_millis(15_000)).await;
    Ok(())
}
