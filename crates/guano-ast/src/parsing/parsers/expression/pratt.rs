use guano_syntax::{node, Node, SyntaxKind};

use crate::parsing::{
    combinators::{tuple, Combinators},
    error::Res,
    parsers::ignorable::eat_ignorable,
    ParseContext, Parser,
};

use super::{
    operator::{infix::binary_op, postfix::postfix_operator, prefix::unary_op},
    primary::primary,
};

#[inline]
pub fn pratt<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    pratt_expr(Power::min()).parse(context)
}

fn pratt_expr<'source>(
    min_bp: Power,
) -> impl FnMut(&mut ParseContext<'source>) -> Res<'source, Node> {
    move |context| {
        let mut lhs = pratt_prefix(context)?;

        loop {
            match pratt_postfix(context, &mut lhs, min_bp)? {
                Action::Break => break,
                Action::Continue => continue,
                Action::Nothing => {}
            }

            match pratt_infix(context, &mut lhs, min_bp)? {
                Action::Break => break,
                Action::Continue => continue,
                Action::Nothing => {}
            }

            break;
        }

        Ok(lhs)
    }
}

fn pratt_postfix<'source>(
    context: &mut ParseContext<'source>,
    lhs: &mut Node,
    min_bp: Power,
) -> Res<'source, Action> {
    let operator = postfix_operator.peek().optional().parse(context)?;

    if let Some(operator) = operator {
        let (left_bp, ()) = operator.bind_power();

        if left_bp < min_bp {
            return Ok(Action::Break);
        }

        operator.parse_expr(lhs, context)?;

        Ok(Action::Continue)
    } else {
        Ok(Action::Nothing)
    }
}

fn pratt_infix<'source>(
    context: &mut ParseContext<'source>,
    lhs: &mut Node,
    min_bp: Power,
) -> Res<'source, Action> {
    let operator = tuple((eat_ignorable, binary_op, eat_ignorable))
        .spanned()
        .peek()
        .optional()
        .parse(context)?;

    if let Some(((left_ws, (operator, kind), right_ws), span)) = operator {
        let (left_bp, right_bp) = kind.bind_power();

        if left_bp < min_bp {
            return Ok(Action::Break);
        }

        context.advance_byte(u32::from(span.len()) as usize)?;

        let rhs = pratt_expr(right_bp)
            .expect("Expected expression")
            .parse(context)?;

        take_mut::take(lhs, |lhs| {
            let mut children = vec![lhs];
            children.extend(left_ws);
            children.push(operator);
            children.extend(right_ws);
            children.push(rhs);

            let binary = node(SyntaxKind::BINARY_EXPR, children);
            // let expr = node(SyntaxKind::EXPR, vec![binary]);

            binary
        });

        Ok(Action::Continue)
    } else {
        Ok(Action::Nothing)
    }
}

fn pratt_prefix<'source>(context: &mut ParseContext<'source>) -> Res<'source, Node> {
    let maybe_operator = unary_op.then(eat_ignorable).optional().parse(context)?;

    match maybe_operator {
        Some(((operator, kind), whitespace)) => {
            let ((), right_bp) = kind.bind_power();

            // TODO: Use expect() here.
            let rhs = pratt_expr(right_bp)
                .expect("Expected expression")
                .parse(context)?;

            let mut children = vec![operator];
            children.extend(whitespace);
            children.push(rhs);

            let unary = node(SyntaxKind::UNARY_EXPR, children);
            // let expr = node(SyntaxKind::EXPR, vec![unary]);

            Ok(unary)
        }
        None => primary(context),
    }
}

enum Action {
    Break,
    Continue,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(super) struct Power(pub u32);

impl Power {
    pub const fn normalized(self) -> Self {
        Self(self.0 * 10)
    }

    pub const fn raised(self) -> Self {
        Self(self.0 + 1)
    }

    pub const fn lowered(self) -> Self {
        Self(self.0.saturating_sub(1))
    }

    #[allow(dead_code)]
    pub const fn max() -> Self {
        Self(u32::MAX)
    }

    pub const fn min() -> Self {
        Self(u32::MIN)
    }
}

impl From<u32> for Power {
    fn from(value: u32) -> Self {
        Power(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Associativity {
    Left,
    Right,
    Neither,
}

impl Associativity {
    #[inline]
    pub fn bind_power(&self, power: Power) -> (Power, Power) {
        let power = power.normalized();
        match self {
            Associativity::Left => (power, power.raised()),
            Associativity::Right => (power, power.lowered()),
            Associativity::Neither => (power, power),
        }
    }
}

pub(super) trait Infix {
    fn associativity(&self) -> Associativity;
    fn power(&self) -> Power;

    #[inline]
    fn bind_power(&self) -> (Power, Power) {
        self.associativity().bind_power(self.power())
    }
}

pub(super) trait Prefix {
    fn power(&self) -> Power;

    #[inline]
    fn bind_power(&self) -> ((), Power) {
        ((), self.power().normalized())
    }
}

pub(super) trait Postfix {
    fn power(&self) -> Power;

    #[inline]
    fn bind_power(&self) -> (Power, ()) {
        (self.power().normalized(), ())
    }
}
