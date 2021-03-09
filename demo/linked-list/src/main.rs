use std::collections::VecDeque;

fn main() {
    let val = 9;
    let mut tree = BinaryTree::new(val);
    tree.insert(11);
    tree.insert(8);
    tree.insert(13);
    tree.insert(15);
    tree.insert(10);
    tree.insert(6);
    tree.insert(3);
    tree.insert(1);
    tree.insert(2);
    tree.insert(0);
    tree.insert(7);
    println!("{:#?}", tree);
    println!("{}", tree.max_len());
}

type TreeNode<T> = Option<Box<Node<T>>>;
type Tepth = u32;

#[derive(Debug)]
struct Node<T: PartialOrd + Copy> {
    val: T,
    left: TreeNode<T>,
    right: TreeNode<T>,
}

impl<T: PartialOrd + Copy> Node<T> {
    #[inline]
    pub fn has_child(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }
}

#[derive(Debug)]
struct BinaryTree<T: PartialOrd + Copy> {
    root: TreeNode<T>,
}

impl<T> BinaryTree<T>
    where
        T: PartialOrd + Copy,
{
    #[inline]
    pub fn new(val: T) -> BinaryTree<T> {
        BinaryTree {
            root: Some(Box::new(Node {
                val,
                left: None,
                right: None,
            })),
        }
    }

    pub fn insert(&mut self, val: T) {
        let mut v = VecDeque::with_capacity(1);
        v.push_back(self.root.as_mut().unwrap());
        while !v.is_empty() {
            let node = v.pop_front().unwrap();
            if val <= node.val {
                generate_child_node(&mut node.left, val, &mut v);
            } else {
                generate_child_node(&mut node.right, val, &mut v);
            }
        }
    }

    pub fn max_len(&self) -> Tepth {
        if !self.root.as_ref().unwrap().has_child() {
            return 1;
        }
        let mut len = 1;
        let mut v = VecDeque::with_capacity(2);
        v.push_back(self.root.as_ref());
        while !v.is_empty() {
            let l = v.len();
            for _ in 0..l {
                let node = &**v.pop_front().unwrap().unwrap();
                if let Some(left) = &node.left {
                    if left.has_child() {
                        v.push_back(Some(left));
                    }
                }
                if let Some(right) = &node.right {
                    if right.has_child() {
                        v.push_back(Some(right));
                    }
                }
            }
            len += 1;
        }
        len
    }
}

fn generate_child_node<'a, T: Copy + PartialOrd>(
    node: &'a mut Option<Box<Node<T>>>,
    val: T,
    v: &mut VecDeque<&'a mut Box<Node<T>>>,
) {
    if node.is_some() {
        v.push_back(node.as_mut().unwrap());
    } else {
        *node = Some(Box::new(Node {
            val,
            left: None,
            right: None,
        }))
    }
}
