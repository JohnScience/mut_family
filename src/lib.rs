#![doc = include_str!("../README.md")]
#![no_std]

mod sealed {
    /// [Sealed trait] for either shared (e.g. `&'a T`) or mutable (e.g. `&'a mut`) references.
    /// 
    /// [Sealed trait]: https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed
    pub trait ArbMutRef<'a, T> {}

    impl<'a, T> ArbMutRef<'a,T> for &'a T {}
    impl<'a, T> ArbMutRef<'a,T> for &'a mut T {}
}

/// The type that represents the degenerate (nonexistent) mutability wrapper.
/// 
/// Learn more about interior and exterior mutability [here].
/// 
/// [here]: https://doc.rust-lang.org/reference/interior-mutability.html
pub struct IdentityFamily;

/// The type that represents the [`core::cell::UnsafeCell`] interior mutability wrapper.
/// 
/// Learn more about interior and exterior mutability [here].
/// 
/// [here]: https://doc.rust-lang.org/reference/interior-mutability.html
pub struct UnsafeCellFamily;

/// The type that represents the [`core::cell::Cell`] interior mutability wrapper.
/// 
/// Learn more about interior and exterior mutability [here].
/// 
/// [here]: https://doc.rust-lang.org/reference/interior-mutability.html
pub struct CellFamily;

/// The type that represents the [`core::cell::RefCell`] interior mutability wrapper.
/// 
/// Learn more about interior and exterior mutability [here].
/// 
/// [here]: https://doc.rust-lang.org/reference/interior-mutability.html
pub struct RefCellFamily;

/// The trait whose implementors represent various interior mutability wrappers. The implementor
/// for the absence of wrapper is [`IdentityFamily`].
///
/// *Note: this trait doesn't cover all interior mutability wrappers, e.g. [`core::cell::LazyCell`]*.
/// 
/// Learn about interior and exterior mutability [here].
/// 
/// [here]: https://doc.rust-lang.org/reference/interior-mutability.html
pub trait MutFamily {
    /// The generic associated type (GAT) that allows to wrap types in various interior mutability
    /// wrappers, if wrap at all.
    type Target<T>;
    /// The generic associated type (GAT) that allows to constuct types of references
    /// (e.g. `&'a Cell<T>` or `&'a mut T`) that allow mutation of the wrapped value.
    type RefAllowingMutation<'a, T>: sealed::ArbMutRef<'a, Self::Target<T>>
    where
        T: 'a,
        Self::Target<T>: 'a;
    /// Constructs a new instance of the parameterized type-wrapper.
    fn new<T>(value: T) -> Self::Target<T>;
    /// Unwraps the instance of the parameterized type-wrapper.
    fn into_inner<T>(target: Self::Target<T>) -> T;
    /// Returns a mutable reference to the wrapped value.
    // TODO: consider how to better support OnceCell
    fn get_mut<'a, T>(mut_ref: &'a mut Self::Target<T>) -> &'a mut T;
    /// Returns a mutable raw pointer to the wrapped value. Check the safety requirements of the
    /// implementors.
    fn as_ptr<'a, T>(ref_: Self::RefAllowingMutation<'a,T>) -> *mut T;
}

/// The trait whose implementors represent various interior mutability wrappers that allow
/// cloning of the wrapped value by shared reference if the type is [`Copy`].
/// 
/// For example, [`core::cell::Cell`] allows cloning of the wrapped value by shared reference
/// only if the type is [`Copy`].
pub trait CloneCopyableInner: MutFamily
{
    /// Returns a copy of the wrapped value.
    fn clone_copyable_inner<'a, T>(ref_: &'a Self::Target<T>) -> T
    where
        T: Clone + Copy;
}

/// The trait whose implementors represent various interior mutability wrappers that allow
/// cloning of the wrapped value by shared reference.
/// 
/// Unlike [`CloneCopyableInner`], this trait doesn't require the wrapped type to be [`Copy`]
/// to allow cloning.
/// 
/// For example, [`core::cell::RefCell`] allows cloning of the wrapped value by shared reference
/// regardless of whether the type is [`Copy`].
pub trait CloneInner: CloneCopyableInner
{
    /// Returns a clone of the wrapped value.
    fn clone_inner<'a, T>(ref_: &'a Self::Target<T>) -> T
    where
        T: Clone;
}

impl MutFamily for IdentityFamily {
    type Target<T> = T;
    type RefAllowingMutation<'a, T> = &'a mut T
    where
        T: 'a;
    
    fn new<T>(value: T) -> T {
        value
    }

    fn into_inner<T>(target: T) -> T {
        target
    }

    fn as_ptr<'a, T>(ref_: &'a mut T) -> *mut T {
        ref_ as *mut T
    }
    
    fn get_mut<'a, T>(mut_ref: &'a mut T) -> &'a mut T {
        mut_ref
    }
}

impl MutFamily for UnsafeCellFamily {
    type Target<T> = core::cell::UnsafeCell<T>;
    
    type RefAllowingMutation<'a, T> = &'a core::cell::UnsafeCell<T>
    where
        T: 'a;

    fn new<T>(value: T) -> Self::Target<T> {
        core::cell::UnsafeCell::new(value)
    }

    fn into_inner<T>(target: Self::Target<T>) -> T {
        target.into_inner()
    }
    
    fn as_ptr<'a, T>(ref_: &'a core::cell::UnsafeCell<T>) -> *mut T {
        ref_.get()
    }

    fn get_mut<'a, T>(mut_ref: &'a mut core::cell::UnsafeCell<T>) -> &'a mut T {
        mut_ref.get_mut()
    }
}

impl MutFamily for CellFamily {
    type Target<T> = core::cell::Cell<T>;
    type RefAllowingMutation<'a, T> = &'a core::cell::Cell<T>
    where
        T: 'a;

    fn new<T>(value: T) -> Self::Target<T> {
        core::cell::Cell::new(value)
    }

    fn into_inner<T>(target: Self::Target<T>) -> T {
        target.into_inner()
    }

    fn as_ptr<'a, T>(ref_: &'a core::cell::Cell<T>) -> *mut T {
        ref_.as_ptr()
    }

    fn get_mut<'a, T>(mut_ref: &'a mut Self::Target<T>) -> &'a mut T {
        mut_ref.get_mut()
    }
}

impl MutFamily for RefCellFamily {
    type Target<T> = core::cell::RefCell<T>;
    type RefAllowingMutation<'a, T> = &'a core::cell::RefCell<T>
    where
        T: 'a;

    fn new<T>(value: T) -> Self::Target<T> {
        core::cell::RefCell::new(value)
    }

    fn into_inner<T>(target: Self::Target<T>) -> T {
        target.into_inner()
    }

    fn as_ptr<'a, T>(ref_: &'a core::cell::RefCell<T>) -> *mut T {
        ref_.as_ptr()
    }

    fn get_mut<'a, T>(mut_ref: &'a  mut core::cell::RefCell<T>) -> &'a mut T {
        mut_ref.get_mut()
    }
}

impl CloneCopyableInner for IdentityFamily {
    fn clone_copyable_inner<'a, T>(ref_: &'a T) -> T
    where
        T: Clone + Copy,
    {
        ref_.clone()
    }
}

impl CloneCopyableInner for CellFamily {
    fn clone_copyable_inner<'a, T>(ref_: &'a core::cell::Cell<T>) -> T
    where
        T: Clone + Copy,
    {
        ref_.get()
    }
}

impl CloneCopyableInner for RefCellFamily {
    fn clone_copyable_inner<'a, T>(ref_: &'a core::cell::RefCell<T>) -> T
    where
        T: Clone + Copy,
    {
        ref_.borrow().clone()
    }
}

impl CloneInner for IdentityFamily {
    fn clone_inner<'a, T>(ref_: &'a T) -> T
    where
        T: Clone,
    {
        ref_.clone()
    }
}

impl CloneInner for RefCellFamily {
    fn clone_inner<'a, T>(ref_: &'a core::cell::RefCell<T>) -> T
    where
        T: Clone,
    {
        ref_.borrow().clone()
    }
}