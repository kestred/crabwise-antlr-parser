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
        span: #0 bytes(1557..1567)
    },
    rules: [
        Rule {
            name: Ident {
                ident: "equation",
                span: #0 bytes(1574..1582)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "expression",
                            span: #0 bytes(1593..1603)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "relop",
                            span: #0 bytes(1604..1609)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "expression",
                            span: #0 bytes(1610..1620)
                        }
                    )
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "expression",
                span: #0 bytes(1636..1646)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "term",
                            span: #0 bytes(1657..1661)
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
                                                span: #0 bytes(1664..1668)
                                            }
                                        ),
                                        Ident(
                                            Ident {
                                                ident: "MINUS",
                                                span: #0 bytes(1671..1676)
                                            }
                                        )
                                    ]
                                ),
                                Ident(
                                    Ident {
                                        ident: "term",
                                        span: #0 bytes(1678..1682)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "term",
                span: #0 bytes(1700..1704)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "factor",
                            span: #0 bytes(1715..1721)
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
                                                span: #0 bytes(1724..1729)
                                            }
                                        ),
                                        Ident(
                                            Ident {
                                                ident: "DIV",
                                                span: #0 bytes(1732..1735)
                                            }
                                        )
                                    ]
                                ),
                                Ident(
                                    Ident {
                                        ident: "factor",
                                        span: #0 bytes(1737..1743)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "factor",
                span: #0 bytes(1761..1767)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "signedAtom",
                            span: #0 bytes(1778..1788)
                        }
                    ),
                    Repeat {
                        pattern: Series(
                            [
                                Ident(
                                    Ident {
                                        ident: "POW",
                                        span: #0 bytes(1790..1793)
                                    }
                                ),
                                Ident(
                                    Ident {
                                        ident: "signedAtom",
                                        span: #0 bytes(1794..1804)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "signedAtom",
                span: #0 bytes(1822..1832)
            },
            pattern: Choice(
                [
                    Series(
                        [
                            Ident(
                                Ident {
                                    ident: "PLUS",
                                    span: #0 bytes(1843..1847)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "signedAtom",
                                    span: #0 bytes(1848..1858)
                                }
                            )
                        ]
                    ),
                    Series(
                        [
                            Ident(
                                Ident {
                                    ident: "MINUS",
                                    span: #0 bytes(1869..1874)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "signedAtom",
                                    span: #0 bytes(1875..1885)
                                }
                            )
                        ]
                    ),
                    Ident(
                        Ident {
                            ident: "atom",
                            span: #0 bytes(1896..1900)
                        }
                    )
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "atom",
                span: #0 bytes(1916..1920)
            },
            pattern: Choice(
                [
                    Ident(
                        Ident {
                            ident: "scientific",
                            span: #0 bytes(1931..1941)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "variable",
                            span: #0 bytes(1952..1960)
                        }
                    ),
                    Series(
                        [
                            Ident(
                                Ident {
                                    ident: "LPAREN",
                                    span: #0 bytes(1971..1977)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "expression",
                                    span: #0 bytes(1978..1988)
                                }
                            ),
                            Ident(
                                Ident {
                                    ident: "RPAREN",
                                    span: #0 bytes(1989..1995)
                                }
                            )
                        ]
                    )
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "scientific",
                span: #0 bytes(2011..2021)
            },
            pattern: Ident(
                Ident {
                    ident: "SCIENTIFIC_NUMBER",
                    span: #0 bytes(2032..2049)
                }
            )
        },
        Rule {
            name: Ident {
                ident: "variable",
                span: #0 bytes(2065..2073)
            },
            pattern: Ident(
                Ident {
                    ident: "VARIABLE",
                    span: #0 bytes(2084..2092)
                }
            )
        },
        Rule {
            name: Ident {
                ident: "relop",
                span: #0 bytes(2108..2113)
            },
            pattern: Choice(
                [
                    Ident(
                        Ident {
                            ident: "EQ",
                            span: #0 bytes(2124..2126)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "GT",
                            span: #0 bytes(2137..2139)
                        }
                    ),
                    Ident(
                        Ident {
                            ident: "LT",
                            span: #0 bytes(2150..2152)
                        }
                    )
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "VARIABLE",
                span: #0 bytes(2169..2177)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "VALID_ID_START",
                            span: #0 bytes(2188..2202)
                        }
                    ),
                    Repeat {
                        pattern: Ident(
                            Ident {
                                ident: "VALID_ID_CHAR",
                                span: #0 bytes(2204..2217)
                            }
                        ),
                        repeat: ZeroOrMore
                    }
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "SCIENTIFIC_NUMBER",
                span: #0 bytes(2235..2252)
            },
            pattern: Series(
                [
                    Ident(
                        Ident {
                            ident: "NUMBER",
                            span: #0 bytes(2263..2269)
                        }
                    ),
                    Repeat {
                        pattern: Series(
                            [
                                Ident(
                                    Ident {
                                        ident: "E",
                                        span: #0 bytes(2271..2272)
                                    }
                                ),
                                Repeat {
                                    pattern: Ident(
                                        Ident {
                                            ident: "SIGN",
                                            span: #0 bytes(2274..2278)
                                        }
                                    ),
                                    repeat: ZeroOrOne
                                },
                                Ident(
                                    Ident {
                                        ident: "NUMBER",
                                        span: #0 bytes(2281..2287)
                                    }
                                )
                            ]
                        ),
                        repeat: ZeroOrOne
                    }
                ]
            )
        },
        Rule {
            name: Ident {
                ident: "LPAREN",
                span: #0 bytes(2305..2311)
            },
            pattern: Literal(
                Literal { lit: Char((), suffix: None, span: Span { lo: BytePos(2322), hi: BytePos(2325), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "RPAREN",
                span: #0 bytes(2342..2348)
            },
            pattern: Literal(
                Literal { lit: Char()), suffix: None, span: Span { lo: BytePos(2359), hi: BytePos(2362), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "PLUS",
                span: #0 bytes(2379..2383)
            },
            pattern: Literal(
                Literal { lit: Char(+), suffix: None, span: Span { lo: BytePos(2394), hi: BytePos(2397), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "MINUS",
                span: #0 bytes(2414..2419)
            },
            pattern: Literal(
                Literal { lit: Char(-), suffix: None, span: Span { lo: BytePos(2430), hi: BytePos(2433), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "TIMES",
                span: #0 bytes(2450..2455)
            },
            pattern: Literal(
                Literal { lit: Char(*), suffix: None, span: Span { lo: BytePos(2466), hi: BytePos(2469), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "DIV",
                span: #0 bytes(2486..2489)
            },
            pattern: Literal(
                Literal { lit: Char(/), suffix: None, span: Span { lo: BytePos(2500), hi: BytePos(2503), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "GT",
                span: #0 bytes(2520..2522)
            },
            pattern: Literal(
                Literal { lit: Char(>), suffix: None, span: Span { lo: BytePos(2533), hi: BytePos(2536), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "LT",
                span: #0 bytes(2553..2555)
            },
            pattern: Literal(
                Literal { lit: Char(<), suffix: None, span: Span { lo: BytePos(2566), hi: BytePos(2569), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "EQ",
                span: #0 bytes(2586..2588)
            },
            pattern: Literal(
                Literal { lit: Char(=), suffix: None, span: Span { lo: BytePos(2599), hi: BytePos(2602), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "POINT",
                span: #0 bytes(2619..2624)
            },
            pattern: Literal(
                Literal { lit: Char(.), suffix: None, span: Span { lo: BytePos(2635), hi: BytePos(2638), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "POW",
                span: #0 bytes(2655..2658)
            },
            pattern: Literal(
                Literal { lit: Char(^), suffix: None, span: Span { lo: BytePos(2669), hi: BytePos(2672), ctxt: #0 } }
            )
        },
        Rule {
            name: Ident {
                ident: "WS",
                span: #0 bytes(2689..2691)
            },
            pattern: Choice(
                [
                    Literal(
                        Literal { lit: Char( ), suffix: None, span: Span { lo: BytePos(2702), hi: BytePos(2705), ctxt: #0 } }
                    ),
                    Literal(
                        Literal { lit: Char(\t), suffix: None, span: Span { lo: BytePos(2716), hi: BytePos(2720), ctxt: #0 } }
                    ),
                    Literal(
                        Literal { lit: Char(\n), suffix: None, span: Span { lo: BytePos(2731), hi: BytePos(2735), ctxt: #0 } }
                    ),
                    Literal(
                        Literal { lit: Char(\r), suffix: None, span: Span { lo: BytePos(2746), hi: BytePos(2750), ctxt: #0 } }
                    )
                ]
            )
        }
    ]
}"#);
}