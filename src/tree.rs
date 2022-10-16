use std::fmt;

#[derive(Clone)]
pub struct TreeNode {
    symbol: Option<char>,
    prob: f32,
    children: Vec<Option<Box<TreeNode>>>,
}

impl TreeNode {
    pub fn new(
        prob: f32,
        symbol: Option<char>,
        radix: usize,
        children: Vec<Option<Box<TreeNode>>>,
    ) -> TreeNode {
        assert_eq!(
            radix,
            children.len(),
            "Radix doesn't equal number of child nodes!"
        );
        TreeNode {
            symbol,
            prob,
            children,
        }
    }

    pub fn get_prob(&self) -> f32 {
        self.prob
    }

    pub fn get_descendants(&self) -> String {
        match self.symbol {
            Some(c) => String::from(c),
            None => {
                let mut ret: Vec<String> = Vec::new();
                for i in self.children.iter() {
                    match i {
                        Some(n) => ret.push(n.get_descendants()),
                        None => {}
                    }
                }
                ret.join("")
            }
        }
    }

    pub fn get_huffman_code(&self) -> Vec<(char, String)> {
        let mut ret: Vec<(char, String)> = vec![];
        let mut has_children = false;
        for i in &self.children {
            if i.is_some() {
                has_children = true;
                break;
            }
        }

        if has_children {
            for i in self.children.iter().enumerate() {
                let idx = i.0;
                match i.1 {
                    Some(n) => {
                        let mut temp_ret = n.get_huffman_code();
                        for j in temp_ret.iter_mut() {
                            j.1 = format!("{}{}", idx, j.1);
                        }
                        ret.append(&mut temp_ret);
                    }
                    None => {}
                }
            }
        } else {
            match &self.symbol {
                Some(c) => ret.push((*c, String::from(""))),
                None => panic!("Node has no children or symbol!"),
            }
        }

        return ret;
    }
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeNode")
            .field("Prob", &self.prob)
            .field("Descendants", &self.get_descendants())
            .finish()
    }
}
