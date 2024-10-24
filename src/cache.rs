use std::collections::HashMap;
pub struct CacheEntry {
    pub name: String,
    pub data: Vec<u8>,
    pub expiry: u64,
    pub size: u64,
    pub checksum: u32,
}

impl CacheEntry {
    pub fn new(name: &str, data: Vec<u8>, expiry: u64, size: u64, checksum: u32) -> Self {
        Self {
            name: name.to_string(),
            data,
            expiry,
            size,
            checksum,
        }
    }
}

struct TrieNode {
    children: HashMap<String, TrieNode>,
    cache_entry: Option<CacheEntry>,
}

impl TrieNode {
    pub fn new(cache_entry: Option<CacheEntry>) -> Self {
        Self {
            children: HashMap::new(),
            cache_entry,
        }
    }
}

pub struct Cache {
    trie_root: TrieNode,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            trie_root: TrieNode::new(None),
        }
    }

    pub fn get(&self, name: &str) -> Option<&CacheEntry> {
        let name_segments = name.split('/').collect::<Vec<&str>>();
        let mut current_node = &self.trie_root;
        for segment in name_segments {
            current_node = current_node.children.get(segment).unwrap();
        }
        current_node.cache_entry.as_ref()
    }

    pub fn set(&mut self, entry: CacheEntry) {
        let name_segments = entry.name.split('/').collect::<Vec<&str>>();
        let mut current_node = &mut self.trie_root;
        for segment in name_segments {
            current_node = current_node
                .children
                .entry(segment.to_string())
                .or_insert(TrieNode::new(None));
        }
        current_node.cache_entry = Some(entry);
    }
}
