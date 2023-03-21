use rand::{Rng, thread_rng};
use std::time::{Instant};

fn main() {
    let mut rng = thread_rng();
    let mut arr: [u32; 100000] = [0; 100000];

    for i in 0..arr.len() {
        arr[i] = rng.gen_range(0..1_000_000);
    }

    let start_time = Instant::now();
    quicksort(&mut arr);
    let elapsed_time = start_time.elapsed().as_micros();
    println!("Quicksort Time: {} microseconds", elapsed_time);

    let start_time = Instant::now();
    mergesort(&mut arr);
    let elapsed_time = start_time.elapsed().as_micros();
    println!("Mergesort Time: {} microseconds", elapsed_time);

    let start_time = Instant::now();
    heapsort(&mut arr);
    let elapsed_time = start_time.elapsed().as_micros();
    println!("Heapsort Time: {} microseconds", elapsed_time);
}

fn quicksort(arr: &mut [u32]) {
    if arr.len() > 1 {
        let pivot_index = partition(arr);
        quicksort(&mut arr[..pivot_index]);
        quicksort(&mut arr[pivot_index + 1..]);
    }
}

fn partition(arr: &mut [u32]) -> usize {
    let pivot_index = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot_index {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_index);
    i
}

fn mergesort(arr: &mut [u32]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        mergesort(&mut arr[..mid]);
        mergesort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

fn merge(arr: &mut [u32], mid: usize) {
    let mut left_arr = arr[..mid].to_vec();
    let mut right_arr = arr[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left_arr.len() && j < right_arr.len() {
        if left_arr[i] < right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_arr.len() {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < right_arr.len() {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn heapsort(arr: &mut [u32]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [u32], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && arr[l] > arr[largest] {
        largest = l;
    }

    if r < n && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}
