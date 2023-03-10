fn main() {
    let mut arr = [4, 3, 2, 1];
    bubble_sort(&mut arr);

    assert_eq!(arr, [1, 2, 3, 4]);
}


/// `bubble_sort` is a bubble sort implementation in Rust.
/// # Example
///
/// ```rust
/// let mut arr = [4, 3, 2, 1];
/// bubble_sort(&mut arr);
///
/// assert_eq!(arr, [1, 2, 3, 4]);
/// ```
fn bubble_sort<const N: usize, T: Clone + PartialOrd + Copy>(arr: &mut [T; N]) {
    for i in 0..N {
        for j in 0..i {
            if arr[j] > arr[i] {
                arr.swap(i, j);
            }
        }
    }
}