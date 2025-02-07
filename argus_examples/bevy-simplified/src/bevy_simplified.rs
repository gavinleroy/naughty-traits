use std::marker::PhantomData;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn insert_resource<T>(self, r: T) -> Self {
        self
    }

    pub fn add_system<M>(self, _: impl IntoSystemConfig<M>) -> Self { self }

    pub fn run(self) { }
}

// General Stuff

pub trait Resource {}

pub struct TheWorld {}
pub struct Res<T> {
    _marker: PhantomData<T>,
}

pub trait SystemParam {}
impl<T: Resource> SystemParam for Res<T> {}
impl<T: SystemParam> SystemParam for (T,) {}

pub trait System {
    type In;
    type Out;
}

pub trait IntoSystemConfig<Marker> {}

pub trait IntoSystem<In, Out, Marker> {
    type System: System<In = In, Out = Out>;
}

// Exclusive system stuff

pub trait ExclusiveSystemParam {}
pub struct IsExclusiveFunctionSystem;

pub trait ExclusiveSystemParamFunction<Marker> {
    type In;
    type Out;
    type Param: ExclusiveSystemParam;
}

pub struct ExclusiveFunctionSystem<Marker, F>
where
    F: ExclusiveSystemParamFunction<Marker>,
{
    g0: PhantomData<Marker>,
    g1: PhantomData<F>,
}

impl<Marker, F> IntoSystem<F::In, F::Out, (IsExclusiveFunctionSystem, Marker)> for F
where
    F: ExclusiveSystemParamFunction<Marker>,
{
    type System = ExclusiveFunctionSystem<Marker, F>;
}

// Normal system stuff

pub trait SystemParamFunction<Marker> {
    type In;
    type Out;
    type Param: SystemParam;
}

pub struct IsFunctionSystem;

pub struct FunctionSystem<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    g0: PhantomData<Marker>,
    g1: PhantomData<F>,
}

impl<Marker, F> IntoSystemConfig<Marker> for F where F: IntoSystem<(), (), Marker> {}

impl<Marker, F> IntoSystem<F::In, F::Out, (IsFunctionSystem, Marker)> for F
where
    F: SystemParamFunction<Marker>,
{
    type System = FunctionSystem<Marker, F>;
}

impl<Marker, F> System for ExclusiveFunctionSystem<Marker, F>
where
    F: ExclusiveSystemParamFunction<Marker>,
{
    type In = F::In;
    type Out = F::Out;
}

impl<Marker, F> System for FunctionSystem<Marker, F>
where
    F: SystemParamFunction<Marker>,
{
    type In = F::In;
    type Out = F::Out;
}

impl<Func, Out, F0> SystemParamFunction<fn(F0) -> Out> for Func
where
    F0: SystemParam,
    // Remove the below constraint to get the funky error message "cannot infer type for F0"
    Func: FnMut(F0) -> Out,
{
    type In = ();
    type Out = Out;
    type Param = (F0,);
}

impl<'a, Func, Out> ExclusiveSystemParamFunction<fn(&'a mut TheWorld) -> Out> for Func 
where 
Func: FnMut(&mut TheWorld) -> Out, {
    type In = ();
    type Out = Out;
    type Param = (&'a mut TheWorld);
}
