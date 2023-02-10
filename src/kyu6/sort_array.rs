//https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust

#[derive(Debug, Clone)]
enum T {
    Number(i32),
    Still(String),
}

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut res: Vec<T> = vec![];
    let mut odds: Vec<i32> = vec![];
    for (i, v) in arr.iter().enumerate() {
        if v % 2 == 0 {
            res.push(T::Number(v.clone()))
        } else {
            res.push(T::Still("".to_string()));
            odds.push(v.clone());
        }
    }
    odds.sort();
    let mut a = 0;
    for (i, v) in res.clone().iter().enumerate() {
        match v {
            T::Still(_) => {
                let s = res[i] = T::Number(odds[a]);
                a += 1;
                s
            }
            _ => (),
        }
    }
    res.iter()
        .filter(
            (|d| match d {
                T::Number(_) => true,
                _ => false,
            }),
        )
        .map(|d| match d {
            T::Number(v) => *v,
            T::Still(_) => 2,
        })
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}
