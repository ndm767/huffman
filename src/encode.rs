pub fn encode(huffman_code: &Vec<(char, String)>, input: String) {
    // create lookup table
    let mut max_symb: usize = 0;
    let mut min_symb: usize = huffman_code[0].0 as usize;
    for i in 0..huffman_code.len() {
        if huffman_code[i].0 as usize > max_symb {
            max_symb = huffman_code[i].0 as usize;
        }
        if (huffman_code[i].0 as usize) < min_symb {
            min_symb = huffman_code[i].0 as usize;
        }
    }
    let mut lookup: Vec<String> = Vec::new();
    for _i in 0..(max_symb - min_symb + 2) {
        lookup.push(String::from(""));
    }
    for i in 0..huffman_code.len() {
        lookup[huffman_code[i].0 as usize - min_symb] = huffman_code[i].1.clone();
    }

    println!("Original message:\n{}", input);
    println!("Encoded message:");
    for c in input.chars() {
        // hack for control characters
        if !(c == '\n' || c == ' ') {
            print!("{}", lookup[c as usize - min_symb]);
        } else {
            print!("{}", c);
        }
    }
    println!("");
}
