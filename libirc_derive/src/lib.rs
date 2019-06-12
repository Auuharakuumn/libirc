#![recursion_limit="128"]

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

mod find;
mod methods;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use syn::Lit;
use syn::Meta;
use syn::MetaNameValue;

use crate::find::{
    struct_find_attr_field,
    struct_find_multiple_attr_field,
    struct_find_arg_fields
};
use crate::methods::CommandFields;

#[proc_macro_derive(IrcCommand, attributes(command, prefix, trailing, separator))]
pub fn irc_command_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut command: Option<String> = None;

    //panic!(input.to_string());

    for option in ast.attrs.clone().into_iter() {
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
        None => ast.ident.clone().to_string(),
    };

    impl_irc_command(&ast, command)
}

fn impl_irc_command(ast: &syn::DeriveInput, command: String) -> TokenStream {
    let name = &ast.ident;
    let command = command.to_uppercase();

    let cmd_struct: syn::DataStruct = match &ast.data {
        syn::Data::Struct(ref struct_) => {
            struct_.clone()
        },
        _ => {
            panic!("IrcCommand may only be derived for a struct.");
        }
    };
    
    let prefix_field: Option<syn::Field> = struct_find_attr_field(&cmd_struct, "prefix");
    let separator_fields: Vec<syn::Field> = struct_find_multiple_attr_field(&cmd_struct, "separator");
    let trailing_field: Option<syn::Field> = struct_find_attr_field(&cmd_struct, "trailing");
    // Find the IRC message arguments in the order they appear in the struct
    // Argument fields either have no attribute or the separator attribute
    let arg_fields: Vec<syn::Field> = struct_find_arg_fields(&cmd_struct);

    let fields = CommandFields {
        arg_fields: arg_fields,
        separator_fields: separator_fields,
        prefix_field: prefix_field,
        trailing_field: trailing_field
    };

    let parse_body = fields.generate_parse_message();
    let create_body = fields.generate_create_message();

    let gen = quote! {
        impl IrcMessage for #name {
            fn parse_message(message: BaseMessage) -> Result<Box<Self>, Box<std::error::Error>> {
                let cmd = message.command.to_string().to_uppercase();
                if (cmd != String::from(#command)) {
                    let err_text = format!("Wrong command text, found {} expected {}", cmd, #command);

                    return Err(Box::new(IrcCommandError::new(err_text)));
                }

                #parse_body
            }

            fn create_message(&self) -> String {
                let cmd = String::from(#command);
                let mut command_string: String = String::from("");

                #create_body

                command_string
            }
        }
    };
    
    TokenStream::from(gen)
}

