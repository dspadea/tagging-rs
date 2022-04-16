# Tag Index

This is a library which abstracts the indexing of tags using multiple
backing stores. 

Tags are arbitrary bits of metadata which can be attached to items.

The tag index specializes in tagging items, storing the tags to various 
backing stores, and reporting which items have a given tag, or which tags a
given item has.

# Purpose

This library is solely for tracking tags. It is not meant to hold your entire object's data. It simply assigns tags to your object's identifier (primary key, perhaps). From the lists returned by this library, 
you can look up your object data elsewhere. 

# Storage

Currently, there are two backing stores implemented: `memory` and `redis`. 

# Concurrency

This is very early phases. It seems to do its job well in a single-threaded, non-async 
environment. Making this thread-safe and async are next up, but I wanted to work on core
functionality first. Expect the APIs to change in this regard. 

# USAGE

## Create a tag index store
```rust
// For in-memory persistence
let mut idx = MemoryTagIndex::new();
idx.start();


// For Redis persistence
let mut idx = RedisTagIndex::new_from_redis_nodelist(vec!["redis://127.0.0.1".to_string()]);
idx.start();

```

# Use the index
```rust
idx.tag_item("my_key", "my_tag");

let exists = idx.tag_exists("my_tag");

let items = idx.retrieve_items_with_tag("my_tag");

let tags = idx.retrieve_tags_for_item("my_key");

idx.untag_item("my_key", "my_tag");

```
