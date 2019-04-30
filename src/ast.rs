use combine_proc_macro::{Ident, Literal};

#[derive(Debug)]
pub struct Grammar {
    pub name: Ident,
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub name: Ident,
    pub pattern: Pattern,
}

#[derive(Debug)]
pub enum Pattern {
    Empty,


    /// A rule or terminal (e.g. `L_PAREN`)
    Ident(Ident),

    /// A literal string (e.g. `"keyword"`)
    Literal(Literal),

    /// Multiple patterns matched in a row (e.g. `L_PAREN IDENT R_PAREN`)
    Series(Vec<Pattern>),

    /// A choice between multiple patterns (e.g. `"public" | "private"`)
    Choice(Vec<Pattern>),

    /// A repetition of a pattern group (e.g. `(IDENT COLON type_expr COMMA)+`)
    Repeat { pattern: Box<Pattern>, repeat: Repeat },

    /// A pre-condition to check before parsing a tree, typically
    /// to resolve syntactic ambiguity; e.g. the `(...) => ...` in:
    ///
    /// >    lhs_expression:
    /// >        ("new" expression) => newExpression
    /// >        | expression
    /// >        ;
    ///
    Predicate { pred: Box<Pattern>, expr: Box<Pattern> },
}

impl Pattern {
    pub fn is_empty(&self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&Pattern::Empty)
    }

    /// Unwraps the top-level of the pattern tree, if possible.
    pub fn flatten_once(mut self) -> Pattern {
        match self {
            Pattern::Choice(ref mut p) if p.len() == 0 => Pattern::Empty,
            Pattern::Choice(ref mut p) if p.len() == 1 => p.drain(0..1).next().unwrap(),
            Pattern::Series(ref mut p) if p.len() == 0 => Pattern::Empty,
            Pattern::Series(ref mut p) if p.len() == 1 => p.drain(0..1).next().unwrap(),
            Pattern::Repeat { ref pattern, .. } if pattern.is_empty() => Pattern::Empty,
            Pattern::Predicate { pred, expr } => {
                if pred.is_empty() {
                    *expr
                } else {
                    Pattern::Predicate { pred, expr }
                }
            }
            _ => self,
        }
    }

    // /// Flattens the entire pattern tree recursively
    // pub fn flatten_all(self) -> Pattern { ... }
}

#[derive(Copy, Clone, Debug)]
pub enum Repeat {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}