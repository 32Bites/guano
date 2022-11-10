use super::{
    parser::Rule,
    span::{IntoSpan, Span, SpanStr},
};
use owning_ref::RcRef;
use pest::iterators::Pair;

#[derive(Debug, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

impl Type {
    pub fn parse(pair: Pair<'_, Rule>, input: RcRef<str>) -> Self {
        let span = pair.as_span().into_span(input.clone());

        let kind = match pair.as_rule() {
            Rule::list_type => {
                let sub_type = pair.into_inner().next().unwrap();

                TypeKind::List(Box::new(Type::parse(sub_type, input)))
            }
            Rule::tuple_type => {
                let mut types = vec![];

                if let Some(sub_types) = pair.into_inner().next() {
                    for sub_type in sub_types.into_inner() {
                        types.push(Type::parse(sub_type, input.clone()));
                    }
                }

                TypeKind::Tuple(types)
            }
            Rule::custom_type => TypeKind::Custom(pair.into_span_str(input)),
            Rule::boolean_type => TypeKind::Boolean,
            Rule::integer_type => TypeKind::Integer,
            Rule::unsigned_integer_type => TypeKind::UnsignedInteger,
            Rule::floating_point_type => TypeKind::FloatingPoint,
            Rule::string_type => TypeKind::String,
            Rule::character_type => TypeKind::Character,
            Rule::primitive_type | Rule::type_ | Rule::declaration_type => {
                return Type::parse(pair.into_inner().next().unwrap(), input)
            }
            _ => unreachable!(),
        };

        Type { kind, span }
    }
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Integer,
    UnsignedInteger,
    FloatingPoint,
    Boolean,
    String,
    Character,
    Custom(SpanStr),
    List(Box<Type>),
    Tuple(Vec<Type>),
}

#[cfg(test)]
mod tests {
    use pest::Parser;

    use super::{
        super::parser::{InternalParser, Rule},
        Type,
    };

    #[test]
    fn test_type() {
        let types = [
            "boolean",
            "int",
            "uint",
            "float",
            "character",
            "string",
            "[]uint",
            "[][]uint",
            "custom",
            "[](custom, uint, other, float)",
        ];

        for ty in types {
            let res = InternalParser::parse(Rule::type_, ty).unwrap();

            for pair in res {
                let ty = Type::parse(pair, todo!());
                println!("{ty:?}");
            }
        }
    }
}
