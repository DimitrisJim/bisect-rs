//! Implements bisection operations on sorted slices.
#[warn(missing_docs)]
use std::cmp::Ordering::{self, Equal, Greater, Less};
// Todo: further expand on crate-level doc.

/// Search for the element and return the rightmost index that can be used to insert
/// it into the sorted slice while maintaining sort order.
///
/// The return value `i` is such that all `e` in `&a[..i]` have `e <= x`, and all `e` in
/// `&a[i..]` have `e > x`.  So if `x` already appears in the list, inserting it will
/// insert **just after** the rightmost `x` already there.
///
/// # Examples
///
/// Basic usage:
///
/// Todo: Add more examples.
///
/// ```
/// use bisectrs::bisect_right;
/// let u = [0, 1, 2, 2, 3, 4];
/// let idx = bisect_right(&u, &2);
///
/// assert_eq!(idx, 4);
/// ```
/// # Panics
///
/// In `slice.len() == usize::MAX` and a search is made for an element that's equal or larger than
/// the last element in the slice, overflow occurs due to trying to add one to the max value of `usize`.
pub fn bisect_right<T>(a: &[T], x: &T) -> usize
where
    T: Ord,
{
    bisect_right_by(a, |k| k.cmp(x))
}

/// Search for an element using a key extraction function and return the rightmost index
/// that can be used to insert it into the sorted slice while maintaining sort order.
///
/// Assumes that the slice is sorted by the key, for instance with
/// `sort_by_key` using the same key extraction function.
///
/// The return value `i` is such that all `e` in `&a[..i]` have `e <= x`, and all `e` in
/// `&a[i..]` have `e > x`.  So if `x` already appears in the list, inserting it will
/// insert **just after** the rightmost `x` already there.
///
/// # Examples
///
/// Todo: Add more examples.
///
/// Basic usage:
///
/// ```
/// use bisectrs::bisect_right_by_key;
/// let u = [0, 1, 2, 2, 3, 4];
/// ```
///
/// # Panics
///
/// In `slice.len() == usize::MAX` and a search is made for an element that's equal or larger than
/// the last element in the slice, overflow occurs due to trying to add one to the max value of `usize`.
pub fn bisect_right_by_key<T, B, F>(a: &[T], b: &B, mut f: F) -> usize
where
    T: Ord,
    B: Ord,
    F: FnMut(&T) -> B,
{
    bisect_right_by(a, |k| f(k).cmp(b))
}

/// Search for an element using a comparator function and return the rightmost index
/// that can be used to insert it into the sorted slice while maintaining sort order.
///
/// The comparator function should implement an order consistent
/// with the sort order of the underlying slice, returning an
/// order code that indicates whether its argument is `Less`,
/// `Equal` or `Greater` the desired target.
///
/// The return value `i` is such that all `e` in `&a[..i]` have `e <= x`, and all `e` in
/// `&a[i..]` have `e > x`.  So if `x` already appears in the list, inserting it will
/// insert **just after** the rightmost `x` already there.
///
/// # Examples
///
/// Todo: Provide more examples.
///
/// Basic usage:
///
/// ```
/// use bisectrs::bisect_right_by;
/// let u = [0, 1, 2, 2, 3, 4];
/// ```
///
/// # Panics
///
/// In `slice.len() == usize::MAX` and a search is made for an element that's equal or larger than
/// the last element in the slice, overflow occurs due to trying to add one to the max value of `usize`.
pub fn bisect_right_by<T, F>(a: &[T], mut f: F) -> usize
where
    T: Ord,
    F: FnMut(&T) -> Ordering,
{
    let mut high = a.len();
    if high == 0 {
        return 0;
    }
    let mut low = 0;
    while low < high {
        let mid = (low + high) / 2;
        // SAFETY: mid always ranges between [0, high).
        //         Lowest possible value of mid occurs when low == 0 and high == 1, mid == 0.
        //         Highest possible value of mid occurs when high = a.len() and low == high-1,
        //         in that case mid will be (2 * high - 1) / 2. This equals (high - 0.5) which,
        //         due to truncation will result in a value of (high - 1).
        match f(unsafe { a.get_unchecked(mid) }) {
            Less | Equal => low = mid + 1, // a[mid] <= x
            Greater => high = mid,         // a[mid] > x
        }
    }
    low
}

/// Search for the element and return the leftmost index that can be used to insert
/// it into the sorted slice while maintaining sort order.
///
/// The return value `i` is such that all `e` in `&a[..i]` have `e < x`, and all `e` in
/// `&a[i..]` have `e >= x`.  So if `x` already appears in the list, inserting it will
/// insert **just before** the leftmost `x` already there.
///
/// # Examples
///
/// Todo: Provide more examples.
///
/// Basic usage:
///
/// ```
/// use bisectrs::bisect_left;
/// let u = [0, 1, 2, 2, 3, 4];
/// let idx = bisect_left(&u, &2);
///
/// assert_eq!(idx, 2);
/// ```
pub fn bisect_left<T>(a: &[T], x: &T) -> usize
where
    T: Ord,
{
    bisect_left_by(a, |k| k.cmp(x))
}

/// Search for an element using a key extraction function and return the leftmost index
/// that can be used to insert it into the sorted slice while maintaining sort order.
///
/// Assumes that the slice is sorted by the key, for instance with
/// `sort_by_key` using the same key extraction function.
///
/// The return value `i` is such that all `e` in `&a[..i]` have `e < x`, and all `e` in
/// `&a[i..]` have `e >= x`.  So if `x` already appears in the list, inserting it will
/// insert **just before** the leftmost `x` already there.
///
/// # Examples
///
/// Todo: Provide more examples.
///
/// Basic usage:
///
/// ```
/// use bisectrs::bisect_left_by_key;
/// let u = [0, 1, 2, 2, 3, 4];
/// ```
pub fn bisect_left_by_key<T, B, F>(a: &[T], b: &B, mut f: F) -> usize
where
    T: Ord,
    B: Ord,
    F: FnMut(&T) -> B,
{
    bisect_left_by(a, |k| f(k).cmp(b))
}

/// Search for an element using a comparator function and return the leftmost index
/// that can be used to insert it into the sorted slice while maintaining sort order.
///
/// The comparator function should implement an order consistent
/// with the sort order of the underlying slice, returning an
/// order code that indicates whether its argument is `Less`,
/// `Equal` or `Greater` the desired target.
///
/// The return value `i` is such that all `e` in `&a[..i]` have `e < x`, and all `e` in
/// `&a[i..]` have `e >= x`.  So if `x` already appears in the list, inserting it will
/// insert **just before** the leftmost `x` already there.
///
/// # Examples
///
/// Todo: Provide more examples.
///
/// Basic usage:
///
/// ```
/// use bisectrs::bisect_left_by;
/// let u = [0, 1, 2, 2, 3, 4];
/// ```
pub fn bisect_left_by<T, F>(a: &[T], mut f: F) -> usize
where
    T: Ord,
    F: FnMut(&T) -> Ordering,
{
    let mut high = a.len();
    if high == 0 {
        return 0;
    }
    let mut low = 0;
    while low < high {
        let mid = (low + high) / 2;
        // SAFETY: mid always ranges between [0, high).
        //         Lowest possible value of mid occurs when low == 0 and high == 1, mid == 0.
        //         Highest possible value of mid occurs when high = a.len() and low == high-1,
        //         in that case mid will be (2 * high - 1) / 2. This equals (high - 0.5) which,
        //         due to truncation will result in a value of (high - 1).
        match f(unsafe { a.get_unchecked(mid) }) {
            Less => low = mid + 1,         // a[mid] < x
            Greater | Equal => high = mid, // a[mid] >= x
        }
    }
    low
}
