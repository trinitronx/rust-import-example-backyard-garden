/*!
# Backyard Garden

A library for modeling backyard and garden items like vegetables.

## Examples

You can use the `vegetables` module like this:

```rust
use backyard::garden::vegetables;

fn main() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

Alternatively, you can use the convenience re-export path like this:

```rust
use backyard::vegetables;

fn main() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

Or use just one vegetable like this:

```rust
use backyard::vegetables::Radicchio;

/// A main function to demonstrate `use` of just `Radicchio`
fn main() {
    let plant = Radicchio {};
    println!("I'm growing backyard::vegetables::{:?}!", plant);
}
```

Or perhaps a convenient vegetable like this:

```rust
use backyard::vegetables;
use vegetables::Asparagus;

/// A main function to demonstrate `use` of simply `Asparagus`
fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

*/


// Some demonstration public re-exports / exposing public crate API
// pub mod backyard;

// use crate::garden::vegetables::Asparagus;
pub use crate::garden::vegetables;
// pub use self::garden::vegetables::Asparagus;
// pub use self::garden::vegetables;

pub mod garden;