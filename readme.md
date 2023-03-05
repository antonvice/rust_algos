# Rust algorithms

## Sort
In this implementation, we use the quicksort algorithm to sort an array of integers. The quicksort function takes a mutable slice of integers as input and recursively sorts the array in-place.
The implementation chooses the last element of the array as the pivot element and partitions the array into two halves based on the pivot element. It then recursively sorts the left and right halves of the array.
This is an elegant implementation of the quicksort algorithm in Rust, from scratch, using efficient memory management and high performance.

## Compression
In this example, we use the LZ77 compression algorithm to compress and decompress data. The compress function takes a byte slice as input and returns a compressed byte vector, while the decompress function takes a compressed byte slice as input and returns a decompressed byte vector.
The implementation uses a sliding window to search for the longest match in the data, and encodes matches as pairs of offset and length. If no match is found, it encodes the next symbol as a literal. The implementation also maintains the sliding window size to limit memory usage.
This is a more elegant implementation of a compression algorithm in Rust, from scratch, using a well-known algorithm. It demonstrates Rust's ability to handle complex algorithms with efficient memory management and high performance.


## Cypher
In this implementation, we use the Caesar cipher algorithm to encrypt a given text by shifting each letter by a given number of positions. The caesar_cipher function takes a string as input and an unsigned 8-bit integer as the shift value. It returns the ciphered text as a string.
The implementation iterates through each character of the input text and checks if it is an ASCII alphabetic character. If it is, the implementation calculates the new position of the character after the shift by subtracting the base value of the character (either 'A' or 'a') and adding the shift value. The result is then converted back to the corresponding ASCII character and added to the result string. If the character is not an ASCII alphabetic character, it is directly added to the result string.
Then, we use the same logic as the encryption algorithm to decrypt a given text that was ciphered with the Caesar cipher algorithm. The caesar_decipher function takes a string as input and an unsigned 8-bit integer as the shift value. It returns the deciphered text as a string.
The implementation iterates through each character of the input text and checks if it is an ASCII alphabetic character. If it is, the implementation calculates the original position of the character before the shift by subtracting the base value of the character (either 'A' or 'a') and adding 26 (the number of letters in the alphabet) and then subtracting the shift value. The result is then converted back to the corresponding ASCII character and added to the result string. If the character is not an ASCII alphabetic character, it is directly added to the result string.

