use rand::Rng;

pub fn filter_number_list(num_list: &[f64]) -> Vec<f64> {
    let mut result = Vec::new();
    for &num in num_list {
        if (-1.0..=1.0).contains(&num) {
            result.push(num);
        }
    }

    result
}

pub fn gen_numbers<R: Rng>(r: &mut R, n: usize) -> Vec<f64> {
    let mut result = Vec::new();
    for _ in 0..n {
        result.push(r.gen_range(-10.0..=10.0));
    }

    result
}

fn main() {
    let n = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();
    let mut rng = rand::thread_rng();
    let nums = gen_numbers(&mut rng, n);
    let mut count = 0;
    for num in &nums {
        if (-1.0..=1.0).contains(num) {
            count += 1;
        }
    }
    let prob = count as f64 / n as f64;

    println!("Probability: {}", prob);
}

#[cfg(test)]
mod tests {
    #[test]
    fn filter_number() {
        let list = [2.0, 1.0, 0.0, -1.0, -2.0];
        assert_eq!(super::filter_number_list(&list), [1.0, 0.0, -1.0])
    }

    #[test]
    fn gen_numbers() {
        let mut rng = rand::thread_rng();
        let nums = super::gen_numbers(&mut rng, 10);

        assert_eq!(nums.len(), 10);

        for num in nums {
            assert!((-10.0..=10.0).contains(&num));
        }
    }
}
