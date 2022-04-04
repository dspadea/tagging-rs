use redis::{Client, Commands};

use crate::storage::IndexStorage;

pub struct RedisTagIndex {
    cluster_nodes: Vec<String>,
    redis_conn: Option<Client>
}

impl RedisTagIndex {
    pub fn new_from_redis_nodelist(cluster_nodes: Vec<String>) -> RedisTagIndex {
        
        RedisTagIndex { 
            cluster_nodes,
            redis_conn: None 
        }

    }
}

impl IndexStorage for RedisTagIndex {
    fn start(&mut self) {
        let redis_conn = redis::Client::open(self.cluster_nodes.first().unwrap().clone())
            .expect("Error connecting to Redis");
    
        self.redis_conn = Some(redis_conn);
    }

    fn shutdown(&mut self) {
        self.redis_conn = None;
    }

    fn tag_item(&mut self, key: &str, tag: &str) {
        let redis_cluster = self.redis_conn.as_ref().unwrap();
        let mut conn = redis_cluster.get_connection().unwrap();

        let _: ()  = conn.sadd(key.to_string(), tag.to_string()).unwrap();
        let _: ()  = conn.sadd(tag.to_string(), key.to_string()).unwrap();

    }

    fn retrieve_tags_for_item(&self, key: &str) -> Vec<String> {
        let redis_cluster = self.redis_conn.as_ref().unwrap();
        let mut conn = redis_cluster.get_connection().unwrap();

        conn.smembers(key).unwrap()
    }

    fn retrieve_items_with_tag(&self, tag: &str) -> Vec<String> {
        let redis_cluster = self.redis_conn.as_ref().unwrap();
        let mut conn = redis_cluster.get_connection().unwrap();

        conn.smembers(tag).unwrap()
    }

    fn untag_item(&mut self, key: &str, tag: &str) {
        let redis_cluster = self.redis_conn.as_ref().unwrap();
        let mut conn = redis_cluster.get_connection().unwrap();

        let _: ()  = conn.srem(tag, key).unwrap();
        let _: ()  = conn.srem(key,tag).unwrap();
    }

    fn tag_exists(&self, tag: &str) -> bool {
        let redis_cluster = self.redis_conn.as_ref().unwrap();
        let mut conn = redis_cluster.get_connection().unwrap();

        conn.exists(tag).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{RedisTagIndex, storage::IndexStorage};

    /*
    NOTE: The tests run in parallel, so make sure they use different key names. Otherwise
    they will delete each others objects from Redis, and the tests will fail.
    
    */


    #[test]
    fn add_item() {
        let mut idx = RedisTagIndex::new_from_redis_nodelist(vec!["redis://127.0.0.1".to_string()]);

        idx.start();
        idx.tag_item("my_key", "my_tag");

        assert!(idx.tag_exists("my_tag"));
        assert!(idx.retrieve_items_with_tag("my_tag").contains(&"my_key".to_string()));
        assert!(idx.retrieve_tags_for_item("my_key").contains(&"my_tag".to_string()));

        idx.untag_item("my_key", "my_tag");
    }


    #[test]
    fn test_remove_tag() {
        let mut idx = RedisTagIndex::new_from_redis_nodelist(vec!["redis://127.0.0.1".to_string()]);
        
        idx.start();
        idx.tag_item("my_key2", "my_tag2");

        assert!(idx.tag_exists("my_tag2"));
        assert!(idx.retrieve_items_with_tag("my_tag2").contains(&"my_key2".to_string()));
        assert!(idx.retrieve_tags_for_item("my_key2").contains(&"my_tag2".to_string()));

        idx.untag_item("my_key2", "my_tag2");
        assert!( !idx.tag_exists("my_tag2")); // For Redis, the tag ceases to exist
        assert!( !idx.retrieve_items_with_tag("my_tag2").contains(&"my_key2".to_string()) );
        assert!( !idx.retrieve_tags_for_item("my_key2").contains(&"my_tag2".to_string()) );

    }

    #[test]
    fn test_remove_nonexistent_tag() {
        let mut idx = RedisTagIndex::new_from_redis_nodelist(vec!["redis://127.0.0.1".to_string()]);
        
        idx.start();
       
        idx.untag_item("my_key3", "my_tag3");
        //assert!( !idx.tag_exists("my_tag")  );
        assert!( !idx.retrieve_items_with_tag("my_tag3").contains(&"my_key3".to_string()) );
        assert!( !idx.retrieve_tags_for_item("my_key3").contains(&"my_tag3".to_string()) );

    }

}