use crate::ast::*;
use combine::{parser as parse_fn, Parser};
use combine::parser::choice::{choice, optional};
use combine::parser::repeat::{many, many1, sep_by};
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
    ( optional((punct('#'), delim('['), many(attribute()), delim(']')).map(|(_, _, list, _)| list))
    , optional(keyword("fragment"))
    , ident()
    , punct(':')
    , pattern()
    , punct(';')
    ).map(|(attrs, _, name, _, pattern, _)| {
        Rule {
            name,
            pattern,
            attributes: attrs.unwrap_or_default()
        }
    })
});

parser!(fn attribute() -> Attribute {
    parse_fn(attribute_recursive)
});

parser!(fn attribute_recursive(input: &mut Input) -> Attribute {
    ( ident()
    , optional((delim('('), many1(attribute()), delim(')')))
    )
    .map(|(word, list)| match list {
        Some((_, attrs, _)) => Attribute::Group(word, attrs),
        None => Attribute::Word(word)
    })
    .parse_stream(input)
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

