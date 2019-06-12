use quote::quote;
use syn::export::TokenStream2;
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;

pub struct CommandFields {
    pub arg_fields: Vec<syn::Field>,
    pub separator_fields: Vec<syn::Field>,
    pub prefix_field: Option<syn::Field>,
    pub trailing_field: Option<syn::Field>
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
                    let #prefix_build = Some(message.prefix?.to_string());
                }
            } else {
                quote! {
                    let #prefix_build = message.prefix?.to_string();
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

            let trailing_define = if trailing_field.ty == option_string {
                quote! {
                    let #trailing_build = Some(message.parameters?.trailing?.to_string());
                }
            } else {
                quote! {
                    let #trailing_build = message.parameters?.trailing?.to_string();
                }
            };

            method_sections.push(quote! {
                #trailing_define
            });
            struct_create_sections.push(quote! {
                #trailing: #trailing_build,
            });
        }

        // TODO: implement
        for arg_field in &self.arg_fields {
            let arg = arg_field.ident.clone().unwrap();
            let arg_build = syn::Ident::new(&format!("{}_build", arg), arg.span());

            let arg_section: TokenStream2 = if self.separator_fields.contains(&arg_field) {
                let mut separator: Option<String> = None;
                for attr in arg_field.attrs.iter() {
                    let attr = attr.parse_meta().unwrap();

                    match attr {
                        Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "separator" => {
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

                if arg_field.ty == option_vector {
                    quote! {
                        let #arg_build: Option<Vec<String>> = None;
                    }
                } else {
                    quote! {
                        let #arg_build: Vec<String> = vec![String::from(#separator)];
                    }
                }
            } else {
                if arg_field.ty == option_string {
                    quote! {
                        let #arg_build: Option<String> = None;
                    }
                } else {
                    quote! {
                        let #arg_build: String = String::from("");
                    }
                }
            };

            method_sections.push(arg_section);
            struct_create_sections.push(quote! {
                #arg: #arg_build,
            });
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
                        Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "separator" => {
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

