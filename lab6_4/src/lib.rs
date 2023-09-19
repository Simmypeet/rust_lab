fn pack_number_tuple_internal(first: &[i32], second: &[i32], min: bool) -> Vec<(i32, i32)> {
    let max = if min {
        std::cmp::min(first.len(), second.len())
    } else {
        std::cmp::max(first.len(), second.len())
    };

    let mut result = Vec::new();
    let mut index = 0;

    while index < max {
        let first = first.get(index).copied().unwrap_or(0);
        let second = second.get(index).copied().unwrap_or(0);

        result.push((first, second));

        index += 1;
    }

    result
}

pub fn pack_number_tuples(first: &[i32], second: &[i32]) -> Vec<(i32, i32)> {
    pack_number_tuple_internal(first, second, false)
}

pub fn pack_number_tuples_s(first: &[i32], second: &[i32]) -> Vec<(i32, i32)> {
    pack_number_tuple_internal(first, second, true)
}

#[cfg(test)]
mod tests {
    use crate::{pack_number_tuples, pack_number_tuples_s};

    #[test]
    fn pack_number_tuples_test() {
        // equal length case
        assert_eq!(
            pack_number_tuples(&[1, 2, 5], &[6, 4, 2]),
            vec![(1, 6), (2, 4), (5, 2)]
        );

        // first array is longer
        assert_eq!(
            pack_number_tuples(&[1, 2, 5, 6], &[6, 4, 2]),
            vec![(1, 6), (2, 4), (5, 2), (6, 0)]
        );

        // second array is longer
        assert_eq!(
            pack_number_tuples(&[1, 2, 5], &[6, 4, 2, 1, 2]),
            vec![(1, 6), (2, 4), (5, 2), (0, 1), (0, 2)]
        );

        // empty arrays
        assert_eq!(pack_number_tuples(&[], &[]), vec![]);

        assert_eq!(pack_number_tuples(&[], &[]), []);
        assert_eq!(pack_number_tuples(&[1], &[]), [(1, 0)]);
        assert_eq!(pack_number_tuples(&[], &[2, 3]), [(0, 2), (0, 3)]);
        assert_eq!(
            pack_number_tuples(&[5, 1, 4], &[2, 3]),
            [(5, 2), (1, 3), (4, 0)]
        );
    }

    #[test]
    fn pack_number_tuples_s_tests() {
        // equal length case
        assert_eq!(
            pack_number_tuples_s(&[1, 2, 5], &[6, 4, 2]),
            vec![(1, 6), (2, 4), (5, 2)]
        );

        // first array is longer
        assert_eq!(
            pack_number_tuples_s(&[1, 2, 5, 6], &[6, 4, 2]),
            vec![(1, 6), (2, 4), (5, 2)]
        );

        // second array is longer
        assert_eq!(
            pack_number_tuples_s(&[1, 2, 5], &[6, 4, 2, 1, 2]),
            vec![(1, 6), (2, 4), (5, 2)]
        );

        // empty arrays
        assert_eq!(pack_number_tuples_s(&[], &[]), vec![]);

        assert_eq!(pack_number_tuples_s(&[], &[]), []);
        assert_eq!(pack_number_tuples_s(&[1], &[]), []);
        assert_eq!(pack_number_tuples_s(&[], &[2, 3]), []);
        assert_eq!(pack_number_tuples_s(&[5, 1, 4], &[2, 3]), [(5, 2), (1, 3)]);
    }
}
