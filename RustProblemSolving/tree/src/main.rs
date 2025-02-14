use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Rc::new(RefCell::new(node)));
        self
    }
    fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Rc::new(RefCell::new(node)));
        self
    }

    /// Insert a value into the tree.
    ///
    /// If the value already exists in the tree, do nothing.
    /// If the value is less than the current node, insert it into the left subtree.
    /// If the value is greater than the current node, insert it into the right subtree.
    /// If the value is equal to the current node, do nothing.
    ///
    /// # Examples
    ///
    /// 
    fn insert(&mut self, value: i32) {
        if self.value == value{
            return;
        }

        let node =  if value < self.value{ &mut self.left }else{
            &mut self.right
        };

        match node {
            Some(n) => n.borrow_mut().insert(value),
            None => *node = Some(Rc::new(RefCell::new(TreeNode {
                value: value,
                left: None,
                right: None,
            }))),
            
        }
        
        
    }
    fn traversal_inorder(&self,node:Option<Rc<RefCell<TreeNode>>>,res: &mut Vec<i32>){
        if let Some(n) = node{
            self.traversal_inorder(n.borrow().left.clone(), res);
            res.push(n.borrow().value);
            self.traversal_inorder(n.borrow().right.clone(), res);
        }
    }
    fn inorder(&self)->Vec<i32>{
        let tree = self.clone();
        let mut res = vec![];
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_inorder(node,&mut res);
        res
    }
    fn traversal_preorder(&self,node:Option<Rc<RefCell<TreeNode>>>,res: &mut Vec<i32>){
        if let Some(n) = node{
            res.push(n.borrow().value);
            self.traversal_preorder(n.borrow().left.clone(), res);
            self.traversal_preorder(n.borrow().right.clone(), res);
        }
    }
    fn preorder(&self)->Vec<i32>{
        let tree = self.clone();
        let mut res = vec![];
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_preorder(node,&mut res);
        res
    }
    fn traversal_postorder(&self,node:Option<Rc<RefCell<TreeNode>>>,res: &mut Vec<i32>){
        if let Some(n) = node{ 
            self.traversal_postorder(n.borrow().left.clone(), res);
            self.traversal_postorder(n.borrow().right.clone(), res);
            res.push(n.borrow().value);
        }
    }
    fn postorder(&self)->Vec<i32>{
        let tree = self.clone();
        let mut res = vec![];
        let node = Some(Rc::new(RefCell::new(tree)));
        self.traversal_postorder(node,&mut res);
        res
    }




}

fn manual_tree() -> TreeNode {
    let root = TreeNode::new(5)
        .left(
            TreeNode::new(3)
                .left(TreeNode::new(2).left(TreeNode::new(1)))
                .right(TreeNode::new(4)),
        )
        .right(
            TreeNode::new(7)
                .left(TreeNode::new(6))
                .right(TreeNode::new(8).right(TreeNode::new(9))),
        );
    root
}




fn main() {
    println!("TREEE");
    let man = manual_tree();
    println!("INORDER: {:?} \nPREORDER: {:?} \nPOSTORDER: {:?} \n", man.inorder(), man.preorder(), man.postorder());


    let mut tree = TreeNode::new(5);
    tree.insert(3);
    tree.insert(2);
    tree.insert(1);
    tree.insert(4);
    tree.insert(7);
    tree.insert(6);
    tree.insert(8);
    tree.insert(9);
    println!("INORDER: {:?} \nPREORDER: {:?} \nPOSTORDER: {:?} \n", tree.inorder(), tree.preorder(), tree.postorder());

}
