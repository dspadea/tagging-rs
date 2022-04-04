
#[cfg(feature = "memory_backing")]
pub mod memory;

#[cfg(feature = "redis_backing")]
pub mod redis;

trait IndexStorage {
    fn start(&mut self);
    fn shutdown(&mut self);
    fn tag_item(&mut self, key: &str, tag: &str);
    fn retrieve_tags_for_item(&self, key: &str) -> Vec<String>;
    fn retrieve_items_with_tag(&self, tag: &str) -> Vec<String>;
    fn untag_item(&mut self, key: &str, tag: &str);
    fn tag_exists(&self, tag: &str) -> bool;
}
