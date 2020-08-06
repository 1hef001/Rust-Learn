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
    