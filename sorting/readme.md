# Github Technical Documentation for sorting algorithms
## Functionality
This code contains the implementation of three sorting algorithms i.e., QuickSort, MergeSort, and HeapSort. The program generates an array of 100000 random unsigned 32-bit integers using Rust's random library. Then it measures the performance of each algorithm by sorting the generated array using each algorithm independently and recording the elapsed time.

## Input
The input to this program is an unsorted array of 100000 random unsigned 32-bit integers.

## Output
The output of the program is the time taken to sort the input array using each of the three sorting algorithms - Quicksort, Mergesort, and Heapsort.

## Sorting Algorithms Used
### QuickSort
QuickSort is a divide-and-conquer algorithm that recursively partitions a given array around a pivot element. It works by selecting an element as a pivot and then partitioning the array around the pivot in such a way that all elements less than the pivot go to its left and all elements greater than the pivot go to its right. This process is repeated on the left and right sub-arrays until the array is sorted.

### MergeSort
MergeSort is also a divide-and-conquer algorithm that divides the given array into two halves, sorts each half recursively, and then merges the two sorted halves into a single sorted array. The merging process involves comparing the elements in both halves and placing them in their correct position in the merged array.

### HeapSort
HeapSort is another comparison-based sorting algorithm that uses a binary heap data structure to sort the given array. It first builds a max heap from the array, then repeatedly extracts the maximum element from the heap, places it at the end of the array, and restores the heap property.

## Performance
The performance of each algorithm is measured using Rust's built-in 
std::time::Instant
 class to record the start and end times of each sorting operation. The elapsed time is calculated by subtracting the start time from the end time. The results show that for the given input size of 100000, Quicksort and Mergesort perform better than Heapsort.

## How to run the code
Install Rust programming language by following the instructions from rust-lang.org.
Clone the repository containing the code file or create a new project using cargo.
Navigate into the project directory and execute the command 
```bash
cargo run
```
The program will generate an array of 100000 random unsigned 32-bit integers and measure the time taken to sort the array using each of the three sorting algorithms.
The output will be displayed on the console showing the elapsed time for each of the three sorting algorithms.
