use alloc::borrow::ToOwned;
use alloc::sync::Arc;
use core::cmp::Ordering;
use core::fmt::Formatter;

/// A wrapper around an [`Arc`] that clones when mutably borrowed if it is not unique.
///
/// Like [`Arc`], this value is thread-safe.
///
/// # Examples
///
/// See the crate documentation for examples.
#[derive(Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct CombArc<T: Clone> {
    inner: Arc<T>
}

impl<T: Clone> CombArc<T> {
    /// Constructs a `CombArc`.
    #[inline]
    pub fn new(what: T) -> CombArc<T> {
        Self {
            inner: Arc::new(what)
        }
    }

    /// Constructs a `CombArc` from an already created `Arc`.
    #[inline]
    pub fn from_arc(what: Arc<T>) -> CombArc<T> {
        Self {
            inner: what
        }
    }

    /// Clones the inner value stored in the `CombArc`, returning a unique clone of it.
    #[inline]
    pub fn clone_unique(what: &CombArc<T>) -> CombArc<T> {
        Self::new(what.inner.as_ref().clone())
    }

    /// Attempt to get the inner value inside of the `CombArc`.
    ///
    /// If this is a unique reference, the inner value will be moved. Otherwise, the reference will
    /// be re-returned.
    #[inline]
    pub fn try_unwrap(what: CombArc<T>) -> Result<T, Self> {
        Arc::try_unwrap(what.inner).map_err(Self::from_arc)
    }

    /// Try to get the inner value inside of the `CombArc` or clone otherwise.
    ///
    /// If this is a unique reference, the inner value will be moved. Otherwise, it will be cloned.
    #[inline]
    pub fn make_inner(what: CombArc<T>) -> T {
        Arc::try_unwrap(what.inner).unwrap_or_else(|e| T::to_owned(e.as_ref()))
    }

    /// Get the inner `Arc` value.
    #[inline]
    pub fn get_arc(what: &CombArc<T>) -> &Arc<T> {
        &what.inner
    }
}

impl<T: Clone + PartialEq> PartialEq<T> for CombArc<T> {
    fn eq(&self, other: &T) -> bool {
        Arc::as_ref(&self.inner) == other
    }
}

impl<T: Clone + PartialOrd> PartialOrd<T> for CombArc<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Arc::as_ref(&self.inner).partial_cmp(other)
    }
}

impl<T: Clone> From<CombArc<T>> for Arc<T> {
    fn from(value: CombArc<T>) -> Self {
        value.inner
    }
}

impl<T: Clone> From<Arc<T>> for CombArc<T> {
    fn from(value: Arc<T>) -> Self {
        CombArc::from_arc(value)
    }
}

impl<T: Clone> core::ops::Deref for CombArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<T: Clone> core::ops::DerefMut for CombArc<T> {
    /// If the inner [`Arc`] has no strong references, get a mutable reference. Otherwise, clone the
    /// [`Arc`].
    ///
    /// This just calls [`Arc::make_mut`] on the inner value.
    fn deref_mut(&mut self) -> &mut T {
        Arc::make_mut(&mut self.inner)
    }
}

impl<T: core::fmt::Display + Clone> core::fmt::Display for CombArc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Arc::as_ref(&self.inner).fmt(f)
    }
}
