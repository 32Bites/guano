use crate::ast::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Modifier {
    Pub = 0b001,
    Veto = 0b010,
    Static = 0b100,
}

impl Modifier {
    fn index(&self) -> usize {
        use Modifier::*;
        match self {
            Pub => 0,
            Veto => 1,
            Static => 2,
        }
    }

    fn from_index(index: usize) -> Self {
        use Modifier::*;
        match index {
            0 => Pub,
            1 => Veto,
            2 => Static,
            _ => panic!("This should never, ever, ever occur"),
        }
    }
}

impl TryFrom<u8> for Modifier {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Modifier::*;
        Ok(match value {
            0b001 => Pub,
            0b010 => Veto,
            0b100 => Static,
            _ => return Err("Invalid modifier"),
        })
    }
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Modifier {
    pub fn as_str(&self) -> &'static str {
        use Modifier::*;
        match self {
            Pub => "pub",
            Veto => "veto",
            Static => "static",
        }
    }

    pub fn parse(input: Span) -> Res<Spanned<Self>> {
        map_opt(Keyword::parse, |k| {
            Some(Spanned::new(k.to_modifier()?, k.span().clone()))
        })(input)
    }
}

impl Parser<Span, Spanned<Modifier>, NomError<Span>> for Modifier {
    fn parse(&mut self, input: Span) -> nom::IResult<Span, Spanned<Modifier>, NomError<Span>> {
        verify(Self::parse, |s| s.value() == self)(input)
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Modifiers {
    /// TODO: FIX THIS
    inner: [Option<NodeSpan>; 3],
}

impl Modifiers {
    pub fn parse(
        mut p: impl Permutation<Span, Modifiers, NomError<Span>>,
    ) -> impl FnMut(Span) -> Res<Modifiers> {
        padded(move |input| p.permutation(input))
    }

    /// Returns true if the modifier wasn't already set.
    fn insert(&mut self, modifier: Spanned<Modifier>) -> bool {
        let value = *modifier.value();

        if !self.contains(value) {
            self.inner[value.index()] = Some(modifier.span().clone());
            true
        } else {
            false
        }
    }

    pub fn modifiers<'b>(&'b self) -> impl Iterator<Item = Modifier> + 'b {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.clone().map(|_| Modifier::from_index(i)))
    }

    pub fn spans<'b>(&'b self) -> impl Iterator<Item = NodeSpan> + 'b {
        self.inner.iter().filter_map(|s| s.clone())
    }

    pub fn iter<'b>(&'b self) -> impl Iterator<Item = Spanned<Modifier>> + 'b {
        self.inner
            .iter()
            .enumerate()
            .filter_map(|(i, s)| s.clone().map(|s| Spanned::new(Modifier::from_index(i), s)))
    }

    pub fn contains(&self, modifier: Modifier) -> bool {
        self[modifier].is_some()
    }

    pub fn span(&self, modifier: Modifier) -> &Option<NodeSpan> {
        &self[modifier]
    }

    pub fn is_empty(&self) -> bool {
        self.inner.iter().all(|o| o.is_none())
    }
}

impl std::ops::Index<Modifier> for Modifiers {
    type Output = Option<NodeSpan>;

    fn index(&self, index: Modifier) -> &Self::Output {
        &self.inner[index.index()]
    }
}

impl From<Spanned<Modifier>> for Modifiers {
    fn from(modifier: Spanned<Modifier>) -> Self {
        let mut modifiers = Modifiers::default();
        modifiers.insert(modifier);
        modifiers
    }
}

impl From<Modifiers> for u8 {
    fn from(m: Modifiers) -> Self {
        m.modifiers().fold(0, |a, m| a | m as u8)
    }
}

impl Permutation<Span, Modifiers, NomError<Span>> for () {
    fn permutation(&mut self, input: Span) -> nom::IResult<Span, Modifiers, NomError<Span>> {
        Ok((input, Modifiers::default()))
    }
}

impl Permutation<Span, Modifiers, NomError<Span>> for Modifier {
    fn permutation(&mut self, input: Span) -> Res<Modifiers> {
        let mut modifiers = Modifiers::default();

        let (input, modifier) = padded(opt(*self))(input)?;
        if let Some(modifier) = modifier {
            modifiers.insert(modifier);
        }

        Ok((input, modifiers))
    }
}

impl Permutation<Span, Modifiers, NomError<Span>> for (Modifier,) {
    fn permutation(&mut self, input: Span) -> nom::IResult<Span, Modifiers, NomError<Span>> {
        self.0.permutation(input)
    }
}

impl Permutation<Span, Modifiers, NomError<Span>> for (Modifier, Modifier) {
    fn permutation(&mut self, input: Span) -> nom::IResult<Span, Modifiers, NomError<Span>> {
        let parsers = (padded(opt(self.0)), padded(opt(self.1)));

        let (input, (first, second)) = permutation(parsers)(input)?;

        let mut modifiers = Modifiers::default();

        if let Some(value) = first {
            modifiers.insert(value);
        }

        if let Some(value) = second {
            if !modifiers.insert(value) {
                input
                    .extra
                    .report_error(Error(input.to_node(), "Duplicate modifier".into()))
            }
        }

        Ok((input, modifiers))
    }
}

impl Permutation<Span, Modifiers, NomError<Span>> for (Modifier, Modifier, Modifier) {
    fn permutation(&mut self, input: Span) -> nom::IResult<Span, Modifiers, NomError<Span>> {
        let parsers = (
            padded(opt(self.0)),
            padded(opt(self.1)),
            padded(opt(self.2)),
        );

        let (input, (first, second, third)) = permutation(parsers)(input)?;

        let mut modifiers = Modifiers::default();

        if let Some(value) = first {
            modifiers.insert(value);
        }

        if let Some(value) = second {
            if !modifiers.insert(value) {
                input
                    .extra
                    .report_error(Error(input.to_node(), "Duplicate modifier".into()))
            }
        }

        if let Some(value) = third {
            if !modifiers.insert(value) {
                input
                    .extra
                    .report_error(Error(input.to_node(), "Duplicate modifier".into()))
            }
        }

        Ok((input, modifiers))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Modifiers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.clone().into())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Modifiers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u8(ModifiersVisitor)
    }
}

#[cfg(feature = "serde")]
struct ModifiersVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for ModifiersVisitor {
    type Value = Modifiers;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Invalid modifiers flag")
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut inner: [Option<NodeSpan>; 3] = [None, None, None];
        let modifiers = [Modifier::Pub, Modifier::Static, Modifier::Veto];

        for m in modifiers {
            if (v | m as u8) != 0 {
                inner[m.index()] = Some(NodeSpan::default());
            }
        }

        Ok(Modifiers { inner })
    }
}
