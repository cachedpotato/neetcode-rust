use rust_linked_list::lrucache::LRUCache;
fn main() {
    let mut lru: LRUCache<i32> = LRUCache::new(2);
    assert_eq!(lru.get(1), None);
    lru.put((1, 10));
    assert_eq!(lru.len(), 1);
    assert_eq!(lru.get(1), Some(&10));
    lru.put((2, 20));
    lru.put((3, 30));
    assert_eq!(lru.get(2), Some(&20));
    assert_eq!(lru.get(1), None);

}
