use std::any::Any;

///
pub trait ToAny: 'static {
    ///
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> ToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
