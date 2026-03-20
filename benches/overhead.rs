#![feature(test)]

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn bench_noop(b: &mut Bencher) {
    let mut guard = forkguard::noop::Guard::default();
    b.iter(|| {
        if guard.detected_fork() {
            black_box(0);
        }
    });
}

#[bench]
fn bench_pid(b: &mut Bencher) {
    let mut guard = forkguard::pid::Guard::default();
    b.iter(|| {
        if guard.detected_fork() {
            black_box(0);
        }
    });
}

#[cfg(all(unix, feature = "atfork"))]
#[bench]
fn bench_atfork(b: &mut Bencher) {
    let mut guard = forkguard::atfork::Guard::default();
    b.iter(|| {
        if guard.detected_fork() {
            black_box(0);
        }
    });
}
