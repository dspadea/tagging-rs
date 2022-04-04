use std::collections::{HashMap, HashSet};

use crate::storage::IndexStorage;

pub struct MemoryTagIndex {
    by_key: HashMap<String, HashSet<String>>, // doc key, set of tags
    by_tag: HashMap<String, HashSet<String>> // tag key, set of docs
}

impl MemoryTagIndex {
    pub fn new() -> MemoryTagIndex {
        MemoryTagIndex { 
            by_key: HashMap::new(), 
            by_tag: HashMap::new() 
        }
    }
}

impl Default for MemoryTagIndex {
    fn default() -> Self {
        MemoryTagIndex::new()
    }
}

impl IndexStorage for MemoryTagIndex {
    fn start(&mut self) {
        
    }

    fn shutdown(&mut self) {
        
    }

    fn tag_item(&mut self, key: &str, tag: &str) {
        
        if ! self.by_key.contains_key(key) {
            let mut new_set = HashSet::new();
            new_set.insert(tag.to_string());
            self.by_key.insert(key.to_string(), new_set);

            let mut new_set = HashSet::new();
            new_set.insert(key.to_string());
            self.by_tag.insert(tag.to_string(), new_set);
        } else {
            let set = self.by_key.get_mut(key).unwrap();
            set.insert(tag.to_string());

            let set = self.by_tag.get_mut(tag).unwrap();
            set.insert(key.to_string());
        }

    }

    fn retrieve_tags_for_item(&self, key: &str) -> Vec<String> {

        if let Some(set) = self.by_key.get(key) {
            set.iter()
            .map(|i| i.to_string())
            .collect()
        } else {
            vec![]
        }
 
    }

    fn retrieve_items_with_tag(&self, tag: &str) -> Vec<String> {
        if let Some(set) = self.by_tag.get(tag) {
            set.iter()
            .map(|i| i.to_string())
            .collect()
        } else {
            vec![]
        }    
    }

    fn untag_item(&mut self, key: &str, tag: &str) {

        if self.by_key.contains_key(key) {
            let set = self.by_key.get_mut(key).unwrap();
            set.remove(tag);

            let set = self.by_tag.get_mut(tag).unwrap();
            set.remove(key);
        }

    }

    fn tag_exists(&self, tag: &str) -> bool {
        self.by_tag.contains_key(tag)
    }
}

#[cfg(test)]
mod test {
    use crate::storage::IndexStorage;

    use super::MemoryTagIndex;


    #[test]
    fn test_add_tag() {
        let mut idx = MemoryTagIndex::new();
        
        idx.start();
        idx.tag_item("my_key", "my_tag");

        assert!(idx.tag_exists("my_tag"));
        assert!(idx.retrieve_items_with_tag("my_tag").contains(&"my_key".to_string()));
        assert!(idx.retrieve_tags_for_item("my_key").contains(&"my_tag".to_string()));
    }

    #[test]
    fn test_remove_tag() {
        let mut idx = MemoryTagIndex::new();
        
        idx.start();
        idx.tag_item("my_key", "my_tag");

        assert!(idx.tag_exists("my_tag"));
        assert!(idx.retrieve_items_with_tag("my_tag").contains(&"my_key".to_string()));
        assert!(idx.retrieve_tags_for_item("my_key").contains(&"my_tag".to_string()));

        idx.untag_item("my_key", "my_tag");
        assert!(idx.tag_exists("my_tag"));
        assert!( !idx.retrieve_items_with_tag("my_tag").contains(&"my_key".to_string()) );
        assert!( !idx.retrieve_tags_for_item("my_key").contains(&"my_tag".to_string()) );

    }

    #[test]
    fn test_remove_nonexistent_tag() {
        let mut idx = MemoryTagIndex::new();
        
        idx.start();
       
        idx.untag_item("my_key", "my_tag");
        assert!( !idx.tag_exists("my_tag")  );
        assert!( !idx.retrieve_items_with_tag("my_tag").contains(&"my_key".to_string()) );
        assert!( !idx.retrieve_tags_for_item("my_key").contains(&"my_tag".to_string()) );

    }


}