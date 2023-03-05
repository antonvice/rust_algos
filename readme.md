# Rust algorithms

## Sort
In this implementation, we define a quicksort function that takes a mutable slice of a generic type T that implements the Ord trait. We first check if the length of the slice is less than 2, and if so, we simply return. Otherwise, we choose a pivot element and partition the slice around it. We then recursively call the quicksort function on the two resulting sub-slices.
The partition function takes a mutable slice of a generic type T that implements the Ord trait and returns the index of the pivot element. We choose the pivot to be the last element of the slice, and then we iterate over the slice and swap elements as necessary to partition the slice around the pivot. Finally, we swap the pivot element into its correct position and return its index.
In the main function, we create an array of integers and call the quicksort function on it. We then print the sorted array to the console using the println! macro.
This implementation of the QuickSort algorithm in Rust is a simple but efficient sorting algorithm that provides good performance on most inputs.

## Compression
In this example, we use the LZ77 compression algorithm to compress and decompress data. The compress function takes a byte slice as input and returns a compressed byte vector, while the decompress function takes a compressed byte slice as input and returns a decompressed byte vector.
The implementation uses a sliding window to search for the longest match in the data, and encodes matches as pairs of offset and length. If no match is found, it encodes the next symbol as a literal. The implementation also maintains the sliding window size to limit memory usage.
This is a more elegant implementation of a compression algorithm in Rust, from scratch, using a well-known algorithm. It demonstrates Rust's ability to handle complex algorithms with efficient memory management and high performance.
