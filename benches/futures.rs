#![feature(test)]

extern crate test;

use std::sync::Arc;

use async_std::task;
use futures::lock::Mutex;
use test::Bencher;

#[bench]
fn create(b: &mut Bencher) {
    b.iter(|| Mutex::new(()));
}

#[bench]
fn contention(b: &mut Bencher) {
    b.iter(|| task::block_on(run(100, 1000)));
}

#[bench]
fn no_contention(b: &mut Bencher) {
    b.iter(|| task::block_on(run(1, 10000)));
}

async fn run(task: usize, iter: usize) {
    let m = Arc::new(Mutex::new(()));
    let mut tasks = Vec::new();

    for _ in 0..task {
        let m = m.clone();
        tasks.push(task::spawn(async move {
            for _ in 0..iter {
                let _ = m.lock().await;
            }
        }));
    }

    for t in tasks {
        t.await;
    }
}
