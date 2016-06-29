//! A non-resizable Vec, produced from a regular Vec.

/// A non-resizable vector, produced from a regular Vec.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct Array<T> {
    vec: Vec<T>
}

impl<T> Array<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.vec.len();
    }
}

impl<T: Clone> Array<T> {
    /// Create an Array by cloning a default value size times.
    pub fn with_default(size: usize, default: T) -> Array<T> {
        Array {
            vec: vec![default.clone(); size]
        }
    }
}

impl<T: Default> Array<T> {
    /// Create an Array by creating default values size times.
    pub fn new(size: usize) -> Array<T> {
        Array {
            vec: vec![Default::default(); size]
        }
    }
}

impl<T> From<Vec<T>> for Array<T> {
    #[inline]
    fn from(value: Vec<T>) -> Array<T> {
        Array {
            vec: value
        }
    }
}

impl<T: Clone> Clone for Array<T> {
    fn clone(&self) -> Array<T> {
        Array {
            vec: self.vec.clone()
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        // NB built-in indexing via `&[T]`
        self.vec.index(index)
    }
}

impl<T> IndexMut<usize> for Array<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.vec.index_mut(index)
    }
}

unsafe impl<T: Send> Send for Array<T> {}
unsafe impl<T: Sync> Sync for Array<T> {}