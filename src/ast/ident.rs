#[derive(Debug, Clone, PartialEq)]
pub struct Ident<'source>(pub &'source str);

impl std::fmt::Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident<{}>", self.0)
    }
}
