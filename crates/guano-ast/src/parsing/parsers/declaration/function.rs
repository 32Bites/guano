use std::vec;

use guano_syntax::{
    consts::{Keyword, Punctuation},
    node, Child, SyntaxKind,
};

use crate::parsing::{
    combinators::{alternation, tuple, Combinators},
    error::Res,
    parsers::{
        expression::block::block,
        ignorable::{eat_ignorable, IgnorableParser},
        symbols::{identifier::iden, ty::ty},
    },
    ParseContext, Parser,
};

pub fn func<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let mut children = func_qualifiers(context)?;

    let (kw, (l_ws, name, r_ws)) =
        tuple((Keyword::FUN, iden.expected().padded())).parse(context)?;
    children.push(kw);
    children.extend(l_ws);
    children.push(name);
    children.extend(r_ws);

    if let Some((params, ws)) = func_params.then(eat_ignorable).optional().parse(context)? {
        children.push(params);
        children.extend(ws);
    }

    if let Some((ty, ws)) = func_type.then(eat_ignorable).optional().parse(context)? {
        children.push(ty);
        children.extend(ws);
    }

    let body = func_body.expected().parse(context)?;
    children.push(body);

    Ok(node(SyntaxKind::FUNC, children))
}

pub fn func_body<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let body = alternation((block, Punctuation::SEMICOLON)).parse(context)?;

    Ok(node(SyntaxKind::FUNC_BODY, vec![body]))
}

pub fn func_type<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (arrow, ws, ty) = tuple((Punctuation::THIN_ARROW, eat_ignorable, ty)).parse(context)?;
    let mut children = vec![arrow];
    children.extend(ws);
    children.push(ty);

    Ok(node(SyntaxKind::FUNC_TYPE, children))
}

pub fn func_params<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let param = tuple((
        iden,
        eat_ignorable,
        Punctuation::COLON.expected(),
        eat_ignorable,
        ty.expected(),
    ));
    let other_params = eat_ignorable.then(func_param).repeated();
    let params = tuple((param, other_params)).optional();

    let (l_paren, (l_ws, params, r_ws), r_paren) = tuple((
        Punctuation::LEFT_PAREN,
        params.padded(),
        Punctuation::RIGHT_PAREN.expected(),
    ))
    .parse(context)?;

    let mut children = vec![l_paren];
    children.extend(l_ws);
    if let Some((first_param, other_params)) = params {
        let first_param = {
            let (name, l_ws, col, r_ws, ty) = first_param;
            let mut children = vec![name];
            children.extend(l_ws);
            children.push(col);
            children.extend(r_ws);
            children.push(ty);

            node(SyntaxKind::FUNC_PARAM, children)
        };

        children.push(first_param);

        for (ws, param) in other_params {
            children.extend(ws);
            children.push(param);
        }
    }
    children.extend(r_ws);
    children.push(r_paren);

    Ok(node(SyntaxKind::FUNC_PARAMS, children))
}

pub fn func_param<'source>(context: &mut ParseContext<'source>) -> Res<'source, Child> {
    let (com, l_ws, name, m_ws, col, r_ws, ty) = tuple((
        Punctuation::COMMA,
        eat_ignorable,
        iden.expected(),
        eat_ignorable,
        Punctuation::COLON.expected(),
        eat_ignorable,
        ty.expected(),
    ))
    .parse(context)?;

    let mut children = vec![com];
    children.extend(l_ws);
    children.push(name);
    children.extend(m_ws);
    children.push(col);
    children.extend(r_ws);
    children.push(ty);

    Ok(node(SyntaxKind::FUNC_PARAM, children))
}

pub fn func_qualifiers<'source>(context: &mut ParseContext<'source>) -> Res<'source, Vec<Child>> {
    let mut children = vec![];

    if let Some((kw, ws)) = Keyword::PUB.then(eat_ignorable).optional().parse(context)? {
        children.push(kw);
        children.extend(ws);
    }

    if let Some((kw, ws)) = Keyword::VETO
        .then(eat_ignorable)
        .optional()
        .parse(context)?
    {
        children.push(kw);
        children.extend(ws);
    }

    if let Some((kw, ws)) = Keyword::STATIC
        .then(eat_ignorable)
        .optional()
        .parse(context)?
    {
        children.push(kw);
        children.extend(ws);
    }

    Ok(children)
}

/// Generalized parser for blocks that
/// consist only of function declarations.
pub fn funcs<'source>(
    kind: SyntaxKind,
) -> impl FnMut(&mut ParseContext<'source>) -> Res<'source, Child> + Clone + Copy {
    move |context| {
        let (l_paren, (l_ws, funcs, r_ws), r_paren) = tuple((
            Punctuation::LEFT_CURLY,
            func.then(eat_ignorable).repeated().padded(),
            Punctuation::RIGHT_CURLY.expected(),
        ))
        .parse(context)?;

        let mut children = vec![l_paren];
        children.extend(l_ws);

        for (func, r_ws) in funcs {
            children.push(func);
            children.extend(r_ws);
        }

        children.extend(r_ws);
        children.push(r_paren);

        Ok(node(kind, children))
    }
}

#[cfg(test)]
mod test {
    use crate::parsing::{
        combinators::Combinators, parsers::ignorable::IgnorableParser, ParseContext,
    };

    #[test]
    pub fn test_func() {
        let source = r#"
        fun main {
            let people = ["Noah", "Gabby", "Jairo", "Rebecca"];
        }
/*         pub veto static fun sqrt_2 -> float {
            // This is a stray comment
            return math::sqrt(2.0);
        }
        
        fun other_sqrt_2 -> float {
            return srt(2.0);
        } */"#;

        let mut context = ParseContext::new(source);
        let parser = super::super::decl.ast().padded().repeated();
        let output = context
            .parse(parser)
            .unwrap()
            .into_iter()
            .map(|(_, n, _)| n);

        for node in output {
            println!("{node:#?}");
        }
    }
}
