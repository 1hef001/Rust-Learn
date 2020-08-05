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