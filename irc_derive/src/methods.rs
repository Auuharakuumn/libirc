use quote::quote;
use syn::export::TokenStream2;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;
use std::fmt;

pub struct CommandFields {
    pub arg_fields: Vec<syn::Field>,
    pub separator_fields: Vec<syn::Field>,
    pub prefix_field: Option<syn::Field>,
    pub trailing_field: Option<syn::Field>,
}

impl fmt::Display for CommandFields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arg_fields: Vec<String> = self.arg_fields.iter().clone().filter(|field|
            field.ident.is_some()
        ).map(|field|
            field.ident.clone().unwrap().to_string()
        ).collect();
        
        let separator_fields: Vec<String> = self.separator_fields.iter().clone().filter(|field|
            field.ident.is_some()
        ).map(|field|
            field.ident.clone().unwrap().to_string()
        ).collect();

        let prefix_field: Option<String> = match self.prefix_field {
            Some(ref pf) => {
                match pf.ident {
                    Some(ref ident) => Some(ident.to_string()),
                    None => None
                }
            },
            None => None
        };
        
        let trailing_field: Option<String> = match self.trailing_field {
            Some(ref pf) => {
                match pf.ident {
                    Some(ref ident) => Some(ident.to_string()),
                    None => None
                }
            },
            None => None
        };

        let result = format!("{:?}\n{:?}\n{:?}\n{:?}", arg_fields, separator_fields, prefix_field, trailing_field);

        write!(f, "{}", &result)
    }
}

impl CommandFields {
    pub fn generate_parse_message(&self) -> TokenStream2 {
        let mut method_sections: Vec<TokenStream2> = Vec::new();
        let mut struct_create_sections: Vec<TokenStream2> = Vec::new();

        let option_string: syn::Type = syn::parse_str("Option<String>").unwrap();
        let option_vector: syn::Type = syn::parse_str("Option<Vec<String>>").unwrap();

        if let Some(prefix_field) = &self.prefix_field {
            let prefix = prefix_field.ident.clone().unwrap();
            let prefix_build = syn::Ident::new(&format!("{}_build", prefix), prefix.span());

            let prefix_define = if prefix_field.ty == option_string {
                quote! {
                    let #prefix_build = match message.prefix {
                        Some(p) => Some(p.to_string()),
                        None => None
                    };
                }
            } else {
                quote! {
                    let #prefix_build = message.prefix.ok_or(
                        IrcCommandError::new("Prefix required but none found")
                    )?.to_string();
                }
            };

            method_sections.push(quote! {
                #prefix_define
            });
            struct_create_sections.push(quote! {
                #prefix: prefix,
            });
        }

        if let Some(trailing_field) = &self.trailing_field {
            let trailing = trailing_field.ident.clone().unwrap();
            let trailing_build = syn::Ident::new(&format!("{}_build", trailing), trailing.span());
            let trailing_err = syn::Ident::new(&format!("{}_err", trailing), trailing.span());

            let trailing_define = if trailing_field.ty == option_string {
                quote! {
                    let #trailing_build = match message.parameters {
                        Some(p) => {
                            match p.trailing => {
                                Some(t) => Some(t.to_string()),
                                None => None
                            }
                        },
                        None => None
                    };
                }
            } else {
                quote! {
                    let #trailing_err = "Trailing argument required but none found";

                    let #trailing_build = message.parameters.ok_or(IrcCommandError::new(trailing_err))?
                        .trailing.ok_or(IrcCommandError::new(trailing_err))?.to_string();
                }
            };

            method_sections.push(quote! {
                #trailing_define
            });
            struct_create_sections.push(quote! {
                #trailing: #trailing_build,
            });
        }

        let mut arg_sections: Vec<TokenStream2> = Vec::new();
        let mut optional_arg_count: usize = 0;
        let argument_list = syn::Ident::new("derive_argument_list", proc_macro2::Span::call_site());

        for (i, arg_field) in self.arg_fields.iter().enumerate() {
            let arg = arg_field.ident.clone().unwrap();
            let arg_build = syn::Ident::new(&format!("{}_build", arg), arg.span());

            let arg_section: TokenStream2 = if self.separator_fields.contains(&arg_field) {
                let mut separator: Option<String> = None;
                for attr in arg_field.attrs.iter() {
                    let attr = attr.parse_meta().unwrap();

                    match attr {
                        Meta::NameValue(MetaNameValue {
                            ref ident, ref lit, ..
                        }) if ident == "separator" => {
                            if let Lit::Str(lit) = lit {
                                separator = Some(lit.value());
                            }
                        }
                        _ => {}
                    }
                }

                let separator = match separator {
                    Some(sep) => sep,
                    None => {
                        panic!("Separator attributes require a value.");
                    }
                };

                let vector = if separator == " " {
                    quote! {
                        #argument_list[#i..#argument_list.len()].to_vec()
                    }
                } else {
                    quote! {
                        #argument_list[#i].split(#separator).map(|s| String::from(s)).collect()
                    }
                };

                if arg_field.ty == option_vector {
                    optional_arg_count += 1;

                    quote! {
                        let #arg_build: Option<Vec<String>> = if #argument_list.len() > #i {
                            Some(#vector)
                        } else {
                            None
                        };
                    }
                } else {
                    quote! {
                        let #arg_build: Vec<String> = #vector;
                    }
                }
            } else {
                if arg_field.ty == option_string {
                    optional_arg_count += 1;

                    quote! {
                        let #arg_build: Option<String> = if #argument_list.len() > #i {
                            Some(#argument_list[#i].clone())
                        } else {
                            None
                        };
                    }
                } else {
                    quote! {
                        let #arg_build: String = #argument_list[#i].clone();
                    }
                }
            };

            arg_sections.push(arg_section);
            struct_create_sections.push(quote! {
                #arg: #arg_build,
            });
        }

        let arg_count = arg_sections.len();
        if arg_count > 0 {
            let arg_section = quote! {
                let #argument_list = message.parameters.ok_or(
                    IrcCommandError::new("No message parameters found, but some required")
                )?.middle.clone();

                if (#argument_list.len() < #arg_count - #optional_arg_count) {
                    let err = format!("Insufficient parameter count for {}", cmd);

                    return Err(IrcCommandError::new(err));
                }

                #(#arg_sections)*
            };

            method_sections.push(arg_section);
        }

        quote! {
            #(#method_sections)*

            Ok(Box::new(Self {
                #(#struct_create_sections)*
            }))
        }
    }

    pub fn generate_create_message(&self) -> TokenStream2 {
        let mut method_sections: Vec<TokenStream2> = Vec::new();

        let option_string: syn::Type = syn::parse_str("Option<String>").unwrap();
        let option_vector: syn::Type = syn::parse_str("Option<Vec<String>>").unwrap();

        if let Some(prefix_field) = &self.prefix_field {
            let prefix = prefix_field.ident.clone().unwrap(); // Only named fields are processed, so unwrap is fine

            let prefix_section = if prefix_field.ty == option_string {
                quote! {
                    if let Some(#prefix) = self.#prefix {
                        command_string.push_str(&format!(":{} ", #prefix));
                    }
                }
            } else {
                quote! {
                    command_string.push_str(&format!(":{} ", self.#prefix));
                }
            };

            method_sections.push(prefix_section);
        }

        method_sections.push(quote! {
            command_string.push_str(&format!("{} ", cmd));

        });

        for arg_field in &self.arg_fields {
            let arg = arg_field.ident.clone().unwrap();

            let arg_section: TokenStream2 = if self.separator_fields.contains(&arg_field) {
                let mut separator: Option<String> = None;
                for attr in arg_field.attrs.iter() {
                    let attr = attr.parse_meta().unwrap();

                    match attr {
                        Meta::NameValue(MetaNameValue {
                            ref ident, ref lit, ..
                        }) if ident == "separator" => {
                            if let Lit::Str(lit) = lit {
                                separator = Some(lit.value());
                            }
                        }
                        _ => {}
                    }
                }

                let separator = match separator {
                    Some(sep) => sep,
                    None => {
                        panic!("Separator attributes require a value.");
                    }
                };

                let arg_str = syn::Ident::new(&format!("{}_str", arg), arg.span());
                if arg_field.ty == option_vector {
                    quote! {
                        if let Some(#arg) = self.#arg.clone() {
                            let #arg_str = #arg.join(#separator);
                            command_string.push_str(&format!("{} ", #arg_str));
                        }
                    }
                } else {
                    quote! {
                        let #arg_str = self.#arg.join(#separator);
                        command_string.push_str(&format!("{} ", #arg_str));
                    }
                }
            } else {
                if arg_field.ty == option_string {
                    quote! {
                        if let Some(#arg) = self.#arg.clone() {
                            command_string.push_str(&format!("{} ", #arg));
                        }
                    }
                } else {
                    quote! {
                        command_string.push_str(&format!("{} ", self.#arg));
                    }
                }
            };

            method_sections.push(arg_section);
        }

        if let Some(trailing_field) = &self.trailing_field {
            let trailing = trailing_field.ident.clone().unwrap();

            let trailing_section = if trailing_field.ty == option_string {
                quote! {
                    if let Some(#trailing) = self.#trailing {
                        command_string.push_str(&format!(":{} ", #trailing));
                    }
                }
            } else {
                quote! {
                    command_string.push_str(&format!(":{} ", self.#trailing));
                }
            };

            method_sections.push(trailing_section);
        }

        quote! {
            #(#method_sections)*
        }
    }
}
