use crate::ast::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Import {
    kind: ImportKind,
    span: NodeSpan,
}

impl Import {
    pub fn kind(&self) -> &ImportKind {
        &self.kind
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, kind)) = consumed(ImportKind::parse)(input)?;
        let import = Self {
            kind,
            span: span.into_node(),
        };

        Ok((input, import))
    }
}

impl std::fmt::Display for Import {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Node for Import {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone)]
pub enum ImportKind {
    Basic(Spanned<ImportItem>),
    From(Vec<Spanned<ImportItem>>, Path),
}

impl Default for ImportKind {
    fn default() -> Self {
        Self::From(vec![], Path::default())
    }
}

impl ImportKind {
    fn parse(input: Span) -> Res<Self> {
        delimited(
            preceded(Keyword::Import, ignorable),
            map(
                expect(alt((Self::parse_from, Self::parse_basic)), "Empty import"),
                move |o| o.unwrap_or_default(),
            ),
            preceded(ignorable, expect(tag(";"), "Expected a semicolon")),
        )(input)
    }

    fn parse_basic(input: Span) -> Res<Self> {
        map(ImportItem::parse, Self::Basic)(input)
    }

    fn parse_from(input: Span) -> Res<Self> {
        let (input, items) = alt((Self::parse_from_multi, Self::parse_from_single))(input)?;
        let (input, import_path) = preceded(padded(Keyword::From), Path::parse)(input)?;

        let kind = Self::From(items, import_path);

        Ok((input, kind))
    }

    fn parse_from_single(input: Span) -> Res<Vec<Spanned<ImportItem>>> {
        map(ImportItem::parse, |i| vec![i])(input)
    }

    fn parse_from_multi(input: Span) -> Res<Vec<Spanned<ImportItem>>> {
        delimited(
            tag("{"),
            map(
                pair(
                    ImportItem::parse,
                    many0(preceded(
                        padded(tag(",")),
                        map(
                            expect(ImportItem::parse, "Expected an import item"),
                            move |o| o.unwrap_or_default(),
                        ),
                    )),
                ),
                |(first, rest)| {
                    let mut items = vec![first];
                    items.extend(rest.into_iter());

                    items
                },
            ),
            expect(tag("}"), "Expected a '}'"),
        )(input)
    }
}

impl std::fmt::Display for ImportKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportKind::Basic(b) => write!(f, "import {b};"),
            ImportKind::From(items, path) => {
                if items.len() == 1 {
                    write!(f, "import {} from {path};", items[0])
                } else {
                    f.write_str("import {")?;

                    let mut iter = items.iter().peekable();

                    while let Some(item) = iter.next() {
                        if iter.peek().is_some() {
                            write!(f, "{item}, ")?;
                        } else {
                            item.fmt(f)?;
                        }
                    }
                    write!(f, "}} from {path}")
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImportItem {
    Path(ImportPath),
    Aliased(ImportPath, Iden),
}

impl Default for ImportItem {
    fn default() -> Self {
        Self::Path(ImportPath::default())
    }
}

impl ImportItem {
    pub fn parse(input: Span) -> Res<Spanned<Self>> {
        let aliased = map(
            separated_pair(ImportPath::parse, padded(Keyword::As), Iden::parse),
            |(path, alias)| Self::Aliased(path, alias),
        );
        let path = map(ImportPath::parse, Self::Path);
        let item = alt((aliased, path));

        spanned(item)(input)
    }
}

impl std::fmt::Display for ImportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportItem::Path(p) => p.fmt(f),
            ImportItem::Aliased(p, i) => write!(f, "{p} as {{}}"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImportPath {
    segments: Vec<ImportPathSegment>,
    span: NodeSpan,
}

impl ImportPath {
    pub fn segments(&self) -> &[ImportPathSegment] {
        &self.segments
    }

    pub fn span(&self) -> &NodeSpan {
        &self.span
    }

    pub fn has_wildcard(&self) -> bool {
        matches!(self.segments.last(), Some(ImportPathSegment::Wildcard))
    }

    pub fn parse(input: Span) -> Res<Self> {
        let (input, (span, segments)) = consumed(Self::parse_inner)(input)?;
        let path = Self {
            segments,
            span: span.into_node(),
        };

        Ok((input, path))
    }
    fn parse_inner(input: Span) -> Res<Vec<ImportPathSegment>> {
        let (input, mut base_segments) = map(opt(Path::parse), |o| {
            o.map_or_else(Vec::new, |p| {
                p.take_segments()
                    .into_iter()
                    .filter(|i| !i.is_empty())
                    .map(ImportPathSegment::Iden)
                    .collect()
            })
        })(input)?;

        let (input, wildcard) = opt(preceded(
            cond(base_segments.len() > 0, padded(tag("::"))),
            expect(tag("*"), "Missing iden or wildcard"),
        ))(input)?;

        if wildcard.is_some() {
            base_segments.push(ImportPathSegment::Wildcard);
        }

        Ok((input, base_segments))
    }
}

impl std::fmt::Display for ImportPath {
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

impl Node for ImportPath {
    fn span(&self) -> &NodeSpan {
        &self.span
    }
}

#[derive(Debug, Clone)]
pub enum ImportPathSegment {
    Iden(Iden),
    Wildcard,
}

impl std::fmt::Display for ImportPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportPathSegment::Iden(i) => i.fmt(f),
            ImportPathSegment::Wildcard => f.write_str("*"),
        }
    }
}
