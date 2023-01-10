use crate::ast::prelude::*;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Path {
    segments: Vec<Iden>,
    #[cfg_attr(feature = "serde", serde(skip))]
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

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.segments.iter().peekable();

        while let Some(segment) = iter.next() {
            write!(f, "{segment}")?;

            if iter.peek().is_some() {
                f.write_str("::")?;
            }
        }

        Ok(())
    }
}

impl Node for Path {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[cfg(test)]
mod test {
    use guano_parser::ast::{helpers::padded, init, symbol::path::Path};

    #[test]
    fn test_parse_path() {
        let (span, state) = init("path::this_::is_ :: a :: test // hello");

        let (remaining, path) = padded(Path::parse)(span).unwrap();

        ptree::print_tree(&serde_value::to_value(path).unwrap()).unwrap();

        // println!("Path: {path}");
        println!("Remaining: {:?}", remaining.display());
        println!("Errors: {:?}", state.errors());
    }
}
