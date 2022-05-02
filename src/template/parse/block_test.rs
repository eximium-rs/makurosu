mod peek_start_test {
    use crate::template::parse::Block;
    use proc_macro2::TokenStream;
    use syn::parse::{Parse, ParseStream};

    struct TestCase {
        valid: bool
    }

    impl Parse for TestCase {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let valid = Block::peek_start(input);
            let _: TokenStream = input.parse()?; // Consume stream.
            Ok(Self { valid })
        }
    }

    #[test]
    fn when_brace_delimiter_is_missing_it_returns_false() {
        let case: TestCase = syn::parse_str("[]").unwrap();
        assert!(!case.valid);
    }

    #[test]
    fn when_brace_delimiter_is_present_it_returns_true() {
        let case: TestCase = syn::parse_str("{}").unwrap();
        assert!(case.valid);
    }
}

mod parse_test {
    use crate::template::parse::Block;
    use insta::assert_debug_snapshot;

    #[test]
    fn it_parses_brace_delimited_group_into_a_block_mapping() {
        let block: Block = syn::parse_str("{ let _ = false; }").unwrap();
        assert_debug_snapshot!(block, @r###"
        Block {
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
                        sym: false,
                    },
                ),
                Verbatim(
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                ),
            ],
        }
        "###);
    }
}
