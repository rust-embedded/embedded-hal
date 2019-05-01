//! Generic mutex traits
//!
//! The traits in this module allow code to be generic over the mutex type used.
//! The types implementing these traits must guarantee that access is always
//! exclusive, even for a `RoMutex`.
//!
//! ## Example Implementation
//!
//! ### Std
//! The following code snippet is a possible implementation of the mutex traits
//! for `std`'s `Mutex`:
//! ```
//! # use embedded_hal::mutex;
//! pub struct StdMutex<T>(std::sync::Mutex<T>);
//!
//! impl<T> mutex::Mutex<T> for StdMutex<T> {
//!     type CreationError = void::Void;
//!
//!     fn create(v: T) -> Result<Self, Self::CreationError> {
//!         Ok(StdMutex(std::sync::Mutex::new(v)))
//!     }
//! }
//!
//! impl<T> mutex::RwMutex<T> for StdMutex<T> {
//!     type Error = ();
//!
//!     fn lock_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Result<R, Self::Error> {
//!         // Erase the error type in this example for simplicity
//!         let mut v = self.0.try_lock().or(Err(()))?;
//!         Ok(f(&mut v))
//!     }
//! }
//!
//! // RoMutex is implemented automatically by adding the following:
//! impl<T> mutex::default::DefaultRo for StdMutex<T> { }
//!
//! # fn main() {
//! // Make use of the mutex:
//! use embedded_hal::mutex::{Mutex, RoMutex, RwMutex};
//!
//! let m = StdMutex::create(123).unwrap();
//! m.lock_mut(|v| {
//!     assert_eq!(*v, 123);
//!     *v = 321;
//! }).unwrap();
//! m.lock(|v|
//!     assert_eq!(*v, 321)
//! ).unwrap();
//! # }
//! ```
//!
//! ### `cortex-m`
//! `cortex-m` uses the `bare-metal` mutex type, an implementation might look
//! like this:
//! ```
//! # use embedded_hal::mutex;
//! pub struct CortexMMutex<T>(cortex_m::interrupt::Mutex<T>);
//!
//! impl<T> mutex::Mutex<T> for CortexMMutex<T> {
//!     type CreationError = void::Void;
//!
//!     fn create(v: T) -> Result<Self, Self::CreationError> {
//!         Ok(CortexMMutex(cortex_m::interrupt::Mutex::new(v)))
//!     }
//! }
//!
//! impl<T> mutex::RoMutex<T> for CortexMMutex<T> {
//!     type Error = void::Void;
//!
//!     fn lock<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, Self::Error> {
//!         Ok(cortex_m::interrupt::free(|cs| {
//!             let v = self.0.borrow(cs);
//!             f(v)
//!         }))
//!     }
//! }
//!
//! // Implement RwMutex for CortexMMutex<RefCell<T>> automatically:
//! impl<T> mutex::default::RefCellRw for CortexMMutex<T> { }
//!
//! // Add a type alias for convenience
//! type CortexMMutexRw<T> = CortexMMutex<core::cell::RefCell<T>>;
//! #
//! # // Check that implementations actually exist
//! # fn is_mu<T, M: mutex::Mutex<T>>() { }
//! # fn is_ro<T, M: mutex::RoMutex<T>>() { }
//! # fn is_rw<T, M: mutex::RwMutex<T>>() { }
//! #
//! # is_mu::<(), CortexMMutex<()>>();
//! # is_ro::<(), CortexMMutex<()>>();
//! # is_mu::<(), CortexMMutexRw<()>>();
//! # is_rw::<(), CortexMMutexRw<()>>();
//! ```

/// A generic mutex abstraction.
///
/// This trait by itself is not that useful, `RoMutex` and `RwMutex` have this
/// as their common requirement.  See the module root for more info.
#[cfg(feature = "unproven")]
pub trait Mutex<T>: Sized {
    /// Creation Error
    type CreationError;

    /// Create a new mutex of this type.
    fn create(v: T) -> Result<Self, Self::CreationError>;
}

/// A read-only (immutable) mutex.
///
/// This means, the value it shares is immutable, but only a single context may
/// have exclusive access.
///
/// `RwMutex`es can implement this trait automatically using
/// ```
/// # use embedded_hal::mutex;
/// # struct MyMutex<T>(T);
/// impl<T> mutex::default::DefaultRo for MyMutex<T> { }
/// ```
#[cfg(feature = "unproven")]
pub trait RoMutex<T>: Mutex<T> {
    /// Locking error
    type Error;

    /// Lock the mutex for the duration of a closure
    ///
    /// `lock` will call a closure with an immutable reference to the unlocked
    /// mutex's value.
    fn lock<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, Self::Error>;
}

/// A read-write (mutable) mutex.
///
/// This mutex type is similar to the Mutex from `std`.  When you lock it, you
/// get access to a mutable reference.
///
/// This trait can automatically be implemented for `RoMutex<RefCell<T>>` by using
/// ```
/// # use embedded_hal::mutex;
/// # struct MyMutex<T>(T);
/// impl<T> mutex::default::RefCellRw for MyMutex<T> { }
/// ```
#[cfg(feature = "unproven")]
pub trait RwMutex<T>: Mutex<T> {
    /// Locking error
    type Error;

    /// Lock the mutex for the duration of a closure
    ///
    /// `lock_mut` will call a closure with a mutable reference to the unlocked
    /// mutex's value.
    fn lock_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Result<R, Self::Error>;
}

/// Blanket implementations for `RoMutex` and `RwMutex`
///
/// Any `RwMutex` can trivially implement `RoMutex` as well.  To enable this,
/// add a line like
/// ```
/// # use embedded_hal::mutex;
/// # struct MyMutex<T>(T);
/// impl<T> mutex::default::DefaultRo for MyMutex<T> { }
/// ```
/// to your mutex definition.
///
/// Similarly, a `RoMutex` and a `RefCell` can be used to implement `RwMutex`.
/// The blanket implementation can be enabled using
/// ```
/// # use embedded_hal::mutex;
/// # struct MyMutex<T>(T);
/// impl<T> mutex::default::RefCellRw for MyMutex<T> { }
/// ```
#[cfg(feature = "unproven")]
pub mod default {
    use super::*;
    use core::cell::RefCell;

    /// Marker trait to enable the default `RoMutex` implementation.
    ///
    /// Your mutex type must implement `RwMutex` for this to have an effect!
    pub trait DefaultRo {}

    // Blanket impl:
    //   Every read-write mutex is also read-only.  Don't confuse this with an
    //   RwLock where multiple reads are allowed simultaneously!
    impl<T, M> RoMutex<T> for M
    where
        M: DefaultRo + RwMutex<T>,
    {
        type Error = <M as RwMutex<T>>::Error;

        fn lock<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, Self::Error> {
            self.lock_mut(|v| f(v))
        }
    }

    /// Marker trait to enable an implementation of `RwMutex` using `RefCell`s
    ///
    /// Your mutex type must implement `RoMutex` for this to have an effect!
    pub trait RefCellRw {}

    // Blanket impl:
    //   You can use a RefCell to make an RoMutex read-write.  This means, the
    //   bare-metal mutex type (which is read-only) can easily be used for
    //   creating a read-write mutex!
    //
    //   This is the wrapper for creation of such a mutex.
    impl<T, M> Mutex<T> for M
    where
        M: RefCellRw + RoMutex<RefCell<T>>,
    {
        type CreationError = <M as Mutex<RefCell<T>>>::CreationError;

        fn create(v: T) -> Result<Self, Self::CreationError> {
            <M as Mutex<RefCell<T>>>::create(RefCell::new(v))
        }
    }

    // Blanket impl:
    //   You can use a RefCell to make an RoMutex read-write.  This means, the
    //   bare-metal mutex type (which is read-only) can easily be used for
    //   creating a read-write mutex!
    //
    //   This is the actual RwMutex implementation.
    impl<T, M> RwMutex<T> for M
    where
        M: RefCellRw + RoMutex<RefCell<T>> + Mutex<T>,
    {
        type Error = <M as RoMutex<RefCell<T>>>::Error;

        fn lock_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> Result<R, Self::Error> {
            self.lock(|v| f(&mut v.borrow_mut()))
        }
    }
}
