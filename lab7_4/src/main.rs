use std::{
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

use rand::Rng;

pub fn filter_points(pt_list: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    for &(x, y) in pt_list {
        let d = (x * x + y * y).sqrt();
        if d <= 1. {
            result.push((x, y));
        }
    }
    result
}

pub fn gen_points<R: Rng>(rng: &mut R, n: usize) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    for _ in 0..n {
        let x = rng.gen_range(-1.0..=1.0);
        let y = rng.gen_range(-1.0..=1.0);
        result.push((x, y));
    }
    result
}

fn main() {
    println!("started");
    let mutex = Arc::new(Mutex::new(()));
    let condvar = Arc::new(Condvar::new());
    let condvar_clone = condvar.clone();
    // Inside of our lock, spawn a new thread, and then wait for it to start.
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(5));
        condvar_clone.notify_one();
    });

    let mut joins = Vec::new();
    for _ in 0..10 {
        let condvar = condvar.clone();
        let mutex = mutex.clone();

        joins.push(std::thread::spawn(move || {
            condvar.wait(mutex.lock().unwrap());
            println!("yo finished?");
        }));
    }

    for join in joins {
        join.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn filter_point() {
        // 1 point in, 2 points out
        let points = [(0.5, 0.4), (1.0, 1.0), (-1.0, -1.0)];
        assert_eq!(super::filter_points(&points), [(0.5, 0.4)]);
    }

    #[test]
    fn gen_points() {
        let mut rng = rand::thread_rng();
        let points = super::gen_points(&mut rng, 10);

        assert_eq!(points.len(), 10);

        for (x, y) in points {
            assert!((-1.0..=1.0).contains(&x));
            assert!((-1.0..=1.0).contains(&y));
        }
    }
}
