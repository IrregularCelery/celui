use crate::vec::Vec;

const EMPTY: u64 = 0; // Marker for empty slots
const TOMBSTONE: u64 = 1; // Marker for deleted slots
const DEFAULT_CAPACITY: usize = 8; // Minimum non-zero capacity, like Vec's 4

// --------------------------------- Iter ---------------------------------- //

/// An iterator over the key-value pairs of a `HashMap`.
pub struct Iter<'a, K, V> {
    hash_map: &'a HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.hash_map.hashes.len() {
            let i = self.index;

            self.index += 1;

            if self.hash_map.hashes[i] > TOMBSTONE {
                // SAFETY: Slot is occupied (hash > TOMBSTONE), so key and value are initialized
                unsafe {
                    return Some((
                        &*self.hash_map.keys[i].as_ptr(),
                        &*self.hash_map.values[i].as_ptr(),
                    ));
                }
            }
        }

        None
    }
}

// ------------------------------- IterMut --------------------------------- //

/// A mutable iterator over the key-value pairs of a `HashMap`.
pub struct IterMut<'a, K, V> {
    hash_map: &'a mut HashMap<K, V>,
    index: usize,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.hash_map.hashes.len() {
            let i = self.index;

            self.index += 1;

            if self.hash_map.hashes[i] > TOMBSTONE {
                // SAFETY: Slot is occupied (hash > TOMBSTONE), so key and value are initialized
                unsafe {
                    return Some((
                        &*self.hash_map.keys[i].as_ptr(),
                        &mut *self.hash_map.values[i].as_mut_ptr(),
                    ));
                }
            }
        }

        None
    }
}

// ----------------------------- QuickHasher ------------------------------- //

/// A simple, fast hasher for internal use in `HashMap`.
///
/// Uses a multiplication-based hashing strategy with wrapping arithmetic.
#[derive(Default)]
pub struct QuickHasher {
    value: u64,
}

impl QuickHasher {
    /// Creates a new `QuickHasher` with an initial value of `0`.
    #[inline(always)]
    pub const fn new() -> Self {
        Self { value: 0 }
    }
}

impl core::hash::Hasher for QuickHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.value
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.value = self.value.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
}

// ------------------------------- HashMap --------------------------------- //

/// A hash map implementation using open addressing with linear probing.
///
/// Stores key-value pairs in a dynamically resizing array, using tombstones
/// to handle deletions. Provides efficient insertion, lookup, and removal.
///
/// # Example
/// ```
/// use celui_collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("key", 69);
/// assert_eq!(map.get(&"key"), Some(&69));
/// ```
pub struct HashMap<K, V> {
    keys: Vec<core::mem::MaybeUninit<K>>, // Uninitialized storage for keys
    values: Vec<core::mem::MaybeUninit<V>>, // Uninitialized storage for values
    hashes: Vec<u64>,                     // Hash values
    mask: usize,                          // Bitmask for indexing (capacity - 1)
    elements: usize,                      // Number of active key-value pairs
    tombstone_count: usize,               // Number of tombstone slots
}

impl<K: core::hash::Hash + Eq, V> HashMap<K, V> {
    /// Creates a new, empty hash map with a default capacity of `DEFAULT_CAPACITY`.
    #[inline]
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }

    /// Creates a new hash map with a specified initial capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();

        let mut keys = Vec::with_capacity(capacity);
        let mut values = Vec::with_capacity(capacity);
        let mut hashes = Vec::with_capacity(capacity);

        // SAFETY: Setting length to capacity is safe since we don't access uninitialized memory
        // until it's written to, and `hashes` is initialized with `EMPTY`.
        unsafe {
            keys.set_len(capacity);
            values.set_len(capacity);
            hashes.set_len(capacity);
            hashes.fill(EMPTY);
        }

        Self {
            keys,
            values,
            hashes,
            mask: capacity - 1,
            elements: 0,
            tombstone_count: 0,
        }
    }

    /// Returns the number of key-value pairs in the map.
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.elements
    }

    /// Returns the current capacity of the map.
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.hashes.len()
    }

    /// Returns `true` if the map contains no key-value pairs.
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.elements == 0
    }

    /// Returns `true` if the map contains the given key.
    #[inline(always)]
    pub fn contains_key<Q: core::hash::Hash + Eq + ?Sized>(&self, key: &Q) -> bool
    where
        K: core::borrow::Borrow<Q>,
    {
        self.get(key).is_some()
    }

    /// Returns a reference to the value associated with the given key, or `None` if not found.
    #[inline]
    pub fn get<Q: core::hash::Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: core::borrow::Borrow<Q>,
    {
        if self.elements == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let i = self.find_slot(hash, key)?;

        if self.hashes[i] <= TOMBSTONE {
            return None;
        }

        // SAFETY: Slot is occupied (hash > TOMBSTONE), so value is initialized
        Some(unsafe { &*self.values[i].as_ptr() })
    }

    /// Returns a mutable reference to the value associated with the key, or `None` if not found.
    #[inline]
    pub fn get_mut<Q: core::hash::Hash + Eq + ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: core::borrow::Borrow<Q>,
    {
        if self.elements == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let i = self.find_slot(hash, key)?;

        if self.hashes[i] <= TOMBSTONE {
            return None;
        }

        // SAFETY: Slot is occupied (hash > TOMBSTONE), so value is initialized
        Some(unsafe { &mut *self.values[i].as_mut_ptr() })
    }

    /// Returns an iterator over the map's key-value pairs.
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            hash_map: self,
            index: 0,
        }
    }

    /// Returns a mutable iterator over the map's key-value pairs.
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            hash_map: self,
            index: 0,
        }
    }

    /// Inserts a key-value pair, replacing the old value if the key already exists.
    ///
    /// Returns the previous value if the key was already present, or `None` if it was inserted.
    #[inline]
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.elements + self.tombstone_count >= (self.hashes.len() * 3) / 4 {
            self.resize();
        }

        let hash = self.hash_key(&key);
        let i = self.find_slot(hash, &key).expect("HashMap is full");

        if self.hashes[i] > TOMBSTONE {
            // SAFETY: Slot is occupied (hash > TOMBSTONE), so value is initialized
            unsafe {
                let old_value =
                    core::mem::replace(&mut self.values[i], core::mem::MaybeUninit::new(value));

                self.keys[i].write(key);

                return Some(old_value.assume_init());
            }
        }

        if self.hashes[i] == TOMBSTONE {
            self.tombstone_count -= 1;
        }

        self.elements += 1;
        self.hashes[i] = hash;
        self.keys[i].write(key);
        self.values[i].write(value);

        None
    }

    /// Removes and returns the value associated with the key, or `None` if not found.
    #[inline]
    pub fn remove<Q: core::hash::Hash + Eq + ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: core::borrow::Borrow<Q>,
    {
        if self.elements == 0 {
            return None;
        }

        let hash = self.hash_key(key);
        let i = self.find_slot(hash, key)?;

        if self.hashes[i] <= TOMBSTONE {
            return None;
        }

        self.elements -= 1;
        self.tombstone_count += 1;
        self.hashes[i] = TOMBSTONE;

        // SAFETY: Slot is occupied (hash > TOMBSTONE), so key and value are initialized
        unsafe {
            let _ = core::mem::replace(&mut self.keys[i], core::mem::MaybeUninit::uninit());
            let value = core::mem::replace(&mut self.values[i], core::mem::MaybeUninit::uninit());

            Some(value.assume_init())
        }
    }

    /// Removes all key-value pairs from the map, leaving it empty.
    #[inline]
    pub fn clear(&mut self) {
        if self.elements == 0 {
            return;
        }

        for i in 0..self.hashes.len() {
            if self.hashes[i] > TOMBSTONE {
                let _ = core::mem::replace(&mut self.keys[i], core::mem::MaybeUninit::uninit());
                let _ = core::mem::replace(&mut self.values[i], core::mem::MaybeUninit::uninit());
            }
        }

        self.hashes.fill(EMPTY);
        self.elements = 0;
        self.tombstone_count = 0;
    }

    /// Ensures the map has space for at least `additional` more key-value pairs.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let needed_capacity = self.len() + additional;

        if needed_capacity > self.capacity() {
            let new_capacity = needed_capacity.next_power_of_two();
            let mut new_map = Self::with_capacity(new_capacity);

            for i in 0..self.hashes.len() {
                let hash = self.hashes[i];

                if hash > TOMBSTONE {
                    // SAFETY: Slot is occupied (hash > TOMBSTONE), so key and value are initialized
                    unsafe {
                        let key =
                            core::mem::replace(&mut self.keys[i], core::mem::MaybeUninit::uninit());
                        let value = core::mem::replace(
                            &mut self.values[i],
                            core::mem::MaybeUninit::uninit(),
                        );

                        new_map.insert_unchecked(hash, key.assume_init(), value.assume_init());
                    }
                }
            }

            *self = new_map;
        }
    }

    /// Returns a 64-bit hash for a key, ensuring it doesn't overlap with special markers.
    #[inline(always)]
    fn hash_key<Q: core::hash::Hash + ?Sized>(&self, key: &Q) -> u64 {
        let mut hasher = QuickHasher::new();

        key.hash(&mut hasher);

        let hash = core::hash::Hasher::finish(&hasher);

        // Ensure hash is greater than TOMBSTONE to avoid conflicts with markers
        if hash <= TOMBSTONE {
            hash + 2
        } else {
            hash
        }
    }

    /// Finds the slot for a key based on its hash, returning the index if found or a free slot.
    #[inline]
    fn find_slot<Q: Eq + ?Sized>(&self, hash: u64, key: &Q) -> Option<usize>
    where
        K: core::borrow::Borrow<Q>,
    {
        debug_assert!(!self.hashes.is_empty());
        debug_assert!(hash > TOMBSTONE);

        let mut i = (hash as usize) & self.mask;
        let start_i = i;
        let mut first_tombstone = None;
        let mut visited = 0;

        while visited < self.hashes.len() {
            let current_hash = self.hashes[i];

            if current_hash == EMPTY {
                return first_tombstone.or(Some(i));
            }

            if current_hash == TOMBSTONE && first_tombstone.is_none() {
                first_tombstone = Some(i);
            } else if current_hash == hash {
                // SAFETY: Slot is occupied (hash > TOMBSTONE), so key is initialized
                let current_key = unsafe { &*self.keys[i].as_ptr() };

                if key == current_key.borrow() {
                    return Some(i);
                }
            }

            i = (i + 1) & self.mask;
            visited += 1;

            if i == start_i {
                return first_tombstone;
            }
        }

        // If we've probed all slots and found no place, return the first tombstone if any
        first_tombstone
    }

    /// Resizes the hash map to at least double its current capacity, rehashing all entries.
    #[inline]
    fn resize(&mut self) {
        let new_capacity = if self.hashes.is_empty() {
            DEFAULT_CAPACITY
        } else {
            self.hashes.len() * 2
        };
        let mut new_map = Self::with_capacity(new_capacity);

        for i in 0..self.hashes.len() {
            let hash = self.hashes[i];

            if hash > TOMBSTONE {
                // SAFETY: Slot is occupied (hash > TOMBSTONE), so key and value are initialized
                unsafe {
                    let key =
                        core::mem::replace(&mut self.keys[i], core::mem::MaybeUninit::uninit());
                    let value =
                        core::mem::replace(&mut self.values[i], core::mem::MaybeUninit::uninit());

                    new_map.insert_unchecked(hash, key.assume_init(), value.assume_init());
                }
            }
        }

        *self = new_map;
    }

    /// Inserts a key-value pair without checking capacity, assuming a slot is available.
    ///
    /// # SAFETY
    /// The caller must ensure:
    /// - `hash` must be greater than `TOMBSTONE`.
    /// - A slot must be available (caller must ensure via `find_slot`).
    #[inline]
    unsafe fn insert_unchecked(&mut self, hash: u64, key: K, value: V) {
        debug_assert!(hash > TOMBSTONE);

        let i = self.find_slot(hash, &key).expect("HashMap is full");

        if self.hashes[i] <= TOMBSTONE {
            self.elements += 1;

            if self.hashes[i] == TOMBSTONE {
                self.tombstone_count -= 1;
            }
        }

        self.hashes[i] = hash;
        self.keys[i].write(key);
        self.values[i].write(value);
    }
}

impl<K: core::hash::Hash + Eq, V> Default for HashMap<K, V> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Drop for HashMap<K, V> {
    fn drop(&mut self) {
        for i in 0..self.hashes.len() {
            if self.hashes[i] > TOMBSTONE {
                // SAFETY: Slot is occupied (hash > TOMBSTONE), so key and value are initialized
                unsafe {
                    core::ptr::drop_in_place(self.keys[i].as_mut_ptr());
                    core::ptr::drop_in_place(self.values[i].as_mut_ptr());
                }
            }
        }
    }
}
