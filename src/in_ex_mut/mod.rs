use crate::refs::*;

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
    /// The type with a generic associated type (GAT) that allows to constuct types of references
    /// (e.g. `&'a Cell<T>` or `&'a mut T`) that allow mutation of the wrapped value unsafely.
    ///
    /// In the absence of associated type aliases, this type is most commonly used this way:
    /// `<Self::RefMutFamilyAllowingMutationUnsafely as RefMutFamily>::Target<'a, Self::Target<T>>`.
    type RefMutFamilyAllowingMutationUnsafely: RefMutFamily;
    /// Constructs a new instance of the parameterized type-wrapper.
    fn new<T>(value: T) -> Self::Target<T>;
    /// Unwraps the instance of the parameterized type-wrapper.
    fn into_inner<T>(target: Self::Target<T>) -> T;
    /// Returns a mutable reference to the wrapped value.
    // TODO: consider how to better support OnceCell
    fn get_mut<T>(mut_ref: &mut Self::Target<T>) -> &mut T;
    /// Returns a mutable raw pointer to the wrapped value. Check the safety requirements of the
    /// implementors.
    fn as_ptr<T>(
        ref_: <Self::RefMutFamilyAllowingMutationUnsafely as RefMutFamily>::Ref<
            '_,
            Self::Target<T>,
        >,
    ) -> *mut T;
}

/// The trait whose implementors represent various interior mutability wrappers that allow
/// cloning of the wrapped value by shared reference if the type is [`Copy`].
///
/// For example, [`core::cell::Cell`] allows cloning of the wrapped value by shared reference
/// only if the type is [`Copy`].
pub trait CloneCopyableInner: MutFamily {
    /// Returns a copy of the wrapped value.
    /// 
    /// # Panics
    /// 
    /// May panic for some implementors, notably [`RefCellFamily`].
    fn clone_copyable_inner<T>(ref_: &Self::Target<T>) -> T
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
pub trait CloneInner: CloneCopyableInner {
    /// Returns a clone of the wrapped value.
    /// 
    /// # Panics
    /// 
    /// May panic for some implementors, notably [`RefCellFamily`].
    fn clone_inner<T>(ref_: &Self::Target<T>) -> T
    where
        T: Clone;
}

/// The trait whose implementors represent various interior mutability wrappers that allow
/// mutation of the wrapped value though "some kind of reference". More specifically, the
/// implementor should specify whether the reference has to be shared or mutable by providing
/// the corresponding [`Set::RefMutFamilyAllowingMutation`] associated type.
pub trait Set: MutFamily {
    /// The type with a generic associated type (GAT) that allows to constuct types of references
    /// (e.g. `&'a Cell<T>` or `&'a mut T`) that allow mutation of the wrapped value safely.
    ///
    /// In the absence of associated type aliases, this type is most commonly used this way:
    /// `<Self::RefMutFamilyAllowingMutation as RefMutFamily>::Target<'a, Self::Target<T>>`.
    type RefMutFamilyAllowingMutation: RefMutFamily;
    /// Sets the wrapped value to the specified one.
    /// 
    /// # Panics
    /// 
    /// May panic for some implementors, notably [`RefCellFamily`].
    fn set<T>(
        ref_: <Self::RefMutFamilyAllowingMutation as RefMutFamily>::Ref<'_, Self::Target<T>>,
        value: T,
    );
}

impl MutFamily for IdentityFamily {
    type Target<T> = T;
    type RefMutFamilyAllowingMutationUnsafely = MutRefFamily;

    fn new<T>(value: T) -> T {
        value
    }

    fn into_inner<T>(target: T) -> T {
        target
    }

    fn as_ptr<T>(ref_: &mut T) -> *mut T {
        ref_ as *mut T
    }

    fn get_mut<T>(mut_ref: &mut T) -> &mut T {
        mut_ref
    }
}

impl MutFamily for UnsafeCellFamily {
    type Target<T> = core::cell::UnsafeCell<T>;

    type RefMutFamilyAllowingMutationUnsafely = SharedRefFamily;

    fn new<T>(value: T) -> Self::Target<T> {
        core::cell::UnsafeCell::new(value)
    }

    fn into_inner<T>(target: Self::Target<T>) -> T {
        target.into_inner()
    }

    fn as_ptr<T>(ref_: &core::cell::UnsafeCell<T>) -> *mut T {
        ref_.get()
    }

    fn get_mut<T>(mut_ref: &mut core::cell::UnsafeCell<T>) -> &mut T {
        mut_ref.get_mut()
    }
}

impl MutFamily for CellFamily {
    type Target<T> = core::cell::Cell<T>;
    type RefMutFamilyAllowingMutationUnsafely = SharedRefFamily;

    fn new<T>(value: T) -> Self::Target<T> {
        core::cell::Cell::new(value)
    }

    fn into_inner<T>(target: Self::Target<T>) -> T {
        target.into_inner()
    }

    fn as_ptr<T>(ref_: &core::cell::Cell<T>) -> *mut T {
        ref_.as_ptr()
    }

    fn get_mut<T>(mut_ref: &mut Self::Target<T>) -> &mut T {
        mut_ref.get_mut()
    }
}

impl MutFamily for RefCellFamily {
    type Target<T> = core::cell::RefCell<T>;
    type RefMutFamilyAllowingMutationUnsafely = SharedRefFamily;

    fn new<T>(value: T) -> Self::Target<T> {
        core::cell::RefCell::new(value)
    }

    fn into_inner<T>(target: Self::Target<T>) -> T {
        target.into_inner()
    }

    fn as_ptr<T>(ref_: &core::cell::RefCell<T>) -> *mut T {
        ref_.as_ptr()
    }

    fn get_mut<T>(mut_ref: &mut core::cell::RefCell<T>) -> &mut T {
        mut_ref.get_mut()
    }
}

impl CloneCopyableInner for IdentityFamily {
    fn clone_copyable_inner<T>(ref_: &T) -> T
    where
        T: Clone + Copy,
    {
        *ref_
    }
}

impl CloneCopyableInner for CellFamily {
    fn clone_copyable_inner<T>(ref_: &core::cell::Cell<T>) -> T
    where
        T: Clone + Copy,
    {
        ref_.get()
    }
}

impl CloneCopyableInner for RefCellFamily {
    fn clone_copyable_inner<T>(ref_: &core::cell::RefCell<T>) -> T
    where
        T: Clone + Copy,
    {
        *ref_.borrow()
    }
}

impl CloneInner for IdentityFamily {
    fn clone_inner<T>(ref_: &T) -> T
    where
        T: Clone,
    {
        ref_.clone()
    }
}

impl CloneInner for RefCellFamily {
    fn clone_inner<T>(ref_: &core::cell::RefCell<T>) -> T
    where
        T: Clone,
    {
        ref_.borrow().clone()
    }
}

impl Set for IdentityFamily {
    type RefMutFamilyAllowingMutation = MutRefFamily;
    fn set<T>(ref_: &mut T, value: T) {
        *ref_ = value;
    }
}

impl Set for CellFamily {
    type RefMutFamilyAllowingMutation = SharedRefFamily;
    fn set<T>(ref_: &core::cell::Cell<T>, value: T) {
        ref_.set(value);
    }
}

impl Set for RefCellFamily {
    type RefMutFamilyAllowingMutation = SharedRefFamily;
    fn set<T>(ref_: &core::cell::RefCell<T>, value: T) {
        *ref_.borrow_mut() = value;
    }
}
