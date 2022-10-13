extern crate csv;

pub mod binary_tree;

use binary_tree::BinaryNode;
use std::env;

fn main() {
    let mut argv = env::args();
    if argv.len() <= 1 {
        println!("No CSV file provided!");
        return;
    }

    let mut alphabet: Vec<(char, f32)> = vec![];
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(argv.nth(1).unwrap())
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
        let mut i = 0usize;
        let start_len = tree.len();
        let mut new_tree: Vec<BinaryNode> = vec![];
        while i < start_len {
            println!("{} {} {}", i, start_len, tree.len());
            if i < start_len - 1 {
                let l = &tree[i];
                let r = &tree[i + 1];
                new_tree.push(BinaryNode::new(
                    l.get_prob() + r.get_prob(),
                    None,
                    Some(Box::new(l.clone())),
                    Some(Box::new(r.clone())),
                ))
            } else {
                new_tree.push(tree[i].clone());
            }
            i += 2;
        }

        tree = new_tree.clone();
    }

    println!("Alphabet {:?}", alphabet);
    println!("Code {:?}", tree[0].get_huffman_code());
}
