mod peek_start_test {
    use crate::template::parse::for_loop::For;
    use proc_macro2::TokenStream;
    use syn::parse::{Parse, ParseStream};

    struct TestCase {
        valid: bool
    }

    impl Parse for TestCase {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let valid = For::peek_start(input);
            let _: TokenStream = input.parse()?; // Consume stream.
            Ok(Self { valid })
        }
    }

    #[test]
    fn when_stream_does_not_start_with_percent_for_it_returns_false() {
        let case: TestCase = syn::parse_str("for i in col {}").unwrap();
        assert!(!case.valid);
    }

    #[test]
    fn when_stream_starts_with_percent_something_else_it_returns_false() {
        let case: TestCase =
            syn::parse_str("%somethingelse i in col {}").unwrap();
        assert!(!case.valid);
    }

    #[test]
    fn when_stream_starts_with_percent_for_it_returns_true() {
        let case: TestCase = syn::parse_str("%for i in col {}").unwrap();
        assert!(case.valid);
    }
}

mod parse_test {
    use crate::template::parse::for_loop::For;
    use insta::assert_debug_snapshot;

    #[test]
    fn it_parses_into_a_for_model() {
        let for_loop: For = syn::parse_str("%for i in [1, 2, 3] {}").unwrap();
        assert_debug_snapshot!(for_loop, @r###"
        For {
            percent_token: Rem,
            for_token: For,
            pat: Ident(
                PatIdent {
                    attrs: [],
                    by_ref: None,
                    mutability: None,
                    ident: Ident(
                        i,
                    ),
                    subpat: None,
                },
            ),
            in_token: In,
            expr: Array(
                ExprArray {
                    attrs: [],
                    bracket_token: Bracket,
                    elems: [
                        Lit(
                            ExprLit {
                                attrs: [],
                                lit: Int(
                                    LitInt {
                                        token: 1,
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Lit(
                            ExprLit {
                                attrs: [],
                                lit: Int(
                                    LitInt {
                                        token: 2,
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Lit(
                            ExprLit {
                                attrs: [],
                                lit: Int(
                                    LitInt {
                                        token: 3,
                                    },
                                ),
                            },
                        ),
                    ],
                },
            ),
            body: Block {
                brace_token: Brace,
                nodes: [],
            },
        }
        "###);
    }
}
