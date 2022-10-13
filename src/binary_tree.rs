#[derive(Clone)]
pub struct BinaryNode {
    symbol: Option<char>,
    prob: f32,
    left: Option<Box<BinaryNode>>,
    right: Option<Box<BinaryNode>>,
}

impl BinaryNode {
    pub fn new(
        prob: f32,
        symbol: Option<char>,
        left: Option<Box<BinaryNode>>,
        right: Option<Box<BinaryNode>>,
    ) -> BinaryNode {
        BinaryNode {
            symbol,
            prob,
            left,
            right,
        }
    }

    pub fn get_symbol(&self) -> Option<char> {
        self.symbol
    }
    pub fn get_prob(&self) -> f32 {
        self.prob
    }

    pub fn get_huffman_code(&self) -> Vec<(char, String)> {
        let mut ret: Vec<(char, String)> = vec![];
        let has_children = self.right.is_some() && self.left.is_some();
        if has_children {
            let mut right_ret = self.right.as_ref().unwrap().get_huffman_code();
            for r in right_ret.iter_mut() {
                r.1 = format!("1{}", r.1);
            }
            ret.append(&mut right_ret);

            let mut left_ret = self.left.as_ref().unwrap().get_huffman_code();
            for l in left_ret.iter_mut() {
                l.1 = format!("0{}", l.1);
            }
            ret.append(&mut left_ret);
        } else {
            match &self.symbol {
                Some(c) => ret.push((*c, String::from(""))),
                None => panic!("Node has no children or symbol!"),
            }
        }

        return ret;
    }
}
