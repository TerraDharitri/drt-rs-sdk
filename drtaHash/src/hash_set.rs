use crate::RandomState;
use std::collections::{hash_set, HashSet};
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;
use std::ops::{BitAnd, BitOr, BitXor, Deref, DerefMut, Sub};

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

/// A [`HashSet`](std::collections::HashSet) using [`RandomState`](crate::RandomState) to hash the items.
/// (Requires the `std` feature to be enabled.)
#[derive(Clone)]
pub struct DRTAHashSet<T, S = crate::RandomState>(HashSet<T, S>);

impl<T> From<HashSet<T, crate::RandomState>> for DRTAHashSet<T> {
    fn from(item: HashSet<T, crate::RandomState>) -> Self {
        DRTAHashSet(item)
    }
}

impl<T> Into<HashSet<T, crate::RandomState>> for DRTAHashSet<T> {
    fn into(self) -> HashSet<T, crate::RandomState> {
        self.0
    }
}

impl<T> DRTAHashSet<T, RandomState> {
    pub fn new() -> Self {
        DRTAHashSet(HashSet::with_hasher(RandomState::default()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DRTAHashSet(HashSet::with_capacity_and_hasher(capacity, RandomState::default()))
    }
}

impl<T, S> DRTAHashSet<T, S>
where
    S: BuildHasher,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        DRTAHashSet(HashSet::with_hasher(hash_builder))
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        DRTAHashSet(HashSet::with_capacity_and_hasher(capacity, hash_builder))
    }
}

impl<T, S> Deref for DRTAHashSet<T, S> {
    type Target = HashSet<T, S>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> DerefMut for DRTAHashSet<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> PartialEq for DRTAHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    fn eq(&self, other: &DRTAHashSet<T, S>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T, S> Eq for DRTAHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
}

impl<T, S> BitOr<&DRTAHashSet<T, S>> for &DRTAHashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = DRTAHashSet<T, S>;

    /// Returns the union of `self` and `rhs` as a new `DRTAHashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use drtahash::DRTAHashSet;
    ///
    /// let a: DRTAHashSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: DRTAHashSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a | &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 3, 4, 5];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitor(self, rhs: &DRTAHashSet<T, S>) -> DRTAHashSet<T, S> {
        DRTAHashSet(self.0.bitor(&rhs.0))
    }
}

impl<T, S> BitAnd<&DRTAHashSet<T, S>> for &DRTAHashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = DRTAHashSet<T, S>;

    /// Returns the intersection of `self` and `rhs` as a new `DRTAHashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use drtahash::DRTAHashSet;
    ///
    /// let a: DRTAHashSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: DRTAHashSet<_> = vec![2, 3, 4].into_iter().collect();
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &DRTAHashSet<T, S>) -> DRTAHashSet<T, S> {
        DRTAHashSet(self.0.bitand(&rhs.0))
    }
}

impl<T, S> BitXor<&DRTAHashSet<T, S>> for &DRTAHashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = DRTAHashSet<T, S>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `DRTAHashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use drtahash::DRTAHashSet;
    ///
    /// let a: DRTAHashSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: DRTAHashSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a ^ &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 4, 5];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitxor(self, rhs: &DRTAHashSet<T, S>) -> DRTAHashSet<T, S> {
        DRTAHashSet(self.0.bitxor(&rhs.0))
    }
}

impl<T, S> Sub<&DRTAHashSet<T, S>> for &DRTAHashSet<T, S>
where
    T: Eq + Hash + Clone,
    S: BuildHasher + Default,
{
    type Output = DRTAHashSet<T, S>;

    /// Returns the difference of `self` and `rhs` as a new `DRTAHashSet<T, S>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use drtahash::DRTAHashSet;
    ///
    /// let a: DRTAHashSet<_> = vec![1, 2, 3].into_iter().collect();
    /// let b: DRTAHashSet<_> = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a - &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn sub(self, rhs: &DRTAHashSet<T, S>) -> DRTAHashSet<T, S> {
        DRTAHashSet(self.0.sub(&rhs.0))
    }
}

impl<T, S> Debug for DRTAHashSet<T, S>
where
    T: Debug,
    S: BuildHasher,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl<T, S> FromIterator<T> for DRTAHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher + Default,
{
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> DRTAHashSet<T, S> {
        DRTAHashSet(HashSet::from_iter(iter))
    }
}

impl<'a, T, S> IntoIterator for &'a DRTAHashSet<T, S> {
    type Item = &'a T;
    type IntoIter = hash_set::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

impl<T, S> IntoIterator for DRTAHashSet<T, S> {
    type Item = T;
    type IntoIter = hash_set::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, S> Extend<T> for DRTAHashSet<T, S>
where
    T: Eq + Hash,
    S: BuildHasher,
{
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<'a, T, S> Extend<&'a T> for DRTAHashSet<T, S>
where
    T: 'a + Eq + Hash + Copy,
    S: BuildHasher,
{
    #[inline]
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<T> Default for DRTAHashSet<T, RandomState> {
    /// Creates an empty `DRTAHashSet<T, S>` with the `Default` value for the hasher.
    #[inline]
    fn default() -> DRTAHashSet<T, RandomState> {
        DRTAHashSet(HashSet::default())
    }
}

#[cfg(feature = "serde")]
impl<T> Serialize for DRTAHashSet<T>
where
    T: Serialize + Eq + Hash,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.deref().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> Deserialize<'de> for DRTAHashSet<T>
where
    T: Deserialize<'de> + Eq + Hash,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let hash_set = HashSet::deserialize(deserializer);
        hash_set.map(|hash_set| Self(hash_set))
    }
}

#[cfg(all(test, feature = "serde"))]
mod test {
    use super::*;

    #[test]
    fn test_serde() {
        let mut set = DRTAHashSet::new();
        set.insert("for".to_string());
        set.insert("bar".to_string());
        let serialization = serde_json::to_string(&set).unwrap();
        let deserialization: DRTAHashSet<String> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(deserialization, set);
    }
}
