#[test]
fn it_works() {
}

struct Node<T> {
   key: T,
   left: Option<Box<Tree<T>>>,
   right: Option<Box<Tree<T>>>,
}

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
    pub fn preorder(&self) -> IterPreorder<T> {
        let mut result: IterPreorder<T> = IterPreorder::new();
        match self.node {
            Some(ref curr_node) => {
                result.curr_key = Some(Box::new(curr_node.key));
                result.next_node = match curr_node.left {
                    Some(ref tree) => {
                        match tree.node {
                            Some(ref node) => Some(node),
                            None => panic!(),
                        }
                    },
                    None => {
                        match curr_node.right{
                            Some(ref tree) => {
                                match tree.node {
                                    Some(ref node) => Some(node),
                                    None => panic!(),
                                }
                            },
                            None => None,
                        }
                    },
                };
                match curr_node.right {
                    Some(ref tree) => result.remaining_siblings.push(tree),
                    None => {;},
                };
            },
            None => {
               ;
            },
        };
      return result;
   }
} //THIS BRACKET CUTS OFF OTHER FUNCTIONS

impl<T> Tree<T> {
    fn node_ref(tree: &Box<Tree<T>>) -> &Box<Node<T>> {
        match tree.node {
            Some(ref node) => {
                node
            },
            None => panic!(),
        }
    }
}

//    /// Returns the inorder traversal of the tree.
//    pub fn inorder(&self) -> IterInorder<T> {
//       let mut result: Vec<&T> = vec![];
//       match self.node {
//          Some(ref curr_node) => {
//             match curr_node.left {
//                Some(ref left_leaf) => {
//                   result.append(&mut left_leaf.inorder());
//                },
//                None => {
//                   ;
//                },
//             }
//             result.push(&curr_node.key);
//             match curr_node.right {
//                Some(ref right_leaf) => {
//                   result.append(&mut right_leaf.inorder());
//                },
//                None => {
//                   ;
//                },
//             }
//          },
//          None => {
//             ;
//          },
//       };
//       return result;
//    }

//    /// Returns the postorder traversal of the tree.
//    pub fn postorder(&self) -> IterPostorder<T> {
//       let mut result: Vec<&T> = vec![];
//       match self.node {
//          Some(ref curr_node) => {
//             match curr_node.left {
//                Some(ref left_leaf) => {
//                   result.append(&mut left_leaf.postorder());
//                },
//                None => {
//                   ;
//                },
//             }
//             match curr_node.right {
//                Some(ref right_leaf) => {
//                   result.append(&mut right_leaf.postorder());
//                },
//                None => {
//                   ;
//                },
//             }
//             result.push(&curr_node.key);
//          },
//          None => {
//             ;
//          },
//       };
//       return result;
//    }

//define a struct that holds a tree and its two sub treess
pub struct IterPreorder<'a, T: 'a> {
    curr_key: Option<Box<T>>,
    next_node: Option<&'a Node<T>>,
    remaining_siblings: Vec<&'a Box<Tree<T>>>,
}


impl<'a, T> IterPreorder<'a, T> {
    fn new() -> Self {
        IterPreorder{
            curr_key: None, 
            next_node: None,
            remaining_siblings: vec![],
        }
    }

}

impl<'a, T> Iterator for IterPreorder<'a, T> {
    type Item = Box<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_key { //If no key is left, next returns none and iterator breaks
            Some(key) => {
                match self.next_node {
                    Some(next) => { //when there is a next node, 
                        //extract its key and children to set up next 
                        //iterator
                        self.curr_key = Some(Box::new(next.key));
                        let node_left: Option<&Node<T>> = match next.left { //taking the node out of the tree in left
                            Some(ref tree) => Some(Tree::node_ref(tree)),
                            None => None,
                        };
                        match next.right {
                             Some(ref tree) => {
                                self.remaining_siblings.push(tree);
                             },
                             None => {
                                ;
                             },
                         };
                        match node_left {
                            Some(new_node) => {//new_node is a tree
                                self.next_node = Some(new_node);
                            },
                            None => {
                                let sibling = self.remaining_siblings.pop().unwrap();
                                self.next_node = Some(Tree::node_ref(sibling));
                            },
                        };
                    },
                    None => {
                        self.curr_key = None;
                    },
                };
                Some(key)
            },
            None => None,
        }
    }
}

// pub struct IterInorder<'a, T: 'a> {
//     //unimplemented!();
// }

// impl<'a, T> IterInorder<'a, T> {
//     //unimplemented!();
// }

// impl<'a, T> Iterator for IterInorder<'a, T> {
//     //unimplemented!();
// }

// pub struct IterPostorder<'a, T: 'a> {
//     //unimplemented!();
// }

// impl<'a, T> IterPostorder<'a, T> {
//     //unimplemented!();
// }

// impl<'a, T> Iterator for IterPostorder<'a, T> {
//     //unimplemented!();
// }