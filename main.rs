/*============= Assignment 7 Question 1 ======================
Q1. Write a Rust Program,
● Make a module with suitable name in main.rs
● Make a sub module with an other name in first module
● Define a simple function in sub module
● Call that function from fn main
*/

mod military{
   pub mod weapon{
      pub  fn smg(){
            println!("SMG is an automatic medium range weapon.");
        }
    }
}

fn main() {
// calling mod's function using absolute path
crate::military::weapon::smg();
}
