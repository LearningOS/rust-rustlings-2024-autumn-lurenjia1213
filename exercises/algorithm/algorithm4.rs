/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match self.root{
            None=>{self.root=Some(Box::new(TreeNode::new(value)))}
            Some(ref mut root)=>{
                /*二叉搜索树？  
                若它的左子树不空，则左子树上所有结点的值均小于它的根结点的值； 
                若它的右子树不空，则右子树上所有结点的值均大于它的根结点的值；
                */
                if value>root.value{//大
                    if let Some(ref mut right)=root.right{//存在右侧树？插入，否则新建
                        right.insert(value);
                    }else{
                        root.right=Some(Box::new(TreeNode::new(value)));
                    }
                }else if value<root.value{
                    if let Some(ref mut left)=root.left{//存在左侧树？插入，否则新建
                        left.insert(value);
                    }else{
                        root.left=Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
        
        }


    }

    // Search for a value in the BST
    //fn search(&self, value: T) -> bool {
      //  true
    //}
    fn search(&self, value: T) -> bool {
        Self::tsearch(&self.root,value)
    }

    pub fn tsearch(node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            None => false,
            Some(ref root) => {
                match value.cmp(&root.value) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::tsearch(&root.left, value),
                    Ordering::Greater => Self::tsearch(&root.right, value),
                }
            }
        }
    }



}
impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value>self.value{//大
            if let Some(ref mut right)=self.right{//存在右侧树？插入，否则新建
                right.insert(value);
            }else{
                self.right=Some(Box::new(TreeNode::new(value)));
            }
        }else{
            if let Some(ref mut left)=self.left{//存在左侧树？插入，否则新建
                left.insert(value);
            }else{
                self.left=Some(Box::new(TreeNode::new(value)));
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());//
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


