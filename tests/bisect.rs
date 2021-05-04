use bisectrs::{bisect_left, bisect_right};

// Todo: Document others? Any point in it?
#[test]
fn test_bisect_left() {
    let sl: [i32; 0] = [];
    assert_eq!(bisect_left(&sl, &0), 0);

    let sl = [1, 1, 1, 2, 3];
    assert_eq!(bisect_left(&sl, &1), 0);
    let b = [4];
    assert_eq!(bisect_left(&b, &3), 0);
    assert_eq!(bisect_left(&b, &4), 0);
    assert_eq!(bisect_left(&b, &5), 1);

    let b = [1, 2, 4, 6, 8, 9];
    assert_eq!(bisect_left(&b, &5), 3);
    assert_eq!(bisect_left(&b, &6), 3);
    assert_eq!(bisect_left(&b, &7), 4);
    assert_eq!(bisect_left(&b, &8), 4);

    let b = [1, 2, 4, 5, 6, 8];
    assert_eq!(bisect_left(&b, &9), 6);

    let b = [1, 2, 4, 6, 7, 8, 9];
    assert_eq!(bisect_left(&b, &6), 3);
    assert_eq!(bisect_left(&b, &5), 3);
    assert_eq!(bisect_left(&b, &8), 5);

    let b = [1, 2, 4, 5, 6, 8, 9];
    assert_eq!(bisect_left(&b, &7), 5);
    assert_eq!(bisect_left(&b, &0), 0);

    let b = [1, 3, 3, 3, 7];
    assert_eq!(bisect_left(&b, &0), 0);
    assert_eq!(bisect_left(&b, &1), 0);
    assert_eq!(bisect_left(&b, &2), 1);
    assert_eq!(bisect_left(&b, &3), 1);
    assert_eq!(bisect_left(&b, &4), 4);
    assert_eq!(bisect_left(&b, &5), 4);
    assert_eq!(bisect_left(&b, &6), 4);
    assert_eq!(bisect_left(&b, &7), 4);
    assert_eq!(bisect_left(&b, &8), 5);

    let b = [(); usize::MAX];
    assert_eq!(bisect_left(&b, &()), 0);
}

#[test]
fn test_bisect_right() {
    let sl: [i32; 0] = [];
    assert_eq!(bisect_right(&sl, &0), 0);

    let sl = [1, 1, 1, 2, 3];
    assert_eq!(bisect_right(&sl, &1), 3);
    let b = [4];
    assert_eq!(bisect_right(&b, &3), 0);
    assert_eq!(bisect_right(&b, &4), 1);
    assert_eq!(bisect_right(&b, &5), 1);

    let b = [1, 2, 4, 6, 8, 9];
    assert_eq!(bisect_right(&b, &5), 3);
    assert_eq!(bisect_right(&b, &6), 4);
    assert_eq!(bisect_right(&b, &7), 4);
    assert_eq!(bisect_right(&b, &8), 5);

    let b = [1, 2, 4, 5, 6, 8];
    assert_eq!(bisect_right(&b, &9), 6);

    let b = [1, 2, 4, 6, 7, 8, 9];
    assert_eq!(bisect_right(&b, &6), 4);
    assert_eq!(bisect_right(&b, &5), 3);
    assert_eq!(bisect_right(&b, &8), 6);

    let b = [1, 2, 4, 5, 6, 8, 9];
    assert_eq!(bisect_right(&b, &7), 5);
    assert_eq!(bisect_right(&b, &0), 0);

    let b = [1, 3, 3, 3, 7];
    assert_eq!(bisect_right(&b, &0), 0);
    assert_eq!(bisect_right(&b, &1), 1);
    assert_eq!(bisect_right(&b, &2), 1);
    assert_eq!(bisect_right(&b, &3), 4);
    assert_eq!(bisect_right(&b, &4), 4);
    assert_eq!(bisect_right(&b, &5), 4);
    assert_eq!(bisect_right(&b, &6), 4);
    assert_eq!(bisect_right(&b, &7), 5);
    assert_eq!(bisect_right(&b, &8), 5);
}

#[test]
#[should_panic]
fn test_bisect_right_overflow() {
    // Not much I could do here. If a sequence filled with
    // equal elements of size usize::MAX is passed, the
    // function tries to return one past it.
    //
    // Looking at how Vec handles a similar case, a panic should
    // be expected here.
    let b = [(); usize::MAX];
    assert_eq!(bisect_right(&b, &()), usize::MAX);
}
