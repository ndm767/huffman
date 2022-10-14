extern crate csv;

pub mod binary_tree;

use binary_tree::BinaryNode;
use std::cmp::Ordering;
use std::env;
use std::fs;

fn vec_insert_val(v: &mut Vec<BinaryNode>, val: BinaryNode) {
    if v.len() == 0 {
        v.push(val);
        return;
    }
    for i in 0..v.len() {
        if v[i].get_prob() > val.get_prob() {
            v.insert(i, val);
            return;
        }
    }
    v.push(val);
}

fn main() {
    let mut argv = env::args();
    if argv.len() <= 3 {
        println!("Not enough arguments!");
        return;
    }

    let csv_path = argv.nth(1).unwrap();
    let encode = argv.nth(0).unwrap().contains("encode");
    let in_file = fs::read_to_string(argv.nth(0).unwrap()).unwrap();

    let mut alphabet: Vec<(char, f32)> = vec![];
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(csv_path)
        .unwrap();
    for res in rdr.records() {
        let rec = res.unwrap();
        let symbol = rec.get(0).unwrap().chars().nth(0).unwrap();
        let prob = rec.get(1).unwrap().trim().parse::<f32>().unwrap();
        alphabet.push((symbol, prob));
    }

    alphabet.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut tree: Vec<BinaryNode> = vec![];
    for a in alphabet.iter() {
        tree.push(BinaryNode::new(a.1, Some(a.0), None, None));
    }

    while tree.len() > 1 {
        let n1 = tree[0].clone();
        let n2 = tree[1].clone();
        tree.remove(0);
        tree.remove(0);
        vec_insert_val(
            &mut tree,
            BinaryNode::new(
                n1.get_prob() + n2.get_prob(),
                None,
                Some(Box::new(n1)),
                Some(Box::new(n2)),
            ),
        );
    }

    println!("Alphabet: {:?}", alphabet);
    let mut huffman_code = tree[0].get_huffman_code();
    println!("Code: {:?}", huffman_code);

    if encode {
        // get create lookup table
        let mut max_symb: usize = 0;
        for i in 0..huffman_code.len() {
            if huffman_code[i].0 as usize > max_symb {
                max_symb = huffman_code[i].0 as usize;
            }
        }
        let mut lookup: Vec<String> = Vec::new();
        for _i in 0..(max_symb as usize + 2) {
            lookup.push(String::from(""));
        }
        for i in 0..huffman_code.len() {
            lookup[huffman_code[i].0 as usize] = huffman_code[i].1.clone();
        }

        println!("Original message:\n{}", in_file);
        println!("Encoded message:");
        for c in in_file.chars() {
            // hack for control characters
            if !(c == '\n' || c == ' ') {
                print!("{}", lookup[c as usize]);
            } else {
                print!("{}", c);
            }
        }
        println!("");
    } else {
        huffman_code.sort_by(|a, b| {
            if a.1.len() < b.1.len() {
                return Ordering::Less;
            } else if a.1.len() > b.1.len() {
                return Ordering::Greater;
            }
            a.1.trim()
                .parse::<u64>()
                .unwrap()
                .cmp(&b.1.trim().parse::<u64>().unwrap())
        });
        // create lookup table based upon string length
        let mut matches: Vec<Vec<(String, char)>> = Vec::new();
        for i in huffman_code.iter() {
            while matches.len() <= i.1.len() {
                matches.push(Vec::new())
            }

            matches[i.1.len()].push((i.1.clone(), i.0));
        }

        let mut start: usize = 0;
        let mut curr: usize = 0;
        let file_len = in_file.len();
        println!("Original message:\n{}", in_file);
        println!("Decoded message:");
        while curr < file_len {
            if in_file[curr..=curr].cmp(" ") == Ordering::Equal {
                print!(" ");
                curr += 1;
                start = curr;
            } else if in_file[curr..=curr].cmp("\n") == Ordering::Equal {
                print!("\n");
                curr += 1;
                start = curr;
            } else {
                let test = String::from(&in_file[start..=curr]);
                if test.len() > matches.len() {
                    panic!("No match found for {}!", test);
                }

                for t in matches[test.len()].iter() {
                    if t.0.cmp(&test) == Ordering::Equal {
                        print!("{}", t.1);
                        start = curr + 1;
                        break;
                    }
                }
                curr += 1;
            }
        }
        println!();
    }

    println!("Alphabet: {:?}", alphabet);
    println!("Code: {:?}", huffman_code);
}
