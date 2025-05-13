#[derive(Debug)]
pub struct Tree<T> {
    root : Option<Box<Node<T>>>
}

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl <T: Ord> Node<T> {
    fn new(elem: T) -> Self {
        Node {elem: elem, left: None, right: None}
    }

    fn add(&mut self, elem: T) -> () {
        if elem < self.elem {
            match self.left {
                Option::None => self.left = Some(Box::new(Node::new(elem))),
                Option::Some(ref mut left_node) => left_node.add(elem),
            }
        } 
        else if elem > self.elem {
            match self.right {
                Option::None => self.right = Some(Box::new(Node::new(elem))),
                Option::Some(ref mut right_node) => right_node.add(elem), 
            }
        }
        // do nothing if elem is already on the tree
    }

    fn remove_and_return_greatest(node: &mut Box<Node<T>>) -> Option<T> {
        match node.right {
            //caso em que a raiz é o maior, mas possui filho esquerdo
            Option::None => {
                let left_subtree = node.left.take();
                if let Some(left_node) = left_subtree {
                    let value = std::mem::replace(&mut node.elem, left_node.elem);
                    node.left = left_node.left;
                    node.right = left_node.right;
                    Some(value)
                }
                else {
                    None
                }

            }
            //o filho direito existe
            Option::Some(ref mut right_child) => {
                //se o filho direito do filho direito não existe, então o filho atual é o maior
                if right_child.right.is_none() {
                    //salva o maior nó
                    let max_node = node.right.take().unwrap();
                    //realoca o filho esquesdo para substituir o pai
                    node.right = max_node.left;
                    
                    Some(max_node.elem)
                }
                else {
                    Node::remove_and_return_greatest(right_child)
                }
            }
        }
    }

}

impl <T: Ord> Tree<T> {
    fn new() -> Self {
        Tree { root: None }
    }

    fn new_with_elem(elem: T) -> Self {
        Tree {root: Some(Box::new(Node::new(elem)))}
    }

    fn add(&mut self, elem: T) -> () {
        match self.root {
            Option::None => self.root = Some(Box::new(Node::new(elem))),
            Option::Some(ref mut node) => node.add(elem),

        }
    }

    fn remove_and_return_greatest(&mut self) -> Option<T> {
        match self.root.as_mut() {
            Option::None => {
                println!("Empty tree, no elements to remove or show");
                Option::None
            }
            Option::Some(node) => {
                let is_leaf = node.left.is_none() && node.right.is_none();

                if is_leaf {
                    let node = self.root.take().unwrap();
                    return Some(node.elem)
                }
                else {
                    Node::remove_and_return_greatest(node)
                }

            }
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.add(2);
    // tree.add(6);
    // tree.add(3);
    // tree.add(2);
    // tree.add(5);
    // tree.add(9);
    // tree.add(12);
    // tree.add(1);
    let greatest_elem = tree.remove_and_return_greatest();
    println!("Maior elemento: {:?}", greatest_elem);
    println!("{:?}", tree);
}
