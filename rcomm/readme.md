# Command Line Tool
This command-line tool is designed for file manipulation, text processing, and system monitoring. It reads an input file, processes the text using a regex pattern, and writes the results to an output file.

## Usage
To use this tool, first, compile the code using ```bash cargo build --release```. Then, run the following command:
```bash ./target/release/command_line_tool -i input.txt -o output.txt -p "regex_pattern" ```
Replace input.txt with the path to your input file, output.txt with the path to your output file, and regex_pattern with the regex pattern you want to search for.

## Features
* Read lines from an input file
* Process text using a regex pattern
* Write results to an output file
* Skip empty lines and comments (lines starting with //)
* Apply custom transformations to the matched lines (modify the custom_transformation function as needed)
* Error Handling
### The tool includes error handling for the following scenarios:
* Parsing command-line arguments
* Checking if the input file exists
* Checking if the output file is writable
## Code Example
Here's a snippet of the main functionality:
```rust
if let Ok(lines) = read_lines(input_path) {
    let mut results = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            if should_skip_line(&l) {
                continue;
            }
            if regex.is_match(&l) {
                // Apply custom transformation to the line before adding it to the results
                results.push(custom_transformation(l));
            }
        }
    }

    // Write results to output file
    let mut file = fs::File::create(output_path).unwrap();
    for result in results {
        writeln!(file, "{}", result).unwrap();
    }
}
```

## Dependencies
This tool uses the following external libraries:

clap: For parsing command-line arguments
regex: For processing text using regex patterns
Make sure to include these dependencies in your Cargo.toml file:
```
[dependencies]
clap = "2.33.3"
regex = "1.5.4"
```
## License
This project is licensed under the MIT License.
