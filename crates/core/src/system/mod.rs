#[cfg(test)]
mod tests;

pub trait System {
        type InputType: Sized + Send + Sync;
        type OutputType: Sized + Send + Sync;
        
        /// For archetype indexing
        fn identifier() -> String;

        /// Called per tick when it's needed with the args set to the input type.
        /// Must return the output type specified.
        ///
        /// If you are passing the OutputType (O) of one system into the InputType (I) of
        /// another system, please make sure you have defined an `impl Into<I> for O` implementation.
        ///
        /// Pulsar will automatically use `SomeSystem::call(args.into())` for every system call to ensure these
        /// implementations are used.
        fn call(args: Self::InputType) -> Self::OutputType;
}