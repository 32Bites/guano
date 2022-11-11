## This is a comment

#$
    Multi
        Line
            Comment!
$#

let global_name = "This Global Variable Was Created By Noah";

type function_type_example = fun: string; ## function with no parameters but returns a string.
type function_type_example_2 = fun([]string, uint): string; ## function that returns a string while accepting a slice of strings, and a uint.
type function_type_example_3 = fun(string, uint); ## function that has no return value but accepts a string and a uint.
type_function_type_example_4 = fun; ## function that has no parameters or return value.

## Entrypoint
fun main @ args: []string {
    let name: string = "Noah";
    let first: float = 11.0;
    let second: uint = 6;
    let age: float = add(first, second);
    let new_age: float = add(first, add(second, second));

    if true {

    } else if false {

    }

    while true {
        
    }

    personPrint(age, name);
}

## Prints a person's age and name.
fun personPrint @ age: float, name: string {
    print("{}'s age: {}" : (name, age));
}

## Add two values
fun add: float @ first: float, second: uint {
    return first + second as float;
}

let test = (5 + 6) << 1 >> 2 + ree(1)[0][1].ree.ree().mee(1, 2) as []uint + (1, 2) - (1 + 4,) + (1,);
let i = 1;

proto something {
    fun print;
}

proto somethingElse(something) {
}

class first_class {
    priv fun main: string @ first: ree {
        return "";
    }
}

class my_class(first_class): something {
    
}