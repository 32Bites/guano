use guano_common::rowan::{GreenNode, NodeOrToken};
use guano_syntax::{
    parser::{keyword::kw_this, punctuation::colon2, wrap, Input, Res as Result},
    SyntaxKind,
};

use crate::ast::prelude::*;

use super::iden;

pub fn name<'a>(input: Input<'a>) -> Result<'a> {
    wrap(
        alt((kw_this(iden::parse_raw), iden::parse)),
        SyntaxKind::NAME,
    )(input)
}

pub fn path_segment<'a>(input: Input<'a>) -> Result<'a> {
    let (input, ((ignored_left, sep, ignored_right), name)) = pair(pad(colon2), name)(input)?;

    let mut children = ignored_left;
    // The 2 is from `sep` and `name`
    children.reserve(2 + ignored_right.len());

    children.push(sep);
    children.extend(ignored_right);
    children.push(name);

    let node = NodeOrToken::Node(GreenNode::new(SyntaxKind::PATH_SEGMENT.into(), children));

    Ok((input, node))
}

pub fn path<'a>(input: Input<'a>) -> Result<'a> {
    let (input, first) = wrap(name, SyntaxKind::PATH_SEGMENT)(input)?;
    let (input, segments) = many0(path_segment)(input)?;

    let mut children = vec![first];

    children.reserve(segments.len());
    children.extend(segments);

    let node = NodeOrToken::Node(GreenNode::new(SyntaxKind::PATH.into(), children));

    Ok((input, node))
}

#[derive(Debug, Clone, Default)]
pub struct Path {
    segments: Vec<Iden>,
    span: NodeSpan,
}

impl From<Iden> for Path {
    fn from(i: Iden) -> Self {
        let span = i.span().clone();

        Self {
            segments: vec![i],
            span,
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}

impl Eq for Path {}

impl Path {
    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, segments)) = consumed(Self::parse_segments)(input)?;

        let path = Self {
            segments,
            span: span.into_node(),
        };

        Ok((input, path))
    }

    fn parse_segments(input: Span) -> Res<Vec<Iden>> {
        let (input, first_segment) = Iden::parse(input)?;
        let segment = preceded(
            padded(tag("::")),
            map(expect(Iden::parse, "Expected iden"), |o| {
                o.unwrap_or_default()
            }),
        );

        let (input, segments) = fold_many0(
            segment,
            || vec![first_segment.clone()],
            |mut segments, segment| {
                segments.push(segment);
                segments
            },
        )(input)?;

        Ok((input, segments))
    }

    pub fn segments(&self) -> &[Iden] {
        &self.segments
    }

    pub(crate) fn take_segments(self) -> Vec<Iden> {
        self.segments
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }
}

impl<'a, D: ?Sized + pretty::DocAllocator<'a, ()>> pretty::Pretty<'a, D, ()> for &'a Path {
    fn pretty(self, allocator: &'a D) -> pretty::DocBuilder<'a, D, ()> {
        allocator.intersperse(self.segments(), "::")
    }
}

impl Node for Path {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use crate::ast::{display::ToDisplay, helpers::padded, init, symbol::path::Path};

    #[test]
    fn test_parse_path() {
        let (span, state) = init("path::this_::is_ :: a :: test // hello");

        let (remaining, path) = padded(Path::parse)(span).unwrap();

        println!("{}", path.display_width(0));

        // println!("Path: {path}");
        println!("Remaining: {:?}", remaining.display());
        println!("Errors: {:?}", state.errors());
    }
}
