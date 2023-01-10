use input::Input;
// use span::Span;

pub mod input;
pub mod span;

pub struct Display<'a, I>(&'a I);

impl std::fmt::Display for Display<'_, Input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_str().fmt(f)
    }
}

impl std::fmt::Debug for Display<'_, Input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.as_str())
    }
}

impl<X> std::fmt::Display for Display<'_, span::Span<X>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_str().fmt(f)
    }
}

impl<X> std::fmt::Debug for Display<'_, span::Span<X>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.as_str())
    }
}
