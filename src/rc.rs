use alloc::borrow::ToOwned;
use alloc::rc::Rc;
use core::cmp::Ordering;
use core::fmt::Formatter;

/// A wrapper around an [`Rc`] that clones when mutably borrowed if it is not unique.
///
/// Like [`Rc`], this is not thread-safe.
///
/// # Examples
///
/// See the crate documentation for examples.
#[derive(Clone, Default, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct CombRc<T: Clone> {
    inner: Rc<T>
}

impl<T: Clone> CombRc<T> {
    /// Constructs a `CombRc`.
    #[inline]
    pub fn new(what: T) -> CombRc<T> {
        Self {
            inner: Rc::new(what)
        }
    }

    /// Constructs a `CombRc` from an already created `Rc`.
    #[inline]
    pub fn from_rc(what: Rc<T>) -> CombRc<T> {
        Self {
            inner: what
        }
    }

    /// Clones the inner value stored in the `CombRc`, returning a unique clone of it.
    #[inline]
    pub fn clone_unique(what: &CombRc<T>) -> CombRc<T> {
        Self::new(what.inner.as_ref().clone())
    }

    /// Attempt to get the inner value inside of the `CombRc`.
    ///
    /// If this is a unique reference, the inner value will be moved. Otherwise, the reference will
    /// be re-returned.
    #[inline]
    pub fn try_unwrap(what: CombRc<T>) -> Result<T, Self> {
        Rc::try_unwrap(what.inner).map_err(Self::from_rc)
    }

    /// Try to get the inner value inside of the `CombArc` or clone otherwise.
    ///
    /// If this is a unique reference, the inner value will be moved. Otherwise, it will be cloned.
    #[inline]
    pub fn make_inner(what: CombRc<T>) -> T {
        Rc::try_unwrap(what.inner).unwrap_or_else(|e| T::to_owned(e.as_ref()))
    }

    /// Get the inner `Rc` value.
    #[inline]
    pub fn get_rc(what: &CombRc<T>) -> &Rc<T> {
        &what.inner
    }
}

impl<T: Clone + PartialEq> PartialEq<T> for CombRc<T> {
    fn eq(&self, other: &T) -> bool {
        Rc::as_ref(&self.inner) == other
    }
}

impl<T: Clone + PartialOrd> PartialOrd<T> for CombRc<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Rc::as_ref(&self.inner).partial_cmp(other)
    }
}

impl<T: Clone> From<CombRc<T>> for Rc<T> {
    fn from(value: CombRc<T>) -> Self {
        value.inner
    }
}

impl<T: Clone> From<Rc<T>> for CombRc<T> {
    fn from(value: Rc<T>) -> Self {
        CombRc::from_rc(value)
    }
}

impl<T: Clone> core::ops::Deref for CombRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<T: Clone> core::ops::DerefMut for CombRc<T> {
    /// If the inner [`Rc`] has no strong references, get a mutable reference. Otherwise, clone the
    /// [`Rc`].
    /// 
    /// This just calls [`Rc::make_mut`] on the inner value.
    fn deref_mut(&mut self) -> &mut T {
        Rc::make_mut(&mut self.inner)
    }
}

impl<T: core::fmt::Display + Clone> core::fmt::Display for CombRc<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Rc::as_ref(&self.inner).fmt(f)
    }
}
