use std::collections::{HashMap, HashSet};

use pylist_expr::{dict, iter, set, list};

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2..=x.isqrt() {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let b = list![x.pow(2) for x in a.iter() if is_prime(**x)];
    assert_eq!(b, vec![4, 9, 25]);

    let a = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]];
    let b = list![j + 1 for i in a.into_iter() if i.len() == 3 for j in i.into_iter() if *j > 2 && is_prime(*j)];
    assert_eq!(b, vec![4, 6]);

    let b = list![k * v for (k, v) in [1, 2, 3].into_iter().zip([1, 2, 4].into_iter())];
    assert_eq!(b, vec![1, 4, 12]);

    let mut c = HashMap::new();
    for i in 1..=10 {
        c.insert(i, i);
    }
    let d = dict! { i: i for i in (1..=10) };
    assert_eq!(d, c);

    let c = b.iter().map(|i| *i).enumerate().collect::<HashMap<_, _>>();
    let d = dict! { k: b[k] for k in (0..b.len()) };
    assert_eq!(d, c);

    let c = b
        .iter()
        .map(|i| *i)
        .enumerate()
        .filter(|(_, v)| *v > 1)
        .collect::<HashMap<_, _>>();
    let d = dict! { k: b[k] for k in (0..b.len()) if b[*k] > 1 };
    assert_eq!(d, c);

    let b = list![i for i in (0..10)];
    let c = (0..10)
        .map(|k| {
            (
                k,
                b.iter()
                    .filter(|i| **i != k)
                    .map(|i| *i)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    let d = dict! {k: list![*i for i in b.iter() if **i != k] for k in (0..10)};
    assert_eq!(d, c);

    let e = (1..=10)
        .flat_map(|i| (1..=10).map(move |j| i * j))
        .collect::<HashSet<_>>();
    let s = set! { i * j for i in (1..=10) for j in (1..=10) };
    assert_eq!(s, e);

    let i = iter!(c for c in ("Hello Steve!".chars()) if *c != ' ').collect::<String>();
    assert_eq!(i, "HelloSteve!");
}
