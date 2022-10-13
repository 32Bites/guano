## This is a comment

#$
    Multi
        Line
            Comment!
$#

let global_name = "This Global Variable Was Created By Noah";

## Entrypoint
fun main @ args: []string {
    let name: string = "Noah";
    let first: float = 11.0;
    let second: usize = 6;
    let age: float = add @ first, second;
    let new_age: float = add @ first, (add @ second, second);

    personPrint @ age, name;

    ret 0;
}

## Prints a person's age and name.
fun personPrint @ age: float, name: string {
    print @ "{}'s age: {}" : (name, age);
}

## Add two values
fun add: f32 @ first: float, second: uint {
    ret first + second as float;
}