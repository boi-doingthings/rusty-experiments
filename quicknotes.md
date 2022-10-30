## Variables & Mutability
### Variables
* Rust variables are immutable by default. The value assigned to a variable at initialisation can't be changed. --> Gives compiler error if done.
* Add `mut` keyword before variable to explictly tell compiler that variable is mutable.

### Constants
* To make a variable that doesn't have the power to ever change a value, better use a `constant` using the `const` keyword. But dataype must always be annotated.
  * constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
  *  Rust’s naming convention for constants is to use all uppercase with underscores between words. 

--> Variable Declaration: let a =5;
--> Constant Declaration: const SECONDS_IN_A_DAY: u32= 24 * 60 * 60;


#### Shadowing
