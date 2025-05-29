
    // Advanced Traits: ASSOCIATED Types;
    // these connect a type placeholder with a trait such that the trait method definitions can use these placeholder types
    // in their signatures
    // The implementor of a trait will specify the concrete type to be used instead of the placeholder type for a particular implementation.
    // That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented

    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    } 
    // the definition of an Iterator trait that has an associated Item
    // The type Item is a placeholder, and the next method's definition shows that it will return values of type Option<Self::Item>
    // Implementors of the Iterator trait will specify the conrete type for Item, and the next method will eturn an Option 
    // conatining a value of that concrete type. Associated types may look like generics ,in that generics allows us to define a function without specifying 
    // what types it can handle.
    // EXAMPLE:
    
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            // --Snip
        }
    } // This syntax looks like a generic - So why not just define the Iterator trait with generics as below:?
    
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }
    // A hypothetical definition of an Iterator trait using generics
    // The DIFFERENCE:
    // - when using generics, we must annotate the types in each implementation, 
    // bcs we can also implement Iterator<String> for Counter or any type, we could have multiple implementations of Iterator for Counter.
    // In other words, when trait has a generic parameter, it can be implemented for a type multiple times, changing tje concrete types of the generic type
    // parameters each time.
    // When using the next METHOD on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use
    
    // with ASSOCIATED types - we don't need to annotate types bcs we can't implement a trait on a type multiple times
    // we can choose what the type of Item will be only once, bcs there can be only one type 'impl' Iterator for Counter .
    // We don't have to specify that we want an iterator of u32 values everywhere that we call next on Counter.
    // ASSOCAITED types also become part of the trait's contract: implementors of the trait must provide a type to stand in for the associated type placeholder.




