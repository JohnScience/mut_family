use sealed::sealed;

/// The type that represents shared references as opposed to mutable references.
pub struct SharedRefFamily;
/// The type that represents mutable references as opposed to shared references.
pub struct MutRefFamily;

/// The trait whose two implementors represent either mutability ([`MutRefFamily`])
/// or sharedness ([`SharedRefFamily`]) of references.
#[sealed]
pub trait RefMutFamily: Sized {
    /// The generic associated type ([GAT]) that allows to constuct types of references
    ///
    /// [GAT]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
    type Ref<'a, T>: Ref<'a, T, Self, Pointee = T, RefMutFamily = Self>
    where
        T: 'a;
}

#[sealed]
impl RefMutFamily for SharedRefFamily {
    type Ref<'a,T> = &'a T
    where
        T: 'a;
}

#[sealed]
impl RefMutFamily for MutRefFamily {
    type Ref<'a,T> = &'a mut T
    where
        T: 'a;
}

/// The trait whose implementors represent any of the two reference types.
pub trait Ref<'a, T, M>: From<M::Ref<'a, T>> + Into<M::Ref<'a, T>>
where
    M: RefMutFamily,
    T: 'a,
{
    /// The type that the reference points to.
    type Pointee;
    /// The type that represents the mutability of the reference.
    type RefMutFamily;

    /// The method that returns a concrete shared reference. This is method is needed
    /// in generic context.
    fn as_ref(&self) -> &T;
}

impl<'a, T> Ref<'a, T, SharedRefFamily> for &'a T
{
    type Pointee = T;
    type RefMutFamily = SharedRefFamily;

    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T> Ref<'a, T, MutRefFamily> for &'a mut T
{
    type Pointee = T;
    type RefMutFamily = MutRefFamily;

    fn as_ref(&self) -> &T {
        self
    }
}
