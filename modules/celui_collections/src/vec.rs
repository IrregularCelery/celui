use celui_sys::alloc::{alloc_many, dealloc_many};

const MIN_NON_ZERO_CAPACITY: usize = 4; // Start with a slightly larger minimum capacity

// ------------------------------- IntoIter -------------------------------- //

/// An owning iterator that consumes the `Vec` and returns its elements.
pub struct IntoIter<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
    current: usize,
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // Drop remaining elements
        while self.current < self.len {
            // SAFETY: `self.current` is guaranteed to be within bounds
            unsafe {
                core::ptr::drop_in_place(self.ptr.add(self.current));

                self.current += 1;
            }
        }

        if !self.ptr.is_null() && self.capacity != 0 {
            // SAFETY: `self.ptr` is a valid pointer, and `self.capacity` is not 0
            unsafe { dealloc_many(self.ptr, self.capacity) };
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.len {
            return None;
        }

        // SAFETY: `self.current` is guaranteed to be within bounds
        let item = unsafe { core::ptr::read(self.ptr.add(self.current)) };

        self.current += 1;

        Some(item)
    }
}

// --------------------------------- Iter ---------------------------------- //

/// An iterator over the elements of a `Vec`.
pub struct Iter<'a, T> {
    ptr: *const T,
    end: *const T,

    _marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub fn new(vec: &'a Vec<T>) -> Self {
        let ptr = vec.ptr;
        let len = vec.len;

        // SAFETY: `vec` is a valid Vec<T>, therefore its fields `ptr` and `len` are valid as well
        unsafe {
            Self {
                ptr,
                end: ptr.add(len),

                _marker: core::marker::PhantomData,
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        // SAFETY: `self.ptr` is a valid pointer for reads up to `self.end` elements
        unsafe {
            let item = &*self.ptr;

            self.ptr = self.ptr.add(1);

            Some(item)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end as usize - self.ptr as usize) / core::mem::size_of::<T>();

        (remaining, Some(remaining))
    }
}

// ------------------------------- IterMut --------------------------------- //

/// A mutable iterator over the elements of a `Vec`.
pub struct IterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,

    _marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub fn new(vec: &'a mut Vec<T>) -> Self {
        let ptr = vec.ptr;
        let len = vec.len;

        // SAFETY: `vec` is a valid Vec<T>, therefore its fields `ptr` and `len` are valid as well
        unsafe {
            Self {
                ptr,
                end: ptr.add(len),

                _marker: core::marker::PhantomData,
            }
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr == self.end {
            return None;
        }

        // SAFETY: `self.ptr` is a valid pointer for reads up to `self.end` elements
        unsafe {
            let item = &mut *self.ptr;

            self.ptr = self.ptr.add(1);

            Some(item)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.end as usize - self.ptr as usize) / core::mem::size_of::<T>();

        (remaining, Some(remaining))
    }
}

// --------------------------------- Vec ----------------------------------- //

/// A dynamic, heap-allocated array type.
///
/// Provides methods for adding, removing, and accessing elements efficiently.
///
/// # Example
/// ```
/// let mut vec = Vec::new();
/// vec.push(69);
/// assert_eq!(vec.pop(), Some(69));
/// ```
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    /// Creates a new, empty vector with zero capacity.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            ptr: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    /// Creates a new vector with a specified initial capacity.
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        // SAFETY: `alloc_many` returns a valid pointer
        unsafe {
            let ptr = alloc_many(capacity);

            Self {
                ptr,
                len: 0,
                capacity,
            }
        }
    }

    /// Returns the number of elements in the vector.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the capacity of the vector.
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns `true` if the vector contains no elements.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to the element at the given index, or `None` if out of bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            // SAFETY: `index` is guaranteed to be within bounds
            return Some(unsafe { &*self.ptr.add(index) });
        }

        None
    }

    /// Returns a mutable reference to the element at the given index, or `None` if out of bounds.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            // SAFETY: `index` is guaranteed to be within bounds
            return Some(unsafe { &mut *self.ptr.add(index) });
        }

        None
    }

    /// Returns a slice containing all elements of the vector.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: `self.len` ensures we only create a valid slice within the bounds
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// Returns a mutable slice containing all elements of the vector.
    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        // SAFETY: `self.len` ensures we only create a valid slice within the bounds
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    /// Returns an iterator over the slice.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Returns a mutable iterator over the slice.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }

    /// Adds an element to the end of the vector, growing its capacity if needed.
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            // `0` means default growth strategy
            self.grow(0);
        }

        // SAFETY: `self.len` is always within the allocated capacity
        unsafe { core::ptr::write(self.ptr.add(self.len), value) };

        self.len += 1;
    }

    /// Removes the last element from the vector and returns it, or `None` if empty.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let old_len = self.len;

        self.len = old_len - 1;

        // SAFETY: `self.len` is gauranteed to be more than 0
        unsafe { Some(core::ptr::read(self.ptr.add(old_len - 1))) }
    }

    /// Inserts an element at the given index, shifting subsequent elements to the right.
    pub fn insert(&mut self, index: usize, element: T) {
        assert!(index <= self.len, "Index out of bounds");

        if self.len == self.capacity {
            // `0` means default growth strategy
            self.grow(0);
        }

        // SAFETY: `index` is within bounds; we shift elements to prevent overwrites
        unsafe {
            core::ptr::copy(
                self.ptr.add(index),
                self.ptr.add(index + 1),
                self.len - index,
            );

            core::ptr::write(self.ptr.add(index), element);
        };

        self.len += 1;
    }

    /// Removes and returns the element at the given index, shifting subsequent elements left.
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "Index out of bounds");

        // SAFETY: `index` is within bounds; we shift elements after reading the value
        unsafe {
            let value = core::ptr::read(self.ptr.add(index));

            core::ptr::copy(
                self.ptr.add(index + 1),
                self.ptr.add(index),
                self.len - index - 1,
            );

            self.len -= 1;

            value
        }
    }

    /// Fills the vector with elements by cloning `value`.
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.as_slice_mut().fill(value);
    }

    /// Removes all elements from the vector.
    #[inline]
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Extends the vector with the contents of an iterator.
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let additional = upper.unwrap_or(lower);

        if additional > 0 {
            self.reserve(additional);
        }

        // SAFETY: We reserved `additional` elements, and the iterator is bounded
        unsafe {
            let mut ptr = self.ptr.add(self.len);
            let mut count = 0;

            for item in iter {
                if count < additional {
                    core::ptr::write(ptr, item);

                    ptr = ptr.add(1);
                    count += 1;

                    continue;
                }

                // Fallback to `push()` if iterator yields more than expected
                self.push(item);
            }

            self.len += count;
        }
    }

    /// Truncates the vector to a new length, removing excess elements.
    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.len {
            let old_len = self.len;

            self.len = new_len;

            // SAFETY: `new_len` is guaranteed to be less than `self.len`
            unsafe {
                let slice =
                    core::ptr::slice_from_raw_parts_mut(self.ptr.add(new_len), old_len - new_len);

                core::ptr::drop_in_place(slice);
            }
        }
    }

    /// Ensures that the vector has at least `additional` capacity.
    pub fn reserve(&mut self, additional: usize) {
        let new_capacity = self.len.checked_add(additional).expect("Capacity overflow");

        if new_capacity > self.capacity {
            self.grow(new_capacity);
        }
    }

    /// Sets the length of the vector.
    ///
    /// # Safety
    /// The caller must ensure:
    /// - `new_len` does not exceed the vector's capacity
    /// - All elements between old length and new length are initialized
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());

        self.len = new_len;
    }

    /// Increases the vector's capacity to accommodate at least `minimum_capacity` elements.
    #[inline]
    fn grow(&mut self, minimum_capacity: usize) {
        let new_capacity = if self.capacity == 0 {
            MIN_NON_ZERO_CAPACITY.max(minimum_capacity)
        } else {
            self.capacity
                .checked_mul(2)
                .expect("Capacity overflow")
                .max(minimum_capacity)
        };

        debug_assert!(
            new_capacity >= self.len,
            "New capacity must be at least current length"
        );

        // SAFETY: `alloc_many` returns a valid pointer
        let new_ptr = unsafe { alloc_many(new_capacity) };

        if !self.ptr.is_null() {
            // SAFETY: `self.ptr` is a valid pointer, and `new_capacity` is >= `self.len`
            unsafe {
                core::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);

                dealloc_many(self.ptr, self.capacity);
            }
        }

        self.ptr = new_ptr;
        self.capacity = new_capacity;
    }
}

impl<T> Default for Vec<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::with_capacity(self.capacity);

        // SAFETY: `self.ptr` and `new_vec.ptr` are both valid pointers
        unsafe {
            core::ptr::copy_nonoverlapping(self.ptr, new_vec.ptr, self.len);
        }

        new_vec.len = self.len;

        new_vec
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        let ptr = self.ptr;
        let len = self.len;
        let capacity = self.capacity;

        // Prevent the Vec from deallocating the buffer
        self.ptr = core::ptr::null_mut();
        self.len = 0;
        self.capacity = 0;

        IntoIter {
            ptr,
            len,
            capacity,
            current: 0,
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        self.clear();

        if !self.ptr.is_null() && self.capacity != 0 {
            // SAFETY: `self.ptr` is a valid pointer, and `self.capacity` is not 0
            unsafe { dealloc_many(self.ptr, self.capacity) };
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len, "Index out of bounds");

        // SAFETY: `index` is guaranteed to be within bounds
        unsafe { &*self.ptr.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len, "Index out of bounds");

        // SAFETY: `index` is guaranteed to be within bounds
        unsafe { &mut *self.ptr.add(index) }
    }
}
