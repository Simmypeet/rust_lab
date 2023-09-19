pub fn extract_non_negatives(nums: &[isize]) -> Vec<usize> {
    let mut positives = Vec::new();

    for num in nums {
        if *num >= 0 {
            positives.push(*num as usize)
        }
    }

    positives
}

pub fn extract_non_negatives_r(nums: &[isize]) -> Vec<usize> {
    if let Some(last) = nums.last().copied() {
        let mut previous = extract_non_negatives_r(&nums[..nums.len() - 1]);

        if last >= 0 {
            previous.push(last as usize);
        }

        previous
    } else {
        Vec::new()
    }
}

pub fn split_non_negatives(nums: &[isize]) -> (Vec<usize>, Vec<isize>) {
    let mut positives = Vec::new();
    let mut negatives = Vec::new();

    for num in nums.iter().copied() {
        if num >= 0 { positives.push(num as usize);
        } else {
            negatives.push(num)
        }
    }

    (positives, negatives)
}

#[cfg(test)]
mod tests {
    use crate::{extract_non_negatives, extract_non_negatives_r, split_non_negatives};

    fn validate(f: impl Fn(&[isize]) -> Vec<usize>) {
        // empty array case
        assert_eq!(f(&[]), []);

        // only positives case
        assert_eq!(f(&[1, 68, 420, 0]), [1, 68, 420, 0]);

        // only negatives case
        assert_eq!(f(&[-1, -68, -420]), []);

        // mixed case
        assert_eq!(f(&[-0, 32, -64, 68, -420]), [0, 32, 68]);
    }

    #[test]
    fn test_extract_non_negatives() {
        validate(extract_non_negatives);
        validate(extract_non_negatives_r);
    }

    #[test]
    fn test_split_non_negatives() {
        // empty array case
        assert_eq!(split_non_negatives(&[]), (vec![], vec![]));

        // only positives case
        assert_eq!(
            split_non_negatives(&[1, 68, 420, 0]),
            (vec![1, 68, 420, 0], vec![])
        );

        // only negatives case
        assert_eq!(
            split_non_negatives(&[-1, -68, -420]),
            (vec![], vec![-1, -68, -420])
        );

        // mixed case
        assert_eq!(
            split_non_negatives(&[-0, 32, -64, 68, -420]),
            (vec![0, 32, 68], vec![-64, -420])
        );
    }
}
