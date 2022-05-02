mod parse_fn_test {
    use crate::makurosu::parse;
    use insta::assert_debug_snapshot;
    use quote::quote;

    #[test]
    fn when_input_is_a_macro_definition_it_returns_an_item_macro() {
        let input = quote! {
            #[makurosu]
            macro_rules! test {
                () => {}
            }
        };
        let output = parse(input);
        assert_eq!(output.ident.unwrap(), "test");
        assert_debug_snapshot!(output.mac, @r###"
        Macro {
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident(
                            macro_rules,
                        ),
                        arguments: None,
                    },
                ],
            },
            bang_token: Bang,
            delimiter: Brace(
                Brace,
            ),
            tokens: TokenStream [
                Group {
                    delimiter: Parenthesis,
                    stream: TokenStream [],
                },
                Punct {
                    char: '=',
                    spacing: Joint,
                },
                Punct {
                    char: '>',
                    spacing: Alone,
                },
                Group {
                    delimiter: Brace,
                    stream: TokenStream [],
                },
            ],
        }
        "###);
    }
}
