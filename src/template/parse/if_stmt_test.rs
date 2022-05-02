mod peek_start_test {
    use crate::template::parse::if_stmt::If;
    use proc_macro2::TokenStream;
    use syn::parse::{Parse, ParseStream};

    pub struct TestCase {
        valid: bool
    }

    impl Parse for TestCase {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let valid = If::peek_start(input);
            let _: TokenStream = input.parse()?;
            Ok(TestCase { valid })
        }
    }

    #[test]
    fn when_stream_does_not_start_with_percent_if_it_returns_false() {
        let case: TestCase = syn::parse_str("if true {}").unwrap();
        assert!(!case.valid);
    }

    #[test]
    fn when_stream_starts_with_percent_something_else_it_returns_false() {
        let case: TestCase = syn::parse_str("%somethingelse true {}").unwrap();
        assert!(!case.valid);
    }

    #[test]
    fn when_stream_does_start_with_percent_if_it_returns_true() {
        let case: TestCase = syn::parse_str("%if true {}").unwrap();
        assert!(case.valid);
    }
}

mod parse_test {
    use crate::template::parse::if_stmt::If;
    use insta::assert_debug_snapshot;

    #[test]
    fn when_if_expr_is_an_expr() {
        let if_stmt: If =
            syn::parse_str("%if true { let _ = \"xyz\"; }").unwrap();
        assert_debug_snapshot!(if_stmt, @r###"
        If {
            percent_token: Rem,
            if_token: If,
            if_expr: Expr(
                Lit(
                    ExprLit {
                        attrs: [],
                        lit: Bool(
                            LitBool {
                                value: true,
                            },
                        ),
                    },
                ),
            ),
            body: Block {
                brace_token: Brace,
                nodes: [
                    Verbatim(
                        Ident {
                            sym: let,
                        },
                    ),
                    Verbatim(
                        Ident {
                            sym: _,
                        },
                    ),
                    Verbatim(
                        Punct {
                            char: '=',
                            spacing: Alone,
                        },
                    ),
                    Verbatim(
                        Literal {
                            lit: "xyz",
                        },
                    ),
                    Verbatim(
                        Punct {
                            char: ';',
                            spacing: Alone,
                        },
                    ),
                ],
            },
            otherwise: None,
        }
        "###);
    }

    #[test]
    fn when_if_expr_is_a_pipechain() {
        let if_stmt: If =
            syn::parse_str("%if %{flag|default(false)} { let _ = \"xyz\"; }")
                .unwrap();
        assert_debug_snapshot!(if_stmt, @r###"
        If {
            percent_token: Rem,
            if_token: If,
            if_expr: Pipechain(
                Pipechain {
                    percent_token: Rem,
                    brace_token: Brace,
                    patterns: [
                        Ident(
                            PatIdent {
                                attrs: [],
                                by_ref: None,
                                mutability: None,
                                ident: Ident(
                                    flag,
                                ),
                                subpat: None,
                            },
                        ),
                    ],
                    filters: [
                        PipechainFilterCall {
                            name: Ident(
                                default,
                            ),
                            paren_token: Some(
                                Paren,
                            ),
                            args: Some(
                                [
                                    Pattern(
                                        Lit(
                                            PatLit {
                                                attrs: [],
                                                expr: Lit(
                                                    ExprLit {
                                                        attrs: [],
                                                        lit: Bool(
                                                            LitBool {
                                                                value: false,
                                                            },
                                                        ),
                                                    },
                                                ),
                                            },
                                        ),
                                    ),
                                ],
                            ),
                        },
                    ],
                },
            ),
            body: Block {
                brace_token: Brace,
                nodes: [
                    Verbatim(
                        Ident {
                            sym: let,
                        },
                    ),
                    Verbatim(
                        Ident {
                            sym: _,
                        },
                    ),
                    Verbatim(
                        Punct {
                            char: '=',
                            spacing: Alone,
                        },
                    ),
                    Verbatim(
                        Literal {
                            lit: "xyz",
                        },
                    ),
                    Verbatim(
                        Punct {
                            char: ';',
                            spacing: Alone,
                        },
                    ),
                ],
            },
            otherwise: None,
        }
        "###);
    }

    #[test]
    fn when_if_stmt_has_a_else_stmt() {
        let if_stmt: If =
            syn::parse_str("%if true {} else { let _ = \"qaz\";}").unwrap();
        assert_debug_snapshot!(if_stmt, @r###"
        If {
            percent_token: Rem,
            if_token: If,
            if_expr: Expr(
                Lit(
                    ExprLit {
                        attrs: [],
                        lit: Bool(
                            LitBool {
                                value: true,
                            },
                        ),
                    },
                ),
            ),
            body: Block {
                brace_token: Brace,
                nodes: [],
            },
            otherwise: Some(
                Else(
                    Else {
                        else_token: Else,
                        body: Block {
                            brace_token: Brace,
                            nodes: [
                                Verbatim(
                                    Ident {
                                        sym: let,
                                    },
                                ),
                                Verbatim(
                                    Ident {
                                        sym: _,
                                    },
                                ),
                                Verbatim(
                                    Punct {
                                        char: '=',
                                        spacing: Alone,
                                    },
                                ),
                                Verbatim(
                                    Literal {
                                        lit: "qaz",
                                    },
                                ),
                                Verbatim(
                                    Punct {
                                        char: ';',
                                        spacing: Alone,
                                    },
                                ),
                            ],
                        },
                    },
                ),
            ),
        }
        "###);
    }

    #[test]
    fn when_if_stmts_has_a_else_if_and_else_stmts() {
        let if_stmt: If = syn::parse_str(
            "%if true {} else if false { let _ = \"qaz\"; } else { let _ = \
             \"wsx\"; }"
        )
        .unwrap();
        assert_debug_snapshot!(if_stmt, @r###"
        If {
            percent_token: Rem,
            if_token: If,
            if_expr: Expr(
                Lit(
                    ExprLit {
                        attrs: [],
                        lit: Bool(
                            LitBool {
                                value: true,
                            },
                        ),
                    },
                ),
            ),
            body: Block {
                brace_token: Brace,
                nodes: [],
            },
            otherwise: Some(
                ElseIf(
                    ElseIf {
                        else_token: Else,
                        if_token: If,
                        if_expr: Expr(
                            Lit(
                                ExprLit {
                                    attrs: [],
                                    lit: Bool(
                                        LitBool {
                                            value: false,
                                        },
                                    ),
                                },
                            ),
                        ),
                        body: Block {
                            brace_token: Brace,
                            nodes: [
                                Verbatim(
                                    Ident {
                                        sym: let,
                                    },
                                ),
                                Verbatim(
                                    Ident {
                                        sym: _,
                                    },
                                ),
                                Verbatim(
                                    Punct {
                                        char: '=',
                                        spacing: Alone,
                                    },
                                ),
                                Verbatim(
                                    Literal {
                                        lit: "qaz",
                                    },
                                ),
                                Verbatim(
                                    Punct {
                                        char: ';',
                                        spacing: Alone,
                                    },
                                ),
                            ],
                        },
                        otherwise: Some(
                            Else(
                                Else {
                                    else_token: Else,
                                    body: Block {
                                        brace_token: Brace,
                                        nodes: [
                                            Verbatim(
                                                Ident {
                                                    sym: let,
                                                },
                                            ),
                                            Verbatim(
                                                Ident {
                                                    sym: _,
                                                },
                                            ),
                                            Verbatim(
                                                Punct {
                                                    char: '=',
                                                    spacing: Alone,
                                                },
                                            ),
                                            Verbatim(
                                                Literal {
                                                    lit: "wsx",
                                                },
                                            ),
                                            Verbatim(
                                                Punct {
                                                    char: ';',
                                                    spacing: Alone,
                                                },
                                            ),
                                        ],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        }
        "###);
    }

    #[test]
    fn when_if_stmt_has_a_else_if_stmt_with_a_pipechain() {
        let if_stmt: If = syn::parse_str(
            "%if true {} else if %{flag|default(false)} { let _ = \"xyz\"; }"
        )
        .unwrap();
        assert_debug_snapshot!(if_stmt, @r###"
        If {
            percent_token: Rem,
            if_token: If,
            if_expr: Expr(
                Lit(
                    ExprLit {
                        attrs: [],
                        lit: Bool(
                            LitBool {
                                value: true,
                            },
                        ),
                    },
                ),
            ),
            body: Block {
                brace_token: Brace,
                nodes: [],
            },
            otherwise: Some(
                ElseIf(
                    ElseIf {
                        else_token: Else,
                        if_token: If,
                        if_expr: Pipechain(
                            Pipechain {
                                percent_token: Rem,
                                brace_token: Brace,
                                patterns: [
                                    Ident(
                                        PatIdent {
                                            attrs: [],
                                            by_ref: None,
                                            mutability: None,
                                            ident: Ident(
                                                flag,
                                            ),
                                            subpat: None,
                                        },
                                    ),
                                ],
                                filters: [
                                    PipechainFilterCall {
                                        name: Ident(
                                            default,
                                        ),
                                        paren_token: Some(
                                            Paren,
                                        ),
                                        args: Some(
                                            [
                                                Pattern(
                                                    Lit(
                                                        PatLit {
                                                            attrs: [],
                                                            expr: Lit(
                                                                ExprLit {
                                                                    attrs: [],
                                                                    lit: Bool(
                                                                        LitBool {
                                                                            value: false,
                                                                        },
                                                                    ),
                                                                },
                                                            ),
                                                        },
                                                    ),
                                                ),
                                            ],
                                        ),
                                    },
                                ],
                            },
                        ),
                        body: Block {
                            brace_token: Brace,
                            nodes: [
                                Verbatim(
                                    Ident {
                                        sym: let,
                                    },
                                ),
                                Verbatim(
                                    Ident {
                                        sym: _,
                                    },
                                ),
                                Verbatim(
                                    Punct {
                                        char: '=',
                                        spacing: Alone,
                                    },
                                ),
                                Verbatim(
                                    Literal {
                                        lit: "xyz",
                                    },
                                ),
                                Verbatim(
                                    Punct {
                                        char: ';',
                                        spacing: Alone,
                                    },
                                ),
                            ],
                        },
                        otherwise: None,
                    },
                ),
            ),
        }
        "###);
    }
}
