extern crate proc_macro;
extern crate syn;
extern crate quote;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;

#[proc_macro_derive(IrcCommand)]
pub fn irc_command_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut command: Option<String> = None;

    for option in ast.attrs.into_iter() {
        let option = option.parse_meta().unwrap();

        match option {
            Meta::NameValue(MetaNameValue{ref ident, ref lit, ..}) if ident == "format" => {
                if let Lit::Str(lit) = lit {
                    command = Some(lit.value());
                }
            }
            _ => {}
        }
    }

    let command: String = match command {
        Some(cmd) => cmd,
        None => ast.ident.to_string(),
    };

    impl_irc_command(&ast, command)
}

fn impl_irc_command(ast: &syn::DeriveInput, command: String) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl IrcMessage for #name {
            fn parse_message(message: BaseMessage) -> Result<Box<Self>, Box<std::error:Error>> {
                if (message.command != #command) {
                    let err_text = format!("Wrong command text, found {} expected {}", message.command, #command);
                    // TODO: return Box(SomeError(err_text));
                }

                
            }

            fn create_message(&self) -> String {
            }
        }

        impl IrcCommand for #name {
        }
    };

    TokenStream::from(gen)
}

