#![deny(warnings)]
#![feature(const_mut_refs)]
#![feature(const_ptr_is_null)]

use core::fmt;

/// C linked list wrapper.
pub struct Chain<T, N: Fn(&T) -> *mut T> {
    head: *mut T,
    next: N,
}

/// Iterator over a `Chain`. Returns references to the nodes of the list.
pub struct ChainIter<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> {
    chain: &'a Chain<T, N>,
    prev: Option<&'a T>,
}

/// Iterator over a `Chain`. Returns mutable references to the nodes of the list.
pub struct ChainIterMut<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> {
    chain: &'a mut Chain<T, N>,
    prev: Option<&'a mut T>,
}

impl<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> Chain<T, N> {
    /// Construct a `Chain` by wrapping a C linked list. `head` points to the head element
    /// of the list or is NULL for a list of length 0. `next` is a function that takes a node and
    /// returns a pointer to the next element.
    ///
    /// # Example
    ///
    /// To wrap this C type.
    ///
    /// ```c
    /// struct LinkedListNode {
    ///     int value;
    ///     struct LinkedListNode *next;
    /// };
    /// ```
    ///
    /// Call this function as `Chain::from_ptr(ptr_to_head, |n| n.next)`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it is up to the caller to ensure that `head` is valid.
    pub const unsafe fn from_ptr(head: *mut T, next: N) -> Chain<T, N> {
        Chain { head, next }
    }

    /// Iterate over the linked list, returning references to the nodes of the list.
    pub const fn iter(&'a self) -> ChainIter<'a, T, N> {
        ChainIter {
            chain: self,
            prev: None,
        }
    }

    /// Iterate over the linked list, returning mutable references to the nodes of the list.
    pub const fn iter_mut(&'a mut self) -> ChainIterMut<'a, T, N> {
        ChainIterMut {
            chain: self,
            prev: None,
        }
    }

    /// Returns `true` if the list is empty.
    pub const fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// Calculates the length of the list. This is an `O(n)` operation.
    pub fn len(&self) -> usize {
        let mut node = self.head;
        let mut ret = 0;

        while !node.is_null() {
            node = unsafe { (self.next)(&mut *node) };
            ret += 1;
        }

        ret
    }

    /// Provides a reference to the front element in the list, or `None` if the list is empty.
    pub const fn front(&self) -> Option<&T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&*self.head) }
        }
    }

    /// Provides a mutable reference to the front element in the list, or `None` if the list is
    /// empty.
    pub const fn front_mut(&self) -> Option<&mut T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&mut *self.head) }
        }
    }
}

impl<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> IntoIterator for &'a Chain<T, N> {
    type Item = &'a T;
    type IntoIter = ChainIter<'a, T, N>;

    fn into_iter(self) -> ChainIter<'a, T, N> {
        self.iter()
    }
}

impl<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> IntoIterator for &'a mut Chain<T, N> {
    type Item = &'a mut T;
    type IntoIter = ChainIterMut<'a, T, N>;

    fn into_iter(self) -> ChainIterMut<'a, T, N> {
        self.iter_mut()
    }
}

impl<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> Iterator for ChainIter<'a, T, N> {
    type Item = &'a T;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<&'a T> {
        // Note: implemented this way so that if the user changes the next pointer during iteration
        // it will iterate to the correct next element.
        let next = match self.prev {
            None => self.chain.head,
            Some(ref mut prev) => (self.chain.next)(*prev),
        };
        if next.is_null() {
            None
        } else {
            self.prev = Some(unsafe { &*next });

            Some(unsafe { &*next })
        }
    }
}

impl<'a, T: 'a, N: Fn(&T) -> *mut T + 'a> Iterator for ChainIterMut<'a, T, N> {
    type Item = &'a mut T;

    #[allow(unsafe_code)]
    fn next(&mut self) -> Option<&'a mut T> {
        // Note: implemented this way so that if the user changes the next pointer during iteration
        // it will iterate to the correct next element.
        let next = match self.prev {
            None => self.chain.head,
            Some(ref mut prev) => (self.chain.next)(*prev),
        };

        if next.is_null() {
            None
        } else {
            self.prev = Some(unsafe { &mut *next });

            Some(unsafe { &mut *next })
        }
    }
}

impl<'a, T: fmt::Debug + 'a, N: Fn(&T) -> *mut T + 'a> fmt::Debug for Chain<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
