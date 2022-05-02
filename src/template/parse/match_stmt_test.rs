mod match_arm_struct_test {
    mod peek_start_test {
        use proc_macro2::TokenStream;
        use syn::parse::{Parse, ParseStream};

        use crate::template::parse::match_stmt::MatchArm;

        struct TestCase {
            pub valid: bool
        }

        impl Parse for TestCase {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let valid = MatchArm::peek_case(input);
                let _: TokenStream = input.parse()?; // Consume stream.
                Ok(Self { valid })
            }
        }

        #[test]
        fn when_stream_does_not_start_with_a_pipe_it_returns_false() {
            let case: TestCase = syn::parse_str("&").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_a_logical_or_it_returns_false() {
            let case: TestCase = syn::parse_str("||").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_a_or_equal_it_returns_false() {
            let case: TestCase = syn::parse_str("|=").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_a_pipe_it_returns_true() {
            let case: TestCase = syn::parse_str("|").unwrap();
            assert!(case.valid);

            let case: TestCase = syn::parse_str("|false").unwrap();
            assert!(case.valid);
        }
    }

    mod parse_multi_pat_test {
        use insta::assert_debug_snapshot;
        use syn::{
            parse::{Parse, ParseStream},
            Pat
        };

        use crate::template::parse::match_stmt::MatchArm;

        struct TestCase {
            pat: Pat
        }

        impl Parse for TestCase {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let pat = MatchArm::parse_multi_pat(input)?;
                Ok(Self { pat })
            }
        }

        #[test]
        fn when_matching_arm_only_has_one_pattern() {
            let case: TestCase = syn::parse_str("false").unwrap();
            assert_debug_snapshot!(case.pat, @r###"
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
            )
            "###);
        }

        #[test]
        fn when_matching_arm_has_multiple_patterns() {
            let case: TestCase = syn::parse_str("1|2|3").unwrap();
            assert_debug_snapshot!(case.pat, @r###"
            Or(
                PatOr {
                    attrs: [],
                    leading_vert: None,
                    cases: [
                        Lit(
                            PatLit {
                                attrs: [],
                                expr: Lit(
                                    ExprLit {
                                        attrs: [],
                                        lit: Int(
                                            LitInt {
                                                token: 1,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        Or,
                        Lit(
                            PatLit {
                                attrs: [],
                                expr: Lit(
                                    ExprLit {
                                        attrs: [],
                                        lit: Int(
                                            LitInt {
                                                token: 2,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        Or,
                        Lit(
                            PatLit {
                                attrs: [],
                                expr: Lit(
                                    ExprLit {
                                        attrs: [],
                                        lit: Int(
                                            LitInt {
                                                token: 3,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    ],
                },
            )
            "###);
        }
    }

    mod parse_test {
        use insta::assert_debug_snapshot;

        use crate::template::parse::match_stmt::MatchArm;

        #[test]
        fn it_parses_stream_into_a_match_arm_model() {
            let match_arm: MatchArm =
                syn::parse_str("1|2|3 => { let _ = true; }").unwrap();
            assert_debug_snapshot!(match_arm, @r###"
            MatchArm {
                pat: Or(
                    PatOr {
                        attrs: [],
                        leading_vert: None,
                        cases: [
                            Lit(
                                PatLit {
                                    attrs: [],
                                    expr: Lit(
                                        ExprLit {
                                            attrs: [],
                                            lit: Int(
                                                LitInt {
                                                    token: 1,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            Or,
                            Lit(
                                PatLit {
                                    attrs: [],
                                    expr: Lit(
                                        ExprLit {
                                            attrs: [],
                                            lit: Int(
                                                LitInt {
                                                    token: 2,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                            Or,
                            Lit(
                                PatLit {
                                    attrs: [],
                                    expr: Lit(
                                        ExprLit {
                                            attrs: [],
                                            lit: Int(
                                                LitInt {
                                                    token: 3,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ],
                    },
                ),
                fat_arrow_token: FatArrow,
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
                            Ident {
                                sym: true,
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
                comma: None,
            }
            "###);
        }
    }
}

mod match_expr {
    mod parse_test {
        use insta::assert_debug_snapshot;

        use crate::template::parse::match_stmt::MatchExpr;

        #[test]
        fn when_expr_is_a_generic_expr() {
            let match_expr: MatchExpr = syn::parse_str("true").unwrap();
            assert_debug_snapshot!(match_expr, @r###"
            Expr(
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
            )
            "###);
        }

        #[test]
        fn when_expr_is_a_pipechain() {
            let match_expr: MatchExpr =
                syn::parse_str("%{value|default(true)}").unwrap();
            assert_debug_snapshot!(match_expr, @r###"
            Pipechain(
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
                                    value,
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
                                                                value: true,
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
            )
            "###);
        }
    }
}

mod match_struct_test {
    use insta::assert_debug_snapshot;

    use crate::template::parse::match_stmt::Match;

    mod peek_start_test {
        use proc_macro2::TokenStream;
        use syn::parse::{Parse, ParseStream};

        use crate::template::parse::match_stmt::Match;

        struct TestCase {
            pub valid: bool
        }

        impl Parse for TestCase {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let valid = Match::peek_start(input);
                let _: TokenStream = input.parse()?; // Consume stream.
                Ok(Self { valid })
            }
        }

        #[test]
        fn when_stream_does_not_start_with_percent_match_it_returns_false() {
            let case: TestCase = syn::parse_str("match value {}").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_percent_something_else_it_returns_false() {
            let case: TestCase =
                syn::parse_str("%somethingelse value {}").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_percent_match_it_returns_true() {
            let case: TestCase = syn::parse_str("%match value {}").unwrap();
            assert!(case.valid);
        }
    }

    mod parse_test {
        use crate::template::parse::match_stmt::Match;
        use insta::assert_debug_snapshot;

        #[test]
        fn it_parses_into_a_match_model() {
            let match_stmt: Match = syn::parse_str(
                r###"
                    %match value {
                        "zyz" => {}
                        "qaz" => {}
                        _ => {}
                    }
                "###
            )
            .unwrap();
            assert_debug_snapshot!(match_stmt, @r###"
        Match {
            percent_token: Rem,
            match_token: Match,
            match_expr: Expr(
                Path(
                    ExprPath {
                        attrs: [],
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        value,
                                    ),
                                    arguments: None,
                                },
                            ],
                        },
                    },
                ),
            ),
            brace_token: Brace,
            arms: [
                MatchArm {
                    pat: Lit(
                        PatLit {
                            attrs: [],
                            expr: Lit(
                                ExprLit {
                                    attrs: [],
                                    lit: Str(
                                        LitStr {
                                            token: "zyz",
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    fat_arrow_token: FatArrow,
                    body: Block {
                        brace_token: Brace,
                        nodes: [],
                    },
                    comma: None,
                },
                MatchArm {
                    pat: Lit(
                        PatLit {
                            attrs: [],
                            expr: Lit(
                                ExprLit {
                                    attrs: [],
                                    lit: Str(
                                        LitStr {
                                            token: "qaz",
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    fat_arrow_token: FatArrow,
                    body: Block {
                        brace_token: Brace,
                        nodes: [],
                    },
                    comma: None,
                },
                MatchArm {
                    pat: Wild(
                        PatWild {
                            attrs: [],
                            underscore_token: Underscore,
                        },
                    ),
                    fat_arrow_token: FatArrow,
                    body: Block {
                        brace_token: Brace,
                        nodes: [],
                    },
                    comma: None,
                },
            ],
        }
        "###);
        }
    }

    #[test]
    fn it_handles_pipechain_in_match_expr() {
        let match_stmt: Match = syn::parse_str(
            r###"
                %match %{value|default(false)} {
                    true => {}
                    false => {}
                }
            "###
        )
        .unwrap();
        assert_debug_snapshot!(match_stmt, @r###"
        Match {
            percent_token: Rem,
            match_token: Match,
            match_expr: Pipechain(
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
                                    value,
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
            brace_token: Brace,
            arms: [
                MatchArm {
                    pat: Lit(
                        PatLit {
                            attrs: [],
                            expr: Lit(
                                ExprLit {
                                    attrs: [],
                                    lit: Bool(
                                        LitBool {
                                            value: true,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                    fat_arrow_token: FatArrow,
                    body: Block {
                        brace_token: Brace,
                        nodes: [],
                    },
                    comma: None,
                },
                MatchArm {
                    pat: Lit(
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
                    fat_arrow_token: FatArrow,
                    body: Block {
                        brace_token: Brace,
                        nodes: [],
                    },
                    comma: None,
                },
            ],
        }
        "###);
    }
}
