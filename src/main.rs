// Lesson 17: Advanced Data Structures

use std::rc::Rc;
use std::cell::RefCell;

// Define a Node struct for the linked list
#[derive(Debug)]
struct Node {
    value: i32, // The value stored in this node
    next: Option<Rc<RefCell<Node>>>, // The next node in the list
}

// Define a LinkedList struct to manage the list
#[derive(Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>, // The head (first node) of the list
}

impl LinkedList {
    // Create a new, empty linked list
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Add a value to the front of the list
    fn push_front(&mut self, value: i32) {
        // Create a new node with the given value and the current head as its next node
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        // Update the head to be the new node
        self.head = Some(new_node);
    }

    // Print the values in the list
    fn print(&self) {
        let mut current = self.head.clone(); // Start with the head of the list
        while let Some(node) = current { // Traverse the list
            print!("{} -> ", node.borrow().value); // Print the value of the current node
            current = node.borrow().next.clone(); // Move to the next node
        }
        println!("None"); // Indicate the end of the list
    }
}

// Define a TreeNode struct for the binary tree
#[derive(Debug)]
struct TreeNode {
    value: i32, // The value stored in this node
    left: Option<Box<TreeNode>>, // The left child node
    right: Option<Box<TreeNode>>, // The right child node
}

// Define a BinaryTree struct to manage the tree
#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<TreeNode>>, // The root (first node) of the tree
}

impl BinaryTree {
    // Create a new, empty binary tree
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // Insert a value into the binary tree
    fn insert(&mut self, value: i32) {
        // Insert the value starting from the root
        self.root = Self::insert_node(self.root.take(), value);
    }

    // Helper method to insert a value into a tree node
    fn insert_node(node: Option<Box<TreeNode>>, value: i32) -> Option<Box<TreeNode>> {
        match node {
            Some(mut n) => {
                if value < n.value {
                    // Recursively insert into the left subtree
                    n.left = Self::insert_node(n.left.take(), value);
                } else {
                    // Recursively insert into the right subtree
                    n.right = Self::insert_node(n.right.take(), value);
                }
                Some(n) // Return the modified node
            }
            None => Some(Box::new(TreeNode { // Create a new node if the current node is None
                value,
                left: None,
                right: None,
            })),
        }
    }

    // Print the values in the tree (in-order traversal)
    fn print_in_order(&self) {
        // Helper function for in-order traversal
        fn print_node(node: &Option<Box<TreeNode>>) {
            if let Some(n) = node {
                print_node(&n.left); // Visit the left subtree
                print!("{} ", n.value); // Print the node's value
                print_node(&n.right); // Visit the right subtree
            }
        }
        print_node(&self.root); // Start from the root
        println!(); // Newline at the end
    }
}

fn main() {
    // LinkedList example
    let mut list = LinkedList::new();
    list.push_front(3); // Add 3 to the front of the list
    list.push_front(2); // Add 2 to the front of the list
    list.push_front(1); // Add 1 to the front of the list

    println!("LinkedList:");
    list.print(); // Print the linked list

    // BinaryTree example
    let mut tree = BinaryTree::new();
    tree.insert(5); // Insert 5 into the tree
    tree.insert(3); // Insert 3 into the tree
    tree.insert(7); // Insert 7 into the tree
    tree.insert(2); // Insert 2 into the tree
    tree.insert(4); // Insert 4 into the tree
    tree.insert(6); // Insert 6 into the tree
    tree.insert(8); // Insert 8 into the tree

    println!("BinaryTree (in-order):");
    tree.print_in_order(); // Print the binary tree in order
}
