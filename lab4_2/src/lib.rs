pub fn count_negative(v: &[i64]) -> usize {
    v.iter().filter(|&&x| x < 0).count()
}

#[test]
fn test_counting() {
    assert_eq!(count_negative(&[]), 0);
    assert_eq!(count_negative(&[1, 2, -3, 4, -6, 7]), 2);
}

pub fn doubles(v: &[i64]) -> Vec<i64> {
    let mut result = Vec::new();
    for x in v {
        result.push(2 * x);
    }
    result
}

// no loop
fn doubles_recursion(v: &[i64], mut result: Vec<i64>, index: usize) -> Vec<i64> {
    if index == v.len() {
        result
    } else {
        result.push(2 * v[index]);
        doubles_recursion(v, result, index + 1)
    }
}

pub fn doubles_recursion_start(v: &[i64]) -> Vec<i64> {
    doubles_recursion(v, Vec::new(), 0)
}

#[test]
fn test_doubles() {
    assert_eq!(doubles(&[]), vec![]);
    assert_eq!(doubles(&[1, 2, 3]), vec![2, 4, 6]);
    assert_eq!(doubles_recursion_start(&[]), vec![]);
    assert_eq!(doubles_recursion_start(&[1, 2, 3]), vec![2, 4, 6]);
}
