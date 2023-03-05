fn quicksort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[arr.len() - 1];
    let mut i = 0;

    for j in 0..arr.len() - 1 {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, arr.len() - 1);
    let (left, right) = arr.split_at_mut(i);

    quicksort(left);
    quicksort(&mut right[1..]);
}

fn main() {
    let mut arr = vec![5, 2, 9, 1, 5, 6, 3];
    quicksort(&mut arr);
    assert_eq!(arr, vec![1, 2, 3, 5, 5, 6, 9]);
    println!("Sorting successful!");
}
