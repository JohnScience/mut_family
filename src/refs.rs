use sealed::sealed;

/// The type that represents shared references as opposed to mutable references.
pub struct SharedRefFamily;
/// The type that represents mutable references as opposed to shared references.
pub struct MutRefFamily;

/// The trait whose implementors represent various reference types.
#[sealed]
pub trait RefMutFamily: Sized {
    /// The generic associated type ([GAT]) that allows to constuct types of references
    ///
    /// [GAT]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
    type Target<'a, T>: Ref<'a, T, Self, Pointee = T, RefMutFamily = Self>
    where
        T: 'a;
}

#[sealed]
impl RefMutFamily for SharedRefFamily {
    type Target<'a,T> = &'a T
    where
        T: 'a;
}

#[sealed]
impl RefMutFamily for MutRefFamily {
    type Target<'a,T> = &'a mut T
    where
        T: 'a;
}

/// The trait whose implementors represent various reference types.
pub trait Ref<'a, T, M>
where
    M: RefMutFamily,
{
    /// The type that the reference points to.
    type Pointee;
    /// The type that represents the mutability of the reference.
    type RefMutFamily;
}

impl<'a, T, M> Ref<'a, T, M> for &'a T
where
    M: RefMutFamily,
{
    type Pointee = T;
    type RefMutFamily = M;
}

impl<'a, T, M> Ref<'a, T, M> for &'a mut T
where
    M: RefMutFamily,
{
    type Pointee = T;
    type RefMutFamily = M;
}
