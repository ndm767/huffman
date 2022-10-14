extern crate csv;

pub mod binary_tree;

use binary_tree::BinaryNode;
use std::env;

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
    if argv.len() <= 1 {
        println!("Not enough arguments!");
        return;
    }

    //let encode = argv.nth(2).unwrap().contains("encode");

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

    println!("Alphabet {:?}", alphabet);
    println!("Code {:?}", tree[0].get_huffman_code());
    // TODO encode + decode
}
