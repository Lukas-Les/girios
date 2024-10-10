use std::{collections::HashMap, sync::Arc};

use log::debug;
use tokio::sync::RwLock;

use common::dsa::char_tree::CharTree;

#[derive(Debug)]
pub struct Platform {
    pub data_structures: Arc<RwLock<DataStructures>>,
}

impl Platform {
    pub fn new() -> Self {
        debug!("Creating new platform");
        Platform {
            data_structures: Arc::new(RwLock::new(DataStructures::new())),
        }
    }
}

#[derive(Debug)]
pub struct DataStructures {
    ctrees: RwLock<HashMap<String, Arc<RwLock<CharTree>>>>, // Wrap each CharTree in Arc and RwLock
}

impl DataStructures {
    fn new() -> Self {
        DataStructures {
            ctrees: RwLock::new(HashMap::new()), // Initialize RwLock around the HashMap
        }
    }

    pub async fn insert_ctree(&self, tree: CharTree) {
        debug!("Inserting ctree: {}", tree.name);
        let mut ctrees = self.ctrees.write().await;
        ctrees.insert(tree.name.clone(), Arc::new(RwLock::new(tree))); // Insert the tree wrapped in Arc and RwLock
    }

    pub async fn remove_ctree(&self, name: &str) {
        debug!("Removing ctree: {}", name);
        let mut ctrees = self.ctrees.write().await; // Lock the HashMap for writing
        ctrees.remove(name);
    }

    // Method to get a reference to a ctree
    pub async fn get_ctree(&self, name: &str) -> Option<Arc<RwLock<CharTree>>> {
        debug!("Getting ctree: {}", name);
        let ctrees = self.ctrees.read().await; // Lock the HashMap for reading
        ctrees.get(name).cloned() // Clone the Arc to return a reference
    }

    pub async fn get_all_ctrees(&self) -> Vec<Arc<RwLock<CharTree>>> {
        let ctrees = self.ctrees.read().await;
        ctrees.values().cloned().collect()
    }
}
