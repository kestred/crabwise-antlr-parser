use crate::ast::*;
use combine::{parser as parse_fn, Parser};
use combine::parser::choice::{choice, optional};
use combine::parser::repeat::{many1, sep_by};
use combine_proc_macro::parser;
use combine_proc_macro::parser::{delim, ident, keyword, literal, punct};
use either::Either;

parser!(fn grammar() -> Grammar {
    ( keyword("grammar")
    , ident()
    , punct(';')
    , many1::<Vec<_>, _>(rule())
    ).map(|(_, name, _, rules)| Grammar { name, rules })
});

parser!(fn rule() -> Rule {
    ( optional(keyword("fragment"))
    , ident()
    , punct(':')
    , pattern()
    , punct(';')
    ).map(|(_, name, _, pattern, _)| Rule { name, pattern })
});

parser!(fn pattern() -> Pattern {
    parse_fn(pattern_recursive)
});


parser!(fn pattern_recursive(input: &mut Input) -> Pattern {
    sep_by(series(), punct('|'))
        .map(Pattern::Choice)
        .map(Pattern::flatten_once)
        .parse_stream(input)
});

parser!(fn series() -> Pattern {
    many1(atom())
        .map(Pattern::Series)
        .map(Pattern::flatten_once)
});

parser!(fn atom() -> Pattern {
    parse_fn(atom_recursive)
});

parser!(fn atom_recursive(input: &mut Input) -> Pattern {
    choice((
        ident().map(Pattern::Ident),
        (
            literal(),
            // TODO: Add a `Pattern::Range`
            optional((
                // TODO: Implement joined punct parser
                punct('.'),
                punct('.'),
                literal(),
            )),
        ).map(|(lit, _)| Pattern::Literal(lit)),
        group().map(Pattern::flatten_once),
    ))
    .parse_stream(input)
});

parser!(fn group() -> Pattern {
    ( delim('(')
    , pattern()
    , delim(')')
    , optional(repeat_or_predicate())
    ).map(|(_, pattern, _, special)| {
        match special {
            Some(Either::Left(repeat)) => {
                Pattern::Repeat {
                    pattern: Box::new(pattern),
                    repeat,
                }
            }
            Some(Either::Right(predicate_then)) => {
                Pattern::Predicate {
                    pred: Box::new(pattern),
                    expr: Box::new(predicate_then),
                }
            }
            None => pattern,
        }
    })
});

parser!(fn repeat_or_predicate() -> Either<Repeat, Pattern> {
    choice((
        repeat().map(Either::Left),
        predicate().map(Either::Right),
    ))
});

parser!(fn repeat() -> Repeat {
    choice((
        punct('?').map(|_| Repeat::ZeroOrOne),
        punct('*').map(|_| Repeat::ZeroOrMore),
        punct('+').map(|_| Repeat::OneOrMore),
    ))
});

parser!(fn predicate() -> Pattern {
    ( punct('=')
    , punct('>')
    , series()
    ).map(|(_, _, pattern)| pattern)
});

