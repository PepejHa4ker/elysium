use super::UtlMem;

#[repr(C)]
pub struct Node<K, V> {
    pub left: i32,
    pub right: i32,
    pub parent: i32,
    pub kind: i32,
    pub key: K,
    pub val: V,
}

#[repr(C)]
pub struct UtlMap<K, V> {
    pub cmp_fn: *const (),
    pub mem: UtlMem<Node<K, V>>,
    pub root: i32,
    pub elements_len: i32,
    pub first_free: i32,
    pub last_alloc: i32,
    pub elements: *const Node<K, V>,
}

impl<K, V> UtlMap<K, V> {
    pub fn find<'a>(&self, key: &'a K) -> i32
    where
        K: PartialOrd<&'a K>,
    {
        let mut current = self.root;

        while current != -1 {
            let node = unsafe { &*self.mem.mem.offset(current as isize) };

            if node.key < key {
                current = node.right;
            } else if node.key > key {
                current = node.left;
            } else {
                break;
            }
        }

        current
    }

    pub fn first(&self) -> *const Node<K, V> {
        self.mem.mem
    }

    pub fn last(&self) -> *const Node<K, V> {
        unsafe { self.mem.mem.add(self.elements_len as usize) }
    }
}
