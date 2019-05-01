/*
BSD License

Copyright (c) 2013, Tom Everett
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions
are met:

1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. Neither the name of Tom Everett nor the names of its contributors
   may be used to endorse or promote products derived from this software
   without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

macro_example::const_antlr_ast_text! {
    grammar ARITHMETIC;

    equation
        : expression relop expression
        ;

    #[recursive]
    expression
        : term ((PLUS | MINUS) term)*
        ;

    term
        : factor ((TIMES | DIV) factor)*
        ;

    factor
        : signedAtom (POW signedAtom)*
        ;

    signedAtom
        : PLUS signedAtom
        | MINUS signedAtom
        | atom
        ;

    atom
        : scientific
        | variable
        | LPAREN expression RPAREN
        ;

    scientific
        : SCIENTIFIC_NUMBER
        ;

    variable
        : VARIABLE
        ;

    relop
        : EQ
        | GT
        | LT
        ;


    VARIABLE
        : VALID_ID_START (VALID_ID_CHAR)*
        ;

    SCIENTIFIC_NUMBER
        : NUMBER (E (SIGN)? NUMBER)?
        ;

    LPAREN
        : '('
        ;


    RPAREN
        : ')'
        ;


    PLUS
        : '+'
        ;


    MINUS
        : '-'
        ;


    TIMES
        : '*'
        ;


    DIV
        : '/'
        ;


    GT
        : '>'
        ;


    LT
        : '<'
        ;


    EQ
        : '='
        ;


    POINT
        : '.'
        ;


    POW
        : '^'
        ;


    WS
        : ' '
        | '\t'
        | '\n'
        | '\r'
        ;
}

#[test]
fn test_inspect_ast() {
    assert_eq!(ARITHMETIC, r#"Grammar {
    name: Ident {
        ident: "ARITHMETIC",
        span: #3 bytes(1506..2779)
    },
    rules: [
        Rule {
            name: Ident {
                ident: "equation",
                span: #3 bytes(1506..2779)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "expression",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "relop",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "expression",
                            span: #3 bytes(1506..2779)
                        }
                    )
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "expression",
                span: #3 bytes(1506..2779)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "term",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Repeat {
                        pattern: Series(
                            [
                                Choice(
                                    [
                                        Ident(
                                            Ident {
                                                ident: "PLUS",
                                                span: #3 bytes(1506..2779)
                                            }
                                        ),
                                        Ident(
                                            Ident {
                                                ident: "MINUS",
                                                span: #3 bytes(1506..2779)
                                            }
                                        )
                                    ]
                                ),
                                Ident(
                                    Ident {
                                        ident: "term",
                                        span: #3 bytes(1506..2779)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            ),
            attributes: [
                Word(
                    Ident {
                        ident: "recursive",
                        span: #3 bytes(1506..2779)
                    }
                )
            ]
        },
        Rule {
            name: Ident {
                ident: "term",
                span: #3 bytes(1506..2779)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "factor",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Repeat {
                        pattern: Series(
                            [
                                Choice(
                                    [
                                        Ident(
                                            Ident {
                                                ident: "TIMES",
                                                span: #3 bytes(1506..2779)
                                            }
                                        ),
                                        Ident(
                                            Ident {
                                                ident: "DIV",
                                                span: #3 bytes(1506..2779)
                                            }
                                        )
                                    ]
                                ),
                                Ident(
                                    Ident {
                                        ident: "factor",
                                        span: #3 bytes(1506..2779)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "factor",
                span: #3 bytes(1506..2779)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "signedAtom",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Repeat {
                        pattern: Series(
                            [
                                Ident(
                                    Ident {
                                        ident: "POW",
                                        span: #3 bytes(1506..2779)
                                    }
                                ),
                                Ident(
                                    Ident {
                                        ident: "signedAtom",
                                        span: #3 bytes(1506..2779)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "signedAtom",
                span: #3 bytes(1506..2779)
            },
            pattern: Choice(
                [
                    Series(
                        [
                            Ident(
                                Ident {
                                    ident: "PLUS",
                                    span: #3 bytes(1506..2779)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "signedAtom",
                                    span: #3 bytes(1506..2779)
                                }
                            )
                        ]
                    ),
                    Series(
                        [
                            Ident(
                                Ident {
                                    ident: "MINUS",
                                    span: #3 bytes(1506..2779)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "signedAtom",
                                    span: #3 bytes(1506..2779)
                                }
                            )
                        ]
                    ),
                    Ident(
                        Ident {
                            ident: "atom",
                            span: #3 bytes(1506..2779)
                        }
                    )
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "atom",
                span: #3 bytes(1506..2779)
            },
            pattern: Choice(
                [
                    Ident(
                        Ident {
                            ident: "scientific",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "variable",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Series(
                        [
                            Ident(
                                Ident {
                                    ident: "LPAREN",
                                    span: #3 bytes(1506..2779)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "expression",
                                    span: #3 bytes(1506..2779)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "RPAREN",
                                    span: #3 bytes(1506..2779)
                                }
                            )
                        ]
                    )
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "scientific",
                span: #3 bytes(1506..2779)
            },
            pattern: Ident(
                Ident {
                    ident: "SCIENTIFIC_NUMBER",
                    span: #3 bytes(1506..2779)
                }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "variable",
                span: #3 bytes(1506..2779)
            },
            pattern: Ident(
                Ident {
                    ident: "VARIABLE",
                    span: #3 bytes(1506..2779)
                }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "relop",
                span: #3 bytes(1506..2779)
            },
            pattern: Choice(
                [
                    Ident(
                        Ident {
                            ident: "EQ",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "GT",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "LT",
                            span: #3 bytes(1506..2779)
                        }
                    )
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "VARIABLE",
                span: #3 bytes(1506..2779)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "VALID_ID_START",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Repeat {
                        pattern: Ident(
                            Ident {
                                ident: "VALID_ID_CHAR",
                                span: #3 bytes(1506..2779)
                            }
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "SCIENTIFIC_NUMBER",
                span: #3 bytes(1506..2779)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "NUMBER",
                            span: #3 bytes(1506..2779)
                        }
                    ),
                    Repeat {
                        pattern: Series(
                            [
                                Ident(
                                    Ident {
                                        ident: "E",
                                        span: #3 bytes(1506..2779)
                                    }
                                ),
                                Repeat {
                                    pattern: Ident(
                                        Ident {
                                            ident: "SIGN",
                                            span: #3 bytes(1506..2779)
                                        }
                                    ),
                                    repeat: ZeroOrOne
                                },
                                Ident(
                                    Ident {
                                        ident: "NUMBER",
                                        span: #3 bytes(1506..2779)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrOne
                    }
                ]
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "LPAREN",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char((), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "RPAREN",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char()), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "PLUS",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(+), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "MINUS",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(-), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "TIMES",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(*), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "DIV",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(/), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "GT",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(>), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "LT",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(<), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "EQ",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(=), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "POINT",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(.), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "POW",
                span: #3 bytes(1506..2779)
            },
            pattern: Literal(
                Literal { lit: Char(^), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
            ),
            attributes: []
        },
        Rule {
            name: Ident {
                ident: "WS",
                span: #3 bytes(1506..2779)
            },
            pattern: Choice(
                [
                    Literal(
                        Literal { lit: Char( ), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
                    ),
                    Literal(
                        Literal { lit: Char(\t), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
                    ),
                    Literal(
                        Literal { lit: Char(\n), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
                    ),
                    Literal(
                        Literal { lit: Char(\r), suffix: None, span: Span { lo: BytePos(1506), hi: BytePos(2779), ctxt: #3 } }
                    )
                ]
            ),
            attributes: []
        }
    ]
}"#);
}