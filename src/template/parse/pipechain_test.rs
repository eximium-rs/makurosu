mod pipechain_filter_arg_test {
    mod parse_test {
        use insta::assert_debug_snapshot;

        use crate::template::parse::pipechain::PipechainFilterArg;

        #[test]
        fn when_input_is_a_pattern() {
            let filter_arg: PipechainFilterArg =
                syn::parse_str("false").unwrap();
            assert_debug_snapshot!(filter_arg, @r###"
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
            )
            "###);
        }

        #[test]
        fn when_input_is_not_a_pattern() {
            let filter_arg: PipechainFilterArg = syn::parse_str("pub").unwrap();
            assert_debug_snapshot!(filter_arg, @r###"
            Verbatim(
                Ident {
                    sym: pub,
                },
            )
            "###);
        }
    }
}

mod pipechain_filter_call_test {
    mod peek_start_test {
        use proc_macro2::TokenStream;
        use syn::parse::{Parse, ParseStream};

        use crate::template::parse::pipechain::PipechainFilterCall;

        struct TestCase {
            valid: bool
        }

        impl Parse for TestCase {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let valid = PipechainFilterCall::peek_start(input);
                let _: TokenStream = input.parse()?; // Consume stream.
                Ok(Self { valid })
            }
        }

        #[test]
        fn when_input_does_not_start_with_a_pipe_it_returns_false() {
            let case: TestCase = syn::parse_str("filter").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_input_starts_with_a_pipe_but_no_filter_it_returns_false() {
            let case: TestCase = syn::parse_str("|").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_input_starts_with_a_pipe_and_filter_it_returns_true() {
            let case: TestCase = syn::parse_str("|filter").unwrap();
            assert!(case.valid);
        }
    }

    mod parse_test {
        use insta::assert_debug_snapshot;

        use crate::template::parse::pipechain::PipechainFilterCall;

        #[test]
        fn when_no_args_are_provided() {
            let filter_call: PipechainFilterCall =
                syn::parse_str("|filter").unwrap();
            assert_debug_snapshot!(filter_call, @r###"
            PipechainFilterCall {
                name: Ident(
                    filter,
                ),
                paren_token: None,
                args: None,
            }
            "###);
        }

        #[test]
        fn when_args_are_provided() {
            let filter_call: PipechainFilterCall =
                syn::parse_str("|filter(true, 1, pub)").unwrap();
            assert_debug_snapshot!(filter_call, @r###"
            PipechainFilterCall {
                name: Ident(
                    filter,
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
                        Comma,
                        Pattern(
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
                        ),
                        Comma,
                        Verbatim(
                            Ident {
                                sym: pub,
                            },
                        ),
                    ],
                ),
            }
            "###);
        }
    }
}

mod pipechain {
    mod peek_start_test {
        use proc_macro2::TokenStream;
        use syn::parse::{Parse, ParseStream};

        use crate::template::parse::pipechain::Pipechain;

        struct TestCase {
            valid: bool
        }

        impl Parse for TestCase {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let valid = Pipechain::peek_start(input);
                let _: TokenStream = input.parse()?; // Consume stream.
                Ok(Self { valid })
            }
        }

        #[test]
        fn when_stream_does_not_start_with_percent_brace_it_returns_false() {
            let case: TestCase = syn::parse_str("${}").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_percent_something_else_it_returns_false() {
            let case: TestCase = syn::parse_str("%[]").unwrap();
            assert!(!case.valid);
        }

        #[test]
        fn when_stream_starts_with_percent_brace_it_returns_true() {
            let case: TestCase = syn::parse_str("%{}").unwrap();
            assert!(case.valid);
        }
    }

    mod parse_test {
        use insta::assert_debug_snapshot;

        use crate::template::parse::pipechain::Pipechain;

        #[test]
        fn it_parses_into_a_pipechain_model() {
            let pipechain: Pipechain =
                syn::parse_str("%{\"foo\"|snakecase(false)|ident}").unwrap();
            assert_debug_snapshot!(pipechain, @r###"
            Pipechain {
                percent_token: Rem,
                brace_token: Brace,
                patterns: [
                    Lit(
                        PatLit {
                            attrs: [],
                            expr: Lit(
                                ExprLit {
                                    attrs: [],
                                    lit: Str(
                                        LitStr {
                                            token: "foo",
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ],
                filters: [
                    PipechainFilterCall {
                        name: Ident(
                            snakecase,
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
                    PipechainFilterCall {
                        name: Ident(
                            ident,
                        ),
                        paren_token: None,
                        args: None,
                    },
                ],
            }
            "###);
        }
    }
}
