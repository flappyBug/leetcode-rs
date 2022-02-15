// https://leetcode.com/problems/lru-cache/
// ref: https://docs.rs/lru/0.1.11/src/lru/lib.rs.html
use std::collections::HashMap;
use std::mem;
use std::mem::MaybeUninit;
use std::ptr;

struct LRUEntry {
    key: MaybeUninit<i32>,
    val: MaybeUninit<i32>,
    prev: *mut LRUEntry,
    next: *mut LRUEntry,
}

impl LRUEntry {
    fn new(key: i32, val: i32) -> Self {
        LRUEntry {
            key: MaybeUninit::new(key),
            val: MaybeUninit::new(val),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }

    fn new_sigil() -> Self {
        LRUEntry {
            key: mem::MaybeUninit::uninit(),
            val: mem::MaybeUninit::uninit(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

pub struct LRUCache {
    map: HashMap<i32, Box<LRUEntry>>,
    cap: usize,
    head: *mut LRUEntry,
    tail: *mut LRUEntry,
}

impl LRUCache {
    pub fn new(cap: i32) -> Self {
        let cap = cap as usize;
        let head: *mut LRUEntry = Box::into_raw(Box::new(LRUEntry::new_sigil()));
        let tail: *mut LRUEntry = Box::into_raw(Box::new(LRUEntry::new_sigil()));
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        Self {
            map: HashMap::with_capacity(cap),
            cap,
            head,
            tail,
        }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let (node_ptr, val) = if let Some(node) = self.map.get_mut(&key) {
            (&mut **node as *mut LRUEntry, node.val)
        } else {
            return -1;
        };
        self.detach(node_ptr);
        self.attach(node_ptr);
        unsafe { val.assume_init() }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let node_ptr = self
            .map
            .get_mut(&key)
            .map(|node| &mut (**node) as *mut LRUEntry);
        if let Some(node_ptr) = node_ptr {
            unsafe { (*node_ptr).val = MaybeUninit::new(value) }
            self.detach(node_ptr);
            self.attach(node_ptr);
        } else {
            let mut node = if self.len() == self.cap() {
                let old_key = unsafe { (*(*self.tail).prev).key.assume_init() };
                let mut old_node = self.map.remove(&old_key).unwrap();
                old_node.key = MaybeUninit::new(key);
                old_node.val = MaybeUninit::new(value);
                self.detach(&mut *old_node as *mut LRUEntry);
                old_node
            } else {
                Box::new(LRUEntry::new(key, value))
            };
            let node_ptr = &mut *node as *mut LRUEntry;
            self.attach(node_ptr);
            self.map.insert(key, node);
        }
    }

    fn detach(&mut self, node: *mut LRUEntry) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    fn attach(&mut self, node: *mut LRUEntry) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
            (*(*node).next).prev = node;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LRUCache;

    #[test]
    fn use_case() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
