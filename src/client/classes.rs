use super::Class;
use core::marker::PhantomData;
use daisy_chain::{Chain, ChainIter};

type Next = fn(&Class) -> *mut Class;

fn next(class: &Class) -> *mut Class {
    class.next
}

/// A list of classes.
pub struct Classes<'class> {
    chain: Chain<Class, Next>,
    phantom: PhantomData<&'class Class>,
}

impl<'class> Classes<'class> {
    pub fn new(ptr: *mut Class) -> Self {
        unsafe {
            let chain = Chain::from_ptr(ptr, next as Next);

            Self {
                chain,
                phantom: PhantomData,
            }
        }
    }

    pub fn iter(&'class self) -> ClassesIter<'class> {
        ClassesIter::new(self)
    }
}

/// An iterator over classes.
pub struct ClassesIter<'class> {
    iter: ChainIter<'class, Class, Next>,
}

impl<'class> ClassesIter<'class> {
    pub(crate) fn new(this: &'class Classes) -> Self {
        Self {
            iter: this.chain.iter(),
        }
    }
}

impl<'class> Iterator for ClassesIter<'class> {
    type Item = &'class Class;

    fn next(&mut self) -> Option<&'class Class> {
        Iterator::next(&mut self.iter)
    }
}
