use std::cmp::Ordering;

pub fn decode(huffman_code: &mut Vec<(char, String)>, input: String) {
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
    let file_len = input.len();
    println!("Original message:\n{}", input);
    println!("Decoded message:");
    while curr < file_len {
        if input[curr..=curr].cmp(" ") == Ordering::Equal {
            print!(" ");
            curr += 1;
            start = curr;
        } else if input[curr..=curr].cmp("\n") == Ordering::Equal {
            print!("\n");
            curr += 1;
            start = curr;
        } else {
            let test = String::from(&input[start..=curr]);
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
