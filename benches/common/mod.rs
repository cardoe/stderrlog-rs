extern crate test;

use super::{init_logger, util::with_redirected_stderr};
use log::*;
use std::panic;
use test::Bencher;

#[bench]
fn simple_string(b: &mut Bencher) {
    init_logger();
    with_redirected_stderr(panic::AssertUnwindSafe(|| {
        b.iter(|| {
            debug!("Whoami");
        });
    }));
}

#[bench]
fn complex_format(b: &mut Bencher) {
    init_logger();
    with_redirected_stderr(panic::AssertUnwindSafe(|| {
        b.iter(|| {
            debug!(
                "{}, {:#?}, {:b}",
                0.1f64,
                vec![99, 1, 5, 100, 1, 0, 8],
                0xffb1aa
            )
        })
    }));
}

#[bench]
fn long_line(b: &mut Bencher) {
    let mut long_data = vec![];
    for _ in 0..1_000 {
        long_data.push("this is an item in a long list");
    }

    init_logger();
    with_redirected_stderr(panic::AssertUnwindSafe(|| {
        b.iter(|| {
            debug!("long data: {:?}", long_data);
        })
    }));
}
