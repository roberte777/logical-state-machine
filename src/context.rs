use std::time::Duration;

// S is for user context (state)
// E is for States
#[derive(Clone)]
pub struct AutoClientContext {
    pub tick_rate: u32,
    pub current_state: String,
    pub initial_state: String,
}
impl FromContext for AutoClientContext {
    fn from_context(context: &AutoClientContext) -> Self {
        context.clone()
    }
}
pub struct TickRate(pub Duration);
impl FromContext for TickRate {
    fn from_context(context: &AutoClientContext) -> Self {
        Self(Duration::from_millis(context.tick_rate as u64))
    }
}

pub trait FromContext {
    fn from_context(context: &AutoClientContext) -> Self;
}
pub struct Wrapper<T, F> {
    pub f: F,
    pub marker: std::marker::PhantomData<T>,
}

pub trait Callback {
    fn call(&self, context: &AutoClientContext) -> String;
}
impl<F, T1> Callback for Wrapper<(T1,), F>
where
    F: Fn(T1) -> String,
    T1: FromContext,
{
    fn call(&self, context: &AutoClientContext) -> String {
        (self.f)(T1::from_context(context))
    }
}
impl<F, T1, T2> Callback for Wrapper<(T1, T2), F>
where
    F: Fn(T1, T2) -> String,
    T1: FromContext,
    T2: FromContext,
{
    fn call(&self, context: &AutoClientContext) -> String {
        (self.f)(T1::from_context(context), T2::from_context(context))
    }
}
pub type StoredCallback = Box<dyn Callback>;
