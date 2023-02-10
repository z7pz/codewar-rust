//https://www.codewars.com/kata/5825792ada030e9601000782/train/rust
pub fn zip_with<F, T, S, R>(f: F, a: &[T], b: &[S]) -> Vec<R>
where
    F: Fn(T, S) -> R,
    T: Copy,
    S: Copy,
{
    let smallest = usize::min(a.len(), b.len());
    let (c, d) = (&a[..smallest], &b[..smallest]);
    let mut res: Vec<R> = vec![];
    for (i, _) in c.iter().enumerate() {
        res.push(f(c[i], d[i]));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::zip_with;
    use std::ops::{Add, Sub};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right).";

    #[test]
    fn basic_tests() {
        assert_eq!(
            &zip_with(i32::add, &[0, 1, 2, 3], &[0, 1, 2, 3]),
            &[0, 2, 4, 6],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::add, &[0, 1, 2, 3], &[0, 1, 2, 3]),
            &[0, 2, 4, 6],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::add, &[0, 1, 2, 3, 4, 5], &[6, 5, 4, 3, 2, 1]),
            &[6, 6, 6, 6, 6, 6],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::add, &[0, 1, 2, 3, 4], &[6, 5, 4, 3, 2, 1]),
            &[6, 6, 6, 6, 6],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::add, &[0, 1, 2, 3, 4, 5], &[6, 5, 4, 3, 2]),
            &[6, 6, 6, 6, 6],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::pow, &[10, 10, 10, 10], &[0, 1, 2, 3]),
            &[1, 10, 100, 1000],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::max, &[1, 4, 7, 1, 4, 7], &[4, 7, 1, 4, 7, 1]),
            &[4, 7, 7, 4, 7, 7],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::sub, &[0, 1, 2, 3, 4, 5], &[6, 5, 4, 3, 2, 1]),
            &[-6, -4, -2, 0, 2, 4],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(i32::pow, &[10; 10], &[0, 1, 2, 3, 4, 5, 6]),
            &[1, 10, 100, 1000, 10000, 100000, 1000000],
            "{ERR_MSG}"
        );
        assert_eq!(
            &zip_with(
                |c, n| std::iter::repeat(c).take(n).collect::<String>(),
                &['a', 'b', 'c', 'd', 'e', 'f'],
                &[6, 5, 4, 3, 2, 1]
            ),
            &["aaaaaa", "bbbbb", "cccc", "ddd", "ee", "f"],
            "{ERR_MSG}"
        );
    }
}
