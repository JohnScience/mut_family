use sealed::sealed;

/// The type that represents shared references as opposed to mutable references.
pub struct SharedRefFamily;
/// The type that represents mutable references as opposed to shared references.
pub struct MutRefFamily;

/// The trait whose two implementors represent either mutability ([`MutRefFamily`])
/// or sharedness ([`SharedRefFamily`]) of references.
/// 
/// # Safety
/// 
/// const [`RefMutFamily::IS_SHARED`] must be correct.
#[sealed]
pub unsafe trait RefMutFamily: Sized {
    /// Whether the reference is shared or mutable.
    const IS_SHARED: bool;

    /// The generic associated type ([GAT]) that allows to constuct types of references
    ///
    /// [GAT]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html#what-are-gats
    type Ref<'a, T>: Ref<'a, T, Self, Pointee = T, RefMutFamily = Self>
    where
        T: 'a;
    fn with<T, F, O>(ref_: &mut Self::Ref<'_, T>, f: F) -> O
    where
        F: FnOnce(&mut Self::Ref<'_, T>) -> O,
        T: 'static;

    fn as_ref<T,F,R>(ref_: &mut Self::Ref<'_, T>, f: F) -> R
    where
        F: FnOnce(&mut Self::Ref<'_, T>) -> R,
        T: 'static,
        R: Ref<'static, T, Self, Pointee = T, RefMutFamily = Self>;
}

pub union SomeRef<'a, T, M>
where
    M: RefMutFamily,
{
    shared: &'a T,
    mut_: &'a mut T,
    marker: core::marker::PhantomData<*const M>,
}

/// The trait whose implementors represent any reference of the two reference kinds.
#[sealed]
pub trait Ref<'a, T, M>: Sized
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

    fn into_someref(self) -> SomeRef<'a, T, M>;
}

impl<'a,T> SomeRef<'a,T, SharedRefFamily> {
    pub fn from_shared(shared: &'a T) -> Self {
        Self {
            shared,
        }
    }
}

impl<'a,T> SomeRef<'a,T, MutRefFamily> {
    pub fn from_mut(mut_: &'a mut T) -> Self {
        Self {
            mut_,
        }
    }
}

#[sealed]
unsafe impl RefMutFamily for SharedRefFamily {
    const IS_SHARED: bool = true;

    type Ref<'a,T> = &'a T
    where
        T: 'a;

    fn with<T, F, O>(ref_: &mut &T, f: F) -> O
    where
        F: FnOnce(&mut &T) -> O,
        T: 'static,
    {
        f(ref_)
    }

    fn as_ref<T,F,R>(ref_: &mut &T, f: F) -> R
    where
        F: FnOnce(&mut &T) -> R,
        T: 'static,
        R: Ref<'static, T, Self, Pointee = T, RefMutFamily = Self>,
    {
        f(ref_)
    }
}

#[sealed]
unsafe impl RefMutFamily for MutRefFamily {
    const IS_SHARED: bool = false;

    type Ref<'a,T> = &'a mut T
    where
        T: 'a;

    fn with<T, F, O>(ref_: &mut &mut T, f: F) -> O
    where
        F: FnOnce(&mut &mut T) -> O,
        T: 'static,
    {
        f(ref_)
    }

    fn as_ref<T,F,R>(ref_: &mut &mut T, f: F) -> R
    where
        F: FnOnce(&mut &mut T) -> R,
        T: 'static,
        R: Ref<'static, T, Self, Pointee = T, RefMutFamily = Self>,
    {
        f(ref_)
    }
}

#[sealed]
impl<'a,T,M> Ref<'a,T,M> for SomeRef<'a,T,M>
where
    M: RefMutFamily,
{
    
    type Pointee = T;
    type RefMutFamily = M;

    fn as_ref(&self) -> &T {
        if M::IS_SHARED {
            unsafe { &self.shared }
        } else {
            unsafe { &self.mut_ }
        }
    }

    fn into_someref(self) -> SomeRef<'a, T, M> {
        self
    }
}

#[sealed]
impl<'a, T> Ref<'a, T, SharedRefFamily> for &'a T {
    type Pointee = T;
    type RefMutFamily = SharedRefFamily;

    fn as_ref(&self) -> &T {
        self
    }

    fn into_someref(self) -> SomeRef<'a, T, SharedRefFamily> {
        SomeRef {
            shared: self,
        }
    }
}

#[sealed]
impl<'a, T> Ref<'a, T, MutRefFamily> for &'a mut T {
    type Pointee = T;
    type RefMutFamily = MutRefFamily;

    fn as_ref(&self) -> &T {
        self
    }

    fn into_someref(self) -> SomeRef<'a, T, MutRefFamily> {
        SomeRef {
            mut_: self,
        }
    }
}
