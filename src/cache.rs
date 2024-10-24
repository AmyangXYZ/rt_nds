use std::collections::HashMap;
use std::time;

struct TrieNode {
    pub children: HashMap<String, TrieNode>,
    pub data: Vec<u8>,
    pub expiry: time::SystemTime,
}

impl TrieNode {
    pub fn new(data: Vec<u8>, expiry: time::SystemTime) -> Self {
        Self {
            children: HashMap::new(),
            data,
            expiry,
        }
    }
}

pub struct Cache {
    trie_root: TrieNode,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            trie_root: TrieNode::new(vec![], time::SystemTime::now()),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        let name_segments = name.split('/').collect::<Vec<&str>>();
        let mut current_node = &self.trie_root;

        for (i, &segment) in name_segments.iter().enumerate() {
            match current_node.children.get(segment) {
                Some(node) => current_node = node,
                None => return None,
            }

            if i == name_segments.len() - 1 {
                if current_node.expiry > time::SystemTime::now() {
                    return Some(&current_node.data);
                } else {
                    return None;
                }
            }
        }

        None
    }

    pub fn set(&mut self, name: &str, data: Vec<u8>, expiry: time::Duration) {
        let name_segments = name.split('/').collect::<Vec<&str>>();
        let mut current_node = &mut self.trie_root;

        for &segment in &name_segments[..name_segments.len() - 1] {
            current_node = current_node
                .children
                .entry(segment.to_string())
                .or_insert_with(|| TrieNode::new(vec![], time::SystemTime::now()));
        }

        if let Some(&last_segment) = name_segments.last() {
            current_node
                .children
                .entry(last_segment.to_string())
                .or_insert_with(|| TrieNode::new(data.clone(), time::SystemTime::now() + expiry));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cache_set_and_get() {
        let mut cache = Cache::new();
        let data = vec![1, 2, 3];
        let expiry_duration = Duration::from_secs(1);

        cache.set("test/test", data.clone(), expiry_duration);

        let result = cache.get("test/test");

        assert_eq!(result, Some(&data));

        std::thread::sleep(expiry_duration);

        let expired_result = cache.get("test/test");
        assert_eq!(expired_result, None);
    }
}
