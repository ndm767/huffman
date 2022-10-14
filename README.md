# Huffman

Huffman encoding in Rust

Usage:

```sh
cargo run [--release] [alphabet csv] [encode/decode] [file] 
```

The Shakespeare has been processed to remove any non-letter characters.  
The `--release` flag is optional but will spead up the program signficantly because of compiler optimizations (Shakespeare decode 10.12s > 0.59s )
