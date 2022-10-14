extern crate csv;

mod binary_tree;
mod decode;
mod encode;

use binary_tree::BinaryNode;
use decode::decode;
use encode::encode;

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
    let should_encode = argv.nth(0).unwrap().contains("encode");
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

    if should_encode {
        encode(&huffman_code, in_file);
    } else {
        decode(&mut huffman_code, in_file);
    }

    println!("Alphabet: {:?}", alphabet);
    println!("Code: {:?}", huffman_code);
}
