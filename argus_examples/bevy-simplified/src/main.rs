use bevy_simplified::*;

// impl SystemParam for A {}
struct A;

fn use_system<M>(_: impl IntoSystemConfig<M>) {}

fn bad_system(_: A) {}

fn main() {
    use_system(bad_system)
}

// ------------------------------------------------

mod bevy_simplified {
    use std::marker::PhantomData;

    // Exclusive system stuff

    // trait ExclusiveSystemParam {}
    // struct IsExclusiveFunctionSystem;

    // trait ExclusiveSystemParamFunction<Marker> {
    //     type In;
    //     type Out;
    //     type Param: ExclusiveSystemParam;
    // }

    // struct ExclusiveFunctionSystem<Marker, F>
    // where
    //     F: ExclusiveSystemParamFunction<Marker>,
    // {
    //     g0: PhantomData<Marker>,
    //     g1: PhantomData<F>,
    // }

    // impl<Marker, F> IntoSystem<F::In, F::Out, (IsExclusiveFunctionSystem, Marker)> for F
    // where
    //     F: ExclusiveSystemParamFunction<Marker>,
    // {
    //     type System = ExclusiveFunctionSystem<Marker, F>;
    // }

    // Normal system stuff

    pub trait SystemParam {}

    pub trait System {
        type In;
        type Out;
    }

    pub trait IntoSystemConfig<Marker> {}

    pub trait IntoSystem<In, Out, Marker> {
        type System: System<In = In, Out = Out>;
    }

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

    impl<T: SystemParam> SystemParam for (T,) {}

    impl<Marker, F> IntoSystemConfig<Marker> for F where F: IntoSystem<(), (), Marker> {}

    // Multiple options is necessary for making the error message bad
    // otherwise this isn't necessary
    impl<In, Out, Sys: System<In = In, Out = Out>> IntoSystem<In, Out, ()> for Sys {
        type System = Sys;
    }

    impl<Marker, F> IntoSystem<F::In, F::Out, (IsFunctionSystem, Marker)> for F
    where
        F: SystemParamFunction<Marker>,
    {
        type System = FunctionSystem<Marker, F>;
    }

    // impl<Marker, F> System for ExclusiveFunctionSystem<Marker, F>
    // where
    //     F: ExclusiveSystemParamFunction<Marker>,
    // {
    //     type In = F::In;
    //     type Out = F::Out;
    // }

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
}
