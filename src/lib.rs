//! Implements bisection operations on sorted slices. 
//!
//! Bisection, as the word bisect hints to, is the division of a quantity in two
//! parts. Though similar in spirit and interface to `slice::binary_search`, the bisection
//! operations defined here allow finer control of the position returned.
//!
//! Inspired by Python's [bisect](https://docs.python.org/3/library/bisect.html), two operations
//! are defined, bisecting left and bisecting right. The names indicate the location of the index returned
//! relative to the element found. 
#![warn(rust_2018_idioms, nonstandard_style, missing_docs)]
use std::cmp::Ordering::{self, Equal, Greater, Less};

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
/// use bisect_rs::bisect_right;
/// let u = [0, 1, 2, 2, 3, 4];
///
/// assert_eq!(bisect_right(&u, &4), 6);
/// assert_eq!(bisect_right(&u, &2), 4);
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
/// use bisect_rs::bisect_right_by_key;
///
/// let u = vec![(1, 3), (5, 5), (5, 6), (10, 2), (11, 2)];
///
/// let target = (5, 1);
/// assert_eq!(bisect_right_by_key(&u, &target.0, |&(a, _)| a), 3);
/// let target = (1, 1);
/// assert_eq!(bisect_right_by_key(&u, &target.0, |&(a, _)| a), 1);
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
/// use bisect_rs::bisect_right_by;
///
/// let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
///
/// let seek = 13;
/// assert_eq!(bisect_right_by(&s, |probe| probe.cmp(&seek)), 10);
/// let seek = 1;
/// assert_eq!(bisect_right_by(&s, |probe| probe.cmp(&seek)), 5);
/// let seek = 100;
/// assert_eq!(bisect_right_by(&s, |probe| probe.cmp(&seek)), 13); 
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
/// use bisect_rs::bisect_left;
/// let u = [0, 1, 2, 2, 3, 4];
/// 
/// assert_eq!(bisect_left(&u, &3), 4);
/// assert_eq!(bisect_left(&u, &2), 2);
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
/// use bisect_rs::bisect_right_by_key;
///
/// let u = vec![(1, 3), (5, 5), (5, 6), (10, 2), (11, 2)];
///
/// let target = (5, 1);
/// assert_eq!(bisect_right_by_key(&u, &target.0, |&(a, _)| a), 3);
/// let target = (11, 1);
/// assert_eq!(bisect_right_by_key(&u, &target.0, |&(a, _)| a), 5);
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
/// use bisect_rs::bisect_left_by;
///
/// let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
///
/// let seek = 13;
/// assert_eq!(bisect_left_by(&s, |probe| probe.cmp(&seek)), 9);
/// let seek = 1;
/// assert_eq!(bisect_left_by(&s, |probe| probe.cmp(&seek)), 1);
/// let seek = 100;
/// assert_eq!(bisect_left_by(&s, |probe| probe.cmp(&seek)), 13); 
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

// I'm still unsure if this is really needed. Searching Github for usages of
// bisecting with bounds specified in Python, I currently only found hits for code
// implementing a Trie (and my quick glance of it gives me the impression it might be
// for optimization purposes).
//
// If needed, all bounded forms follow the pattern as seen in `bisect_slice_right`. 
// We grab the start of the bound, and return start + bisect_*(slice[bound], x). 
#[allow(dead_code)]
fn bisect_slice_right<T, B>(a: &[T], x: &T, bound: B) -> usize 
where 
    T: Ord,
    B: std::ops::RangeBounds<usize> + std::slice::SliceIndex<[T], Output = [T]>
{
    let start = bounds_start(&bound);
    // Note: Invalid bound panics here.
    start + bisect_right_by(&a[bound], |k| k.cmp(x))
}

#[allow(dead_code)]
#[inline]
fn bounds_start<B>(bounds: &B) -> usize
where 
    B: std::ops::RangeBounds<usize> 
{
    match bounds.start_bound() {
        std::ops::Bound::Unbounded => 0,
        std::ops::Bound::Excluded(&x) => x-1,
        std::ops::Bound::Included(&x) => x,
    }
}