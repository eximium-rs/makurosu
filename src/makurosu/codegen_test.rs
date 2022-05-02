mod codegen_fn_test {
    use crate::makurosu::codegen;
    use insta::assert_debug_snapshot;
    use syn::{parse_quote, ItemMacro};

    fn build_item() -> ItemMacro {
        parse_quote!(
            macro_rules! example {
                () => {};
            }
        )
    }

    #[test]
    fn it_decorates_macro_rules_body_with_a_template_macro_invocation() {
        let item = build_item();
        let output = codegen(item);
        assert!(syn::parse2::<ItemMacro>(output.clone()).is_ok());
        assert_debug_snapshot!(output, @r###"
        TokenStream [
            Ident {
                sym: macro_rules,
            },
            Punct {
                char: '!',
                spacing: Alone,
            },
            Ident {
                sym: example,
            },
            Group {
                delimiter: Brace,
                stream: TokenStream [
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
                        stream: TokenStream [
                            Ident {
                                sym: makurosu,
                            },
                            Punct {
                                char: ':',
                                spacing: Joint,
                            },
                            Punct {
                                char: ':',
                                spacing: Alone,
                            },
                            Ident {
                                sym: template,
                            },
                            Punct {
                                char: '!',
                                spacing: Alone,
                            },
                            Group {
                                delimiter: Brace,
                                stream: TokenStream [],
                            },
                        ],
                    },
                    Punct {
                        char: ';',
                        spacing: Alone,
                    },
                ],
            },
        ]
        "###);
    }
}
