# Concepts Learned in chapter 3


1.  const keyword
2.  const can't be changed to mutable
3.  data type of const must be annoted
4.  const can only be set to constant expressions and not runtime expressions or function calls
5.  shadowing - don't confuse shadowing with mutable var or immutable var. each instance of let allocates a new memory location for the variable to be stored in. Only the name of any other variable may be reused.
6.  Example for shadowing vs mutabiliy : 
    -   ```
        let spaces = "     ";
        let spaces = spaces.len();      // can change the datatype of the variable
        ```
        vs
    
    -   ```
        let spaces = "     ";
        spaces = spaces.len();          // immutable, can't change the datatype of the variable
        ```

7.  Length function for sting =>  str.len()  -> return type <int>
8.  Datatypes in Rust.
9.  Rust is statically typed.
    ->  Scalar types : 
        - Integer types
                Signed, Unsigned
            - i8, u8
            - i16, u16
            - i32, u32
            - i64, u64
            - i128, u128
            - isize, usize
        
        - Floating Point types
            - f32
            - f64

        - Boolean type
            - bool

        - Character type

    ->  Compound types
        - Tuple type
            - example :
                ```
                    fn main(){
                        let tup: (i32, f64, u8) = (500, 6.4, 1)
                    }
                ```
            - unpacking tuple into multiple variables
            - ** prefix unused variables with "_" **
            - tuple elements can also be accessed using period operator "."

        - Array type
            - example :
                ```
                    fn main(){
                        let a = [1, 2, 3, 4, 5];
                    }
                ```
            Arrays are of fixed length. If you want variable sized ones, use vector or collection.
            Arrays should be subscripted to access values.
            All normal Array principle apply.

10. Functions
    - Unlike C\C++ wherein you have to mention function prototypes in order to write the same after main function. It is not necessary here.
    - function names and variable names follow snake case by convention.
    - functions can be with or without parameters.
    - functions with parameters must annote datatypes.
    - parameters can be of any type, but must explicitly mention within parathesis at function definition.
    * Statements like x = y = 6 are invalid in rust.
    - Variables can be assigned blocks where new scopes are created, so long as it returns a value as in example: 
        ```
        fn main(){
            println!("This is main function");
            let y = {
                let x = 1;
                x + 2
            };
            println!("The value of y is {}", y);
        }
        ```
    - Return type must be specified after the function name in the declaration.
    - There is no explicit return statements in rust. The last value that you find at the end of the block is probably what returns to the previous function.
        - example:
        ```
            fn ret() -> i32 {
                -5
            }
        ```
        ** Note there shouldn't be a semicolon at the end. It is not a mistake.**

11. Conditional statements
    - Example for if-else_if-else block:
    ```
        fn main(){
            let number = 3;

            if number < 5 {
                println!("Number is less than 5");
            }
            else if number == 5 {
                println!("Number is equal to 5");
            }
            else {
                println!("Number is greater than 5");
            }
        }
    ```
    - Example of if statement in let:
    ```
        fn main(){
            let number = if true {5} else {6};
            println!("value of number is {}", number)
        }
    ```
    - Not an example of if statement in let:
    ```
        fn main(){
            let number = if true {5} else {"6"};
            println!("value of number is {}", number)
        }
    ```
    ** Watch the type mismatch. Rust is very specific about it **.

12. Operators
    -  All basic operators and operations apply.

13. Loops
    - Example  (while):
    ```
        fn main(){
            let mut counter: u32 = 0;

            while counter != 10 {
                counter += 1
            }

            println!("Result is: {}", counter);
        }
    ```

    - Example   (loop):
    ```
        fn main(){
            let mut counter: u32 = 0;

            let result = loop{
                counter += 1;

                if counter == 10 {
                    break counter * 2;
                }
            };

            println!("Result is: {}", result);
        }
    ```

    - Example   (for):
    ```
        fn main(){
            let a = [1, 2, 3, 4, 5];

            for i in a.iter() {
                println!("value of i is : {}", i);
            }
        }
    ```

13. The range function equivalent of python is (start..end) here. Example:
    ```
        fn main(){
            // let a = [1, 2, 3, 4, 5];

            for i in (1..4).rev() {
                println!("{}", i);
            }
        }
    ```




