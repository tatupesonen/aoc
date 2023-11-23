/// Rotates a map by
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn chebyshev_distance(x: (i32, i32), y: (i32, i32)) -> i32 {
    std::cmp::max((x.0 - y.0).abs(), (x.1 - y.1).abs())
}

pub fn rotate_right<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    transpose(v)
        .into_iter()
        .map(|e| e.into_iter().rev().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chebyshev() {
        let first = (-3, 0);
        let second = (0, 0);

        assert_eq!(chebyshev_distance(first, second), 3);

        //     012345
        //     ||||||
        // 0 - ......
        // 1 - .x....
        // 2 - ......
        // 3 - ......
        // 4 - ..x...
        // 5 - ......
    }
}
