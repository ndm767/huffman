extern crate csv;

mod decode;
mod encode;
mod tree;

use decode::decode;
use encode::encode;
use tree::TreeNode;

use std::cmp::Ordering;
use std::env;
use std::fs;
use std::path::Path;

fn vec_insert_val(v: &mut Vec<TreeNode>, val: TreeNode) {
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
    let in_str = argv.nth(0).unwrap();
    let input = if Path::exists(Path::new(&in_str)) {
        fs::read_to_string(in_str).unwrap()
    } else {
        in_str
    };
    let radix: usize = if argv.len() != 0 {
        argv.nth(0).unwrap().parse::<usize>().unwrap()
    } else {
        2
    };

    if radix < 2 {
        panic!("Radix must be greater than or equal to 2!");
    }

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

    let mut tree: Vec<TreeNode> = vec![];
    for a in alphabet.iter() {
        tree.push(TreeNode::new(a.1, Some(a.0), radix, vec![None; radix]));
    }

    while tree.len() > 1 {
        let size_lim = if tree.len() < radix {
            tree.len()
        } else {
            radix
        };
        let mut comb_vec: Vec<Option<Box<TreeNode>>> = vec![];
        let mut prob_sum = 0f32;
        for _i in 0..size_lim {
            prob_sum += tree[0].get_prob();
            comb_vec.push(Some(Box::new(tree[0].clone())));
            tree.remove(0);
        }
        for _i in size_lim..radix {
            comb_vec.push(None);
        }
        vec_insert_val(&mut tree, TreeNode::new(prob_sum, None, radix, comb_vec));
    }

    println!("Alphabet: {:?}", alphabet);
    let mut huffman_code = tree[0].get_huffman_code();
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
    println!("Code: {:?}", huffman_code);

    if should_encode {
        encode(&huffman_code, input);
    } else {
        decode(&huffman_code, input);
    }

    println!("Alphabet: {:?}", alphabet);
    println!("Code: {:?}", huffman_code);
}
