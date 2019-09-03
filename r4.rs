#[test]
fn it_works() {
   let mut x: Tree<i32> = Tree::new();
   x.insert(-45);
   x.insert(3);
   x.insert(-45);
   x.insert(-90);
   x.insert(20);
   x.insert(2);
   print!("Tree:\n{:?}\n", x);
   print!("Test1: {:?}\n", x.find(&46));
   print!("Test2: {:?}\n", x.find(&20));
   print!("Test3: {:?}\n", x.find(&-45));
   print!("Test4: {:?}\n", x.insert(20));
   print!("Test5: {:?}\n", x.preorder());
   print!("Test6: {:?}\n", x.inorder());
   print!("Test7: {:?}\n", x.postorder());
}

#[derive(Debug)]
struct Node<T> {
   key: T,
   left: Option<Box<Tree<T>>>,
   right: Option<Box<Tree<T>>>,
}

#[derive(Debug)]
pub struct Tree<T> {
   node: Option<Box<Node<T>>>,
}

impl<T: Ord> Tree<T> {
   /// Creates an empty tree
   pub fn new() -> Self {
      Tree{node: Option::None}
   }

   /// Returns `false` if `key` already exists in the tree, 
   //and `true` otherwise.
   pub fn insert(&mut self, key: T) -> bool {
      match self.node {
         Some(ref mut node_ref) => { //If the tree has a node, pass it to "node_ref"
            if node_ref.key == key {return false}
            else if key < node_ref.key { //if the key is less than the node, check for left leaf
               match node_ref.left {
                  Some(ref mut child_tree) => {
                     return child_tree.insert(key);
                  },
                  None => {
                     let mut new_node: Tree<T> = Tree::new();
                     new_node.insert(key);
                     node_ref.left = Some(Box::new(new_node));
                     return true;
                  },
               }
            }
            else if key > node_ref.key { //if the key is greater than the node, check for right leaf
               match node_ref.right {
                  Some(ref mut child_tree) => {
                     return child_tree.insert(key);
                  },
                  None => {
                     let mut new_node: Tree<T> = Tree::new();
                     new_node.insert(key);
                     node_ref.right = Some(Box::new(new_node));
                     return true;
                  },
               }
            }
            panic!("Should not reach this point! (1)");
         },
         None => { //If the tree is completely empty, insert the first value
            self.node = Some(Box::new(Node{key: key, left: None, right: None}));
            return true;
         },
      }
   }

   /// Returns `true` if `key` exists in the tree, 
   //and `false` otherwise.
   pub fn find(&self, key: &T) -> bool {
      match self.node {
         Some(ref node_ref) => {
            if key == &node_ref.key {
               return true;
            }
            else if key < &node_ref.key { //if less than the node, check left leaf using recursion
               match node_ref.left {
                  Some(ref left_leaf) => {
                     return left_leaf.find(key);
                  },
                  None => {
                     return false;
                  },
               }
            }
            else if key > &node_ref.key { //if greater than the node, check right leaf using recursion
               match node_ref.right {
                  Some(ref right_leaf) => {
                     return right_leaf.find(key);
                  },
                  None => {
                     return false;
                  },
               }
            }
            panic!("Should not reach this point!(2)");
         },
         None => false, //return false immediately if the tree is empty
      }
   }

   /// Returns the preorder traversal of the tree.
   pub fn preorder(&self) -> Vec<&T> {
      let mut result: Vec<&T> = vec![];
      match self.node {
         Some(ref curr_node) => {
            result.push(&curr_node.key);
            match curr_node.left {
               Some(ref left_leaf) => {
                  result.append(&mut left_leaf.preorder());
               },
               None => {
                  ;
               },
            }
            match curr_node.right {
               Some(ref right_leaf) => {
                  result.append(&mut right_leaf.preorder());
               },
               None => {
                  ;
               },
            }
         },
         None => {
            ;
         },
      };
      return result;
   }

   /// Returns the inorder traversal of the tree.
   pub fn inorder(&self) -> Vec<&T> {
      let mut result: Vec<&T> = vec![];
      match self.node {
         Some(ref curr_node) => {
            match curr_node.left {
               Some(ref left_leaf) => {
                  result.append(&mut left_leaf.inorder());
               },
               None => {
                  ;
               },
            }
            result.push(&curr_node.key);
            match curr_node.right {
               Some(ref right_leaf) => {
                  result.append(&mut right_leaf.inorder());
               },
               None => {
                  ;
               },
            }
         },
         None => {
            ;
         },
      };
      return result;
   }

   /// Returns the postorder traversal of the tree.
   pub fn postorder(&self) -> Vec<&T> {
      let mut result: Vec<&T> = vec![];
      match self.node {
         Some(ref curr_node) => {
            match curr_node.left {
               Some(ref left_leaf) => {
                  result.append(&mut left_leaf.postorder());
               },
               None => {
                  ;
               },
            }
            match curr_node.right {
               Some(ref right_leaf) => {
                  result.append(&mut right_leaf.postorder());
               },
               None => {
                  ;
               },
            }
            result.push(&curr_node.key);
         },
         None => {
            ;
         },
      };
      return result;
   }
}