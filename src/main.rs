
//     // Advanced Traits: ASSOCIATED Types;
//     // these connect a type placeholder with a trait such that the trait method definitions can use these placeholder types
//     // in their signatures
//     // The implementor of a trait will specify the concrete type to be used instead of the placeholder type for a particular implementation.
//     // That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented

//     pub trait Iterator {
//         type Item;

//         fn next(&mut self) -> Option<Self::Item>;
//     } 
//     // the definition of an Iterator trait that has an associated Item
//     // The type Item is a placeholder, and the next method's definition shows that it will return values of type Option<Self::Item>
//     // Implementors of the Iterator trait will specify the conrete type for Item, and the next method will eturn an Option 
//     // conatining a value of that concrete type. Associated types may look like generics ,in that generics allows us to define a function without specifying 
//     // what types it can handle.
//     // EXAMPLE:
    
//     impl Iterator for Counter {
//         type Item = u32;

//         fn next(&mut self) -> Option<Self::Item> {
//             // --Snip
//         }
//     } // This syntax looks like a generic - So why not just define the Iterator trait with generics as below:?
    
//     pub trait Iterator<T> {
//         fn next(&mut self) -> Option<T>;
//     }
//     // A hypothetical definition of an Iterator trait using generics
//     // The DIFFERENCE:
//     // - when using generics, we must annotate the types in each implementation, 
//     // bcs we can also implement Iterator<String> for Counter or any type, we could have multiple implementations of Iterator for Counter.
//     // In other words, when trait has a generic parameter, it can be implemented for a type multiple times, changing tje concrete types of the generic type
//     // parameters each time.
//     // When using the next METHOD on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use
    
//     // with ASSOCIATED types - we don't need to annotate types bcs we can't implement a trait on a type multiple times
//     // we can choose what the type of Item will be only once, bcs there can be only one type 'impl' Iterator for Counter .
//     // We don't have to specify that we want an iterator of u32 values everywhere that we call next on Counter.
//     // ASSOCAITED types also become part of the trait's contract: implementors of the trait must provide a type to stand in for the associated type placeholder.

// // Operator Overloading: - to implement the above

//     use std::ops::Add;

//     #[derive(Debug, Copy, Clone, PartialEq)]
//     struct Point {
//         x: i32,
//         y: i32,
//     }

//     impl Add for Point {
//         type Output = Point;

//         fn add(self, other: Point) -> Point {
//             Point {
//                 x: self.x + other.x,
//                 y: self.y + other.y,
//             }
//         }
//     }

//   fn main() {
//         assert_eq!(
//             Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//             Point { x: 3, y: 3}
//         );
//     } // Implementing the Add trait to overload the '+' operator for Point instances

//     // The add method adds the x values of two Point instances and the y values of two Point instances to create a new Point.
//     // The Add trait has an associated type named Output that determines the type  returned from the add method.

//     // The default generic type in this code is within the Add trait:

//     trait Add<Rhs=Self> {
//         type Output;

//         fn add(self, rhs: Rhs) -> Self::Output;
//     }
//     // This code should look familiar: a trait with one method and an associated type.
//     // The new part is Rhs=Self: this syntax is called default type parameters. The Rhs generic type parameter defines the type of the rhs parameter in the add method. 
//     // If we don't specify a concrete type for Rhs when we implement teh Add trait, the type of Rhs will default to Self, which will be the type we're implementing Add on.
//     // When we implemented Add for Point, we used the default for Rhs bcs we wanted to add two Point instances

//     // Below, we implement the Add trait where we want to customize the Rhs type rather than using the default.
//     // - we have two structs Milimeters and Meters, holding values in different units.
//     // - this thin wrapping of an existing type in another struct is known as the 'newtype pattern' 
//     // - we want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly.
    
//     use std::ops::Add;

//     struct Milimeters(u32);
//     struct Meters(u32);

//     impl Add<Meters> for Milimeters {
//         type Output = Milimeters;
        
//         fn add(self, other: Meters) -> Milimeters {
//             Milimeters(self.0 + (other.0 * 1000))
//         }        
//     }

    //DISAMBIGUATING Between Methods with the Same Name:

    
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Uphi, usemoyeni?");        
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms and shoulders ngolona hlobo*");
        }
    }


    // fn main() { 
        

    //     let person = Human;

    //     person.fly();
    //     Pilot::fly(&person);
    //     Wizard::fly(&person);

    // }


// Implementing an ASSOCIATIVE func that is not a method with no self parameter;
// when there are multiple types or traits that define non-method functions with the same function name - use fully qualified syntax

//EXAMPLE:

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A bay dog is called a {}", Dog::baby_name());
}


// ADVANCED types - Aliases:
// - introducing an alias Thunk to reduce repetition

type Thunk = Box<dyn Fn() + Send 'static'>;

let f: Thunk = Box::(|| println!("Molo, unjani?"));

fn takes_long_type(f: Thunk) {
    // ....snip...
}

fn returns_long_type() -> Thunk {
    // --snip--
}

// Type aliases with Result<T, E>: using std::io::Error;

use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}


