use std::cell::UnsafeCell;
use crossbeam::epoch::Atomic;

/// Entry is a bin
///
/// Will _generally_ be `Node`. Any entry that is not first in the bin, will be a `Node` 
pub(crate) enum BinEntry<K,V> {
    Node(Node<K,V>)
}



/// Key-value entry.  
pub(crate) struct Node<K,V> {
    pub(crate) hash: u64,
    pub(crate) key: K,
    pub(crate) value: UnsafeCell<V>,
    pub(crate) next: Atomic<Node<K,V>>,
}


