use std::{collections::HashMap, sync::{Arc, RwLock}};

use common::dsa::char_tree::CharTree;


pub struct Platform {
    data_structures: DataStructures,
}

impl Platform {
    pub fn new() -> Self {
        Platform { data_structures: DataStructures::new()}
    }
}



pub struct DataStructures {
    ctrees: HashMap<String, Arc<RwLock<CharTree>>>
}

impl DataStructures {
    fn new() -> Self {
        DataStructures {
            ctrees: HashMap::new(),
        }
    }

    pub fn insert_ctree(&mut self, tree: CharTree) {
        self.ctrees.insert(tree.name.clone(), Arc::new(RwLock::new(tree)));
    }

    pub fn remove_ctree(&mut self, name: String) {
        self.ctrees.remove(&name);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::RwLock;
    use common::dsa::char_tree::CharTree;


    #[test]
    fn test_insert_ctree() {
        let mut data_structures = DataStructures::new();
        let tree_name = "tree1".to_string();

        // Create a CharTree
        let ctree = CharTree::new(tree_name.clone());

        // Insert CharTree into DataStructures
        data_structures.insert_ctree(ctree);

        // Ensure the tree is inserted and exists
        assert!(data_structures.ctrees.contains_key(&tree_name));
    }

    #[test]
    fn test_remove_ctree() {
        let mut data_structures = DataStructures::new();
        let tree_name = "tree1".to_string();

        // Create and insert a CharTree
        let ctree = CharTree::new(tree_name.clone());
        data_structures.insert_ctree(ctree);

        // Ensure the tree is inserted
        assert!(data_structures.ctrees.contains_key(&tree_name));

        // Now remove it
        data_structures.remove_ctree(tree_name.clone());

        // Ensure the tree is removed
        assert!(!data_structures.ctrees.contains_key(&tree_name));
    }

    #[test]
    fn test_platform_creation() {
        // Ensure the Platform and DataStructures can be created
        let platform = Platform::new();

        // Check that DataStructures are initialized
        assert!(platform.data_structures.ctrees.is_empty());
    }

    #[test]
    fn test_insert_and_read_ctree_with_lock() {
        let mut data_structures = DataStructures::new();
        let tree_name = "tree_with_lock".to_string();

        // Create and insert a CharTree
        let ctree = CharTree::new(tree_name.clone());
        data_structures.insert_ctree(ctree);

        // Read the CharTree using the RwLock
        if let Some(arc_rwlock_tree) = data_structures.ctrees.get(&tree_name) {
            let tree_read_lock = arc_rwlock_tree.read().unwrap();
            assert_eq!(tree_read_lock.name, tree_name);
        } else {
            panic!("CharTree not found!");
        }
    }
}
