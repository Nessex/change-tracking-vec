use core::{fmt, slice};
use std::borrow::Cow;
use std::vec::{IntoIter, Splice};
use std::collections::TryReserveError;
use std::hash::{Hash, Hasher};
use std::mem::MaybeUninit;
use std::ops;
use std::ops::{Index, IndexMut, RangeBounds};
use std::slice::SliceIndex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Drain;

pub struct ChangeTrackingVec<T> {
    revision: AtomicUsize,
    checked_revision: usize,
    inner: Vec<T>,
}

impl<T> ChangeTrackingVec<T> {
    #[inline]
    fn n(inner: Vec<T>) -> Self {
        Self {
            revision: AtomicUsize::new(0),
            checked_revision: 0,
            inner,
        }
    }

    #[inline]
    pub fn new() -> Self {
        Self::n(Vec::new())
    }

    #[inline]
    pub fn count(&self) {
        self.revision.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn revision(&self) -> usize {
        self.revision.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn changed(&mut self) -> bool {
        let rev = self.revision.load(Ordering::Relaxed);
        let changed = rev != self.checked_revision;
        self.checked_revision = rev;

        changed
    }

    #[inline]
    pub fn inner(&self) -> &Vec<T> {
        self.inner.as_ref()
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        self.count();

        self.inner.as_mut()
    }

    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::n(Vec::with_capacity(capacity))
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
        Self::n(Vec::from_raw_parts(ptr, length, capacity))
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.inner_mut().reserve(additional)
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner_mut().reserve_exact(additional)
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner_mut().try_reserve(additional)
    }

    #[inline]
    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner_mut().try_reserve_exact(additional)
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.inner_mut().shrink_to_fit()
    }

    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.inner_mut().shrink_to(min_capacity)
    }

    #[inline]
    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.into_inner().into_boxed_slice()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.inner_mut().truncate(len)
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.inner()
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner_mut()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.inner().as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.inner_mut().as_mut_ptr()
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.inner.set_len(new_len)
    }

    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.inner_mut().swap_remove(index)
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.inner_mut().insert(index, element)
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.inner_mut().remove(index)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.inner_mut().retain(f)
    }

    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.inner_mut().dedup_by_key(key)
    }

    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        self.inner_mut().dedup_by(same_bucket)
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.inner_mut().push(value)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.inner_mut().pop()
    }

    #[inline]
    pub fn append(&mut self, other: &mut Self) {
        self.inner_mut().append(other.inner_mut())
    }

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, T>
    where
        R: RangeBounds<usize>,
    {
        self.inner_mut().drain(range)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.inner_mut().clear()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    // #[inline]
    // pub fn split_off(&mut self, at: usize) -> Vec<T>
    // where
    //     A: Clone,
    // {
    //     self.inner_mut().split_off(at)
    // }

    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        self.inner_mut().resize_with(new_len, f)
    }

    // #[inline]
    // pub fn leak<'a>(self) -> &'a mut [T]
    // where
    //     A: 'a,
    // {
    //     self.into_inner().leak()
    // }

    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        self.inner_mut().spare_capacity_mut()
    }
}

impl<T: Clone> ChangeTrackingVec<T> {
    pub fn resize(&mut self, new_len: usize, value: T) {
        self.inner_mut().resize(new_len, value)
    }

    pub fn extend_from_slice(&mut self, other: &[T]) {
        self.inner_mut().extend_from_slice(other)
    }

    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: RangeBounds<usize>,
    {
        self.inner_mut().extend_from_within(src)
    }
}

impl<T: PartialEq> ChangeTrackingVec<T> {
    #[inline]
    pub fn dedup(&mut self) {
        self.inner_mut().dedup()
    }
}

impl<T> ops::Deref for ChangeTrackingVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.inner.deref()
    }
}

impl<T> ops::DerefMut for ChangeTrackingVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.inner_mut().deref_mut()
    }
}

impl<T: Hash> Hash for ChangeTrackingVec<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T, I: SliceIndex<[T]>> Index<I> for ChangeTrackingVec<T> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.inner.index(index)
    }
}

impl<T, I: SliceIndex<[T]>> IndexMut<I> for ChangeTrackingVec<T> {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.inner_mut().index_mut(index)
    }
}

impl<T> FromIterator<T> for ChangeTrackingVec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::n(Vec::from_iter(iter))
    }
}

impl<T> IntoIterator for ChangeTrackingVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        self.inner.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a ChangeTrackingVec<T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> slice::Iter<'a, T> {
        self.inner.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ChangeTrackingVec<T> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> slice::IterMut<'a, T> {
        self.inner.iter_mut()
    }
}

impl<T> Extend<T> for ChangeTrackingVec<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.inner_mut().extend(iter)
    }
}

impl<T> ChangeTrackingVec<T> {
    #[inline]
    pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        self.inner_mut().splice(range, replace_with)
    }
}

impl<'a, T: Copy + 'a> Extend<&'a T> for ChangeTrackingVec<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.inner_mut().extend(iter)
    }
}

impl<T: PartialEq> PartialEq for ChangeTrackingVec<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner[..] == other.inner[..]
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.inner[..] != other.inner[..]
    }
}

impl<T: PartialOrd> PartialOrd for ChangeTrackingVec<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Eq> Eq for ChangeTrackingVec<T> {}
impl<T: Ord + PartialOrd> Ord for ChangeTrackingVec<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T> Default for ChangeTrackingVec<T> {
    fn default() -> Self {
        ChangeTrackingVec::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for ChangeTrackingVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T> AsRef<Vec<T>> for ChangeTrackingVec<T> {
    fn as_ref(&self) -> &Vec<T> {
        self.inner.as_ref()
    }
}

impl<T> AsMut<Vec<T>> for ChangeTrackingVec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self.inner_mut().as_mut()
    }
}

impl<T> AsRef<[T]> for ChangeTrackingVec<T> {
    fn as_ref(&self) -> &[T] {
        self.inner.as_ref()
    }
}

impl<T> AsMut<[T]> for ChangeTrackingVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        self.inner_mut().as_mut()
    }
}

impl<T: Clone> From<&[T]> for ChangeTrackingVec<T> {
    fn from(s: &[T]) -> Self {
        Self::n(Vec::from(s))
    }
}

impl<T: Clone> From<&mut [T]> for ChangeTrackingVec<T> {
    fn from(s: &mut [T]) -> Self {
        Self::n(Vec::from(s))
    }
}

impl<T, const N: usize> From<[T; N]> for ChangeTrackingVec<T> {
    fn from(s: [T; N]) -> Self {
        Self::n(Vec::from(s))
    }
}

impl<'a, T> From<Cow<'a, [T]>> for ChangeTrackingVec<T>
where
    [T]: ToOwned<Owned = Vec<T>>,
{
    fn from(s: Cow<'a, [T]>) -> Self {
        Self::n(Vec::from(s))
    }
}

impl<T> From<Box<[T]>> for ChangeTrackingVec<T> {
    fn from(s: Box<[T]>) -> Self {
        Self::n(Vec::from(s))
    }
}

impl<T> From<ChangeTrackingVec<T>> for Box<[T]> {
    fn from(v: ChangeTrackingVec<T>) -> Self {
        v.inner.into_boxed_slice()
    }
}

impl From<&str> for ChangeTrackingVec<u8> {
    fn from(s: &str) -> Self {
        Self::n(Vec::from(s))
    }
}

impl<T, const N: usize> TryFrom<ChangeTrackingVec<T>> for [T; N] {
    type Error = ChangeTrackingVec<T>;

    fn try_from(vec: ChangeTrackingVec<T>) -> Result<[T; N], ChangeTrackingVec<T>> {
        if vec.len() != N {
            return Err(vec);
        }

        vec.inner.try_into().map_err(|e| ChangeTrackingVec::n(e))
    }
}

impl<T: Clone> Clone for ChangeTrackingVec<T> {
    fn clone(&self) -> Self {
        Self::n(self.inner.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::ChangeTrackingVec;

    #[test]
    fn it_works() {
        let mut ct_vec = ChangeTrackingVec::new();
        assert_eq!(ct_vec.changed(), false);
        assert_eq!(ct_vec.revision(), 0);
        assert_eq!(ct_vec.changed(), false);
        assert_eq!(ct_vec.revision(), 0);

        ct_vec.push(1);
        assert_eq!(ct_vec.changed(), true);
        assert_eq!(ct_vec.revision(), 1);
        assert_eq!(ct_vec.changed(), false);
        assert_eq!(ct_vec.revision(), 1);

        let _ = ct_vec.pop();
        assert_eq!(ct_vec.changed(), true);
        assert_eq!(ct_vec.revision(), 2);

        let mut ct_vec_2 = ChangeTrackingVec::new();
        ct_vec_2.extend_from_slice(&mut [0, 1, 2, 3]);
        assert_eq!(ct_vec_2.changed(), true);
        assert_eq!(ct_vec_2.revision(), 1);

        ct_vec.append(&mut ct_vec_2);
        assert_eq!(ct_vec.changed(), true);
        assert_eq!(ct_vec.revision(), 3);
        assert_eq!(ct_vec_2.changed(), true);
        assert_eq!(ct_vec_2.revision(), 2);
    }
}
