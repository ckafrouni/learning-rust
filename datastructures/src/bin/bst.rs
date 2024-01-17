use datastructures::binary_search_tree::BinarySearchTree as BST;

fn main() {
    let mut bst = BST::<u32>::new();

    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(2);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    println!("{:#?}", bst);
}
