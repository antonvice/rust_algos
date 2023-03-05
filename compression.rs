fn compress(data: &[u8]) -> Vec<u8> {
    let mut compressed = Vec::new();
    let mut window = VecDeque::new();
    let mut pos = 0;

    while pos < data.len() {
        let mut max_match_len = 0;
        let mut match_pos = 0;

        // search for the longest match in the sliding window
        for i in 0..window.len() {
            let mut match_len = 0;
            while pos + match_len < data.len()
                && window[i + match_len] == data[pos + match_len]
                && match_len < 255
            {
                match_len += 1;
            }

            if match_len > max_match_len {
                max_match_len = match_len;
                match_pos = i;
            }
        }

        // encode the match as a pair of offset and length
        if max_match_len > 2 {
            compressed.push((match_pos >> 8) as u8);
            compressed.push(match_pos as u8);
            compressed.push(max_match_len as u8);
            pos += max_match_len;
        } else {
            // no match found, encode the next symbol as a literal
            compressed.push(data[pos]);
            window.push_back(data[pos]);
            pos += 1;
        }

        // maintain the sliding window size
        if window.len() > 4096 {
            window.pop_front();
        }
    }

    compressed
}

fn decompress(data: &[u8]) -> Vec<u8> {
    let mut decompressed = Vec::new();
    let mut window = VecDeque::new();
    let mut pos = 0;

    while pos < data.len() {
        let symbol = data[pos];
        pos += 1;

        if symbol >= 192 {
            // decode a match from a pair of offset and length
            let offset = ((symbol as u16 - 192) << 8) | data[pos] as u16;
            pos += 1;

            let length = symbol & 0x1F;
            for i in 0..length {
                let byte = window[(offset + i) as usize];
                decompressed.push(byte);
                window.push_back(byte);
            }
        } else {
            // decode a literal symbol
            decompressed.push(symbol);
            window.push_back(symbol);
        }

        // maintain the sliding window size
        if window.len() > 4096 {
            window.pop_front();
        }
    }

    decompressed
}

fn main() {
    let data = b"hello world!hello world!hello world!";
    let compressed = compress(data);
    let decompressed = decompress(&compressed);
    assert_eq!(data, &decompressed[..]);
    println!("Compression successful!");
}
