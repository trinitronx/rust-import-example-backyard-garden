
/**
  # Backyard: Rust Book Modules Example

  An example provided in Rust Book Ch. 7:
  Managing Growing Projects with Packages, Crates, and Modules

  `backyard` root crate contains another module: `garden`
   Inside `garden` is another modules: `vegetables`
   Declared in `vegetables` is: `Asparagus`

   This project structure exemplifies how to use such a submodule.
*/

// Try commenting & un-commenting the following lines to see their effect on import paths

/// Demonstrate use of backyard:: prefixed import paths
use backyard::garden;
use backyard::vegetables;
// use backyard::garden::vegetables;
// use backyard::vegetables::Asparagus;

/// Demonstrate use of shorter names post-public API import
use vegetables::Asparagus;
use vegetables::Radicchio;

// Demonstrate use of crate:: namespaced imports
// use crate::garden::vegetables::Asparagus;
// use crate::garden::vegetables::Radicchio;

/// The main function to demonstrate `use` of `Asparagus`
fn main() {
    let plant = Asparagus {};
    println!("I'm growing simply {:?}!", plant);

    main2();
    main3();
    main4();
    main5();
}

/// Alt main function to demonstrate `use` of `vegetables` module
fn main2() {
    let plant = vegetables::Asparagus {};
    println!("I'm growing vegetables::{:?}!", plant);
}

/// Alt main function to demonstrate `use` of `backyard::garden` module
fn main3() {
    let plant = garden::vegetables::Asparagus {};
    println!("I'm growing garden::vegetables::{:?}!", plant);
}

/// Alt main function to demonstrate `use` of full verbose path to `backyard::garden::vegetables::Radicchio`
fn main4() {
    let plant = backyard::garden::vegetables::Radicchio {};
    println!("I'm growing backyard::garden::vegetables::{:?}!", plant);
}

/// Alt main function to demonstrate `use` of less rediculous path to `Radicchio`
fn main5() {
    let plant = Radicchio {};
    println!("I'm growing simply {:?}!", plant);
    println!("... it's a little less Radicchi...yo!");
}
