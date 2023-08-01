#[must_use]
#[derive(Debug)]
pub struct MustUseVariable<T>{
    pub value: T
}

impl<T> MustUseVariable<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T> std::ops::Deref for MustUseVariable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub type MustBool = MustUseVariable<bool>;