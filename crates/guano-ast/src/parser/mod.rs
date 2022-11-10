pub mod expression;
pub mod literal;
pub mod operator;
pub mod declaration;
pub mod typing;
pub mod statement;
pub mod block;
pub mod source_file;
pub mod span;
pub mod parser;

pub use parser::*;

#[cfg(test)]
mod tests {
    use super::{InternalParser, Rule};
    use pest::Parser;

    #[test]
    pub fn test_pest() {
        let comment = r#"
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
    let second: uint = 6;
    let age: float = add(first, second);
    let new_age: float = add(first, add(second, second));

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

}

class my_class(first_class): something {
    
}
        "#;
        let parse = InternalParser::parse(Rule::source_file, comment);

        match parse {
            Ok(res) => {
                println!("Result: {res}");
            }
            Err(error) => println!("Error: {error}"),
        }
    }
}
