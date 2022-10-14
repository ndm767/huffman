# Huffman

Huffman encoding in Rust

Usage:

```sh
cargo run [--release] [alphabet csv] [encode/decode] [file] 
```

The Shakespeare has been processed to remove any non-letter characters.  
The `--release` flag is optional but will speed up the program signficantly due to compiler optimizations (Shakespeare decode 10.12s > 0.59s).
