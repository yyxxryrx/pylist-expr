use pylist_expr::list;

#[test]
fn simple() {
    let a = list![i for i in [1, 2, 3].into_iter()];
    assert_eq!(a, vec![1, 2, 3]);

    let a = list![i * 2 for i in (1..=6)];
    assert_eq!(a, vec![2, 4, 6, 8, 10, 12]);

    let a = list![i for i in [1, 2, 3, 4, 5, 6].into_iter() if i % 2 == 0];
    assert_eq!(a, vec![2, 4, 6]);

    let a = list![i - 1 for i in (1..7) if i % 2 == 0];
    assert_eq!(a, vec![1, 3, 5]);
}

#[test]
fn nested() {
    let a = list![i * j for i in (0..10) for j in (0..3)];
    let b = (0..10)
        .flat_map(|i| (0..3).map(move |j| i * j))
        .collect::<Vec<_>>();
    assert_eq!(a, b);

    let a = list![i * j for i in (0..10) for j in (0..3) if i + j > 10];
    let b = (0..10)
        .flat_map(|i| (0..3).filter(move |j| i + j > 10).map(move |j| i * j))
        .collect::<Vec<_>>();
    assert_eq!(a, b);
}
