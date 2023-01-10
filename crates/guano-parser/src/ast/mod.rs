use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::Rc,
};

use guano_span::input::Input;
use helpers::padded;
use nom::{combinator::opt, IResult};
use nom_locate::LocatedSpan;

use crate::ast::expression::Expr;

pub mod block;
pub mod comment;
pub mod declaration;
pub mod error;
pub mod expression;
pub mod implementation;
pub mod node;
pub mod span;
pub mod symbol;
pub mod helpers;

mod prelude;

use self::{span::Span, declaration::module::ModuleItem};

pub type Res<T> = IResult<Span, T>;

#[derive(Debug, Clone)]
pub struct Source {
    input: Input,
    items: Vec<ModuleItem>,
    errors: Vec<error::Error>
}

pub fn init(source: impl Into<Input>) -> (Span, ParserState) {
    let state = ParserState::new();
    let span = LocatedSpan::new_extra(source.into(), state.clone()).into();

    (span, state)
}

pub fn parse(source: impl Into<Input>) -> (Input, Option<Expr>, Vec<error::Error>) {
    let (span, state) = init(source);

    let (input, s) = padded(opt(Expr::parse))(span).unwrap();

    (input.fragment().clone(), s, state.into_errors())
}

#[derive(Debug, Clone)]
pub struct ParserState(Rc<RefCell<Vec<error::Error>>>);

impl ParserState {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(vec![])))
    }

    pub fn errors(&self) -> Ref<'_, Vec<error::Error>> {
        self.0.borrow()
    }

    pub fn into_errors(self) -> Vec<error::Error> {
        self.0.take()
    }

    pub fn report_error(&self, error: error::Error) {
        self.0.borrow_mut().push(error)
    }

    pub fn report(&self, error: impl Into<error::Error>) {
        self.report_error(error.into())
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{declaration::modifier::Modifiers, span::NodeSpan};

    use super::parse;
    use peak_alloc::PeakAlloc;

    #[global_allocator]
    static PEAK_ALLOC: PeakAlloc = PeakAlloc;

    #[test]
    fn test() {
        let(_remaining, _expression, _errors) = parse("!--1000(100, 20)()[i] - 100 * i::  awra ::    awdefs\t as peepee * 2 * (ree.reeee)(iiii) + 1 + !--1000(100, 20)()[i] - 100 * i::  awra ::    awdefs\t as peepee * 2 * (ree.reeee)(iiii) = 100");
        //let(expr, errors) = parse("!--1000(100, 20)()[i] - 100 * i::  awra ::    awdefs\t as peepee * 2 * (ree.reeee)(iiii) + 1 + !--1000(100, 20)()[i] - 100 * i::  awra ::    awdefs\t as peepee * 2 * (ree.reeee)(iiii) = 100");
        parse("5878709870987809879087097809870987098709908709.077098709870987098709870987987987");
        parse(
            "''   /* fslejkflkejsnflks */
        'x' // Hello",
        );
        parse(r##""Hello, world\\\ ""Test """##);

        parse("help::self::i");
        dbg!(parse(r#"'\x0a'"#));

        println!("Peak: {} KB", PEAK_ALLOC.peak_usage_as_kb());
        println!("Current: {} KB", PEAK_ALLOC.current_usage_as_kb());
        println!("Modifiers: {}", std::mem::size_of::<Modifiers>());
        println!("NodeSpan: {}", std::mem::size_of::<NodeSpan>());
        println!(
            "Option<NodeSpan>: {}",
            std::mem::size_of::<Option<NodeSpan>>()
        );
        println!("usize: {}", std::mem::size_of::<usize>());
        println!("u32: {}", std::mem::size_of::<u32>());
        println!("&str: {}", std::mem::size_of::<&str>());
        println!("(): {}", std::mem::size_of::<()>());
        println!(
            "Alignment of nodespan: {}",
            std::mem::align_of::<NodeSpan>()
        );
    }
}
