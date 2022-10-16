# Huffman

Huffman coding in Rust for my Information and Coding Theory Independent Study.

Example output: [bookprob_output.md](bookprob_output.md)

Usage:

```sh
cargo run [--release] <alphabet csv> {encode|decode} <filename|string> [radix] 
```  

The CSV file should be formatted with comma separators and without headers.  
The radix argument is optional (default 2) and must be greater than or equal to 2.  
The `--release` flag is optional but will speed up the program signficantly due to compiler optimizations (Shakespeare decode 10.12s > 0.59s).  

The Shakespeare has been processed to remove any non-letter characters (with the exception of spaces and newlines).  

## Attributions

- Shakespeare Anthology from Project Gutenberg.
- English frequency list from Wikipedia.
- book_probs.csv from Introduction to Coding and Information Theory by Steven Roman (1997)
- Independent study advisor: [Prof. Jeff Miller](http://jeffmiller.oxycreates.org/)
- Moral support: [Noah Willis](https://github.com/knoahwillis/)
