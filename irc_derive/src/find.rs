use syn::Meta;
use syn::MetaNameValue;

pub fn struct_find_attr_field(struct_: &syn::DataStruct, attr_name: &str) -> Option<syn::Field> {
    let mut attr_field: Option<syn::Field> = None;

    match struct_.fields {
        syn::Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                for attr in field.attrs.iter() {
                    let attr = attr.parse_meta().unwrap();

                    match attr {
                        //Meta::NameValue(MetaNameValue { ref ident, .. }) if ident == attr_name => {
                        Meta::NameValue(MetaNameValue { ref ident, .. }) => {
                            println!("Found {}, looking for {}", ident, attr_name);
                            if ident == attr_name {
                                attr_field = Some(field.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }

    attr_field
}

pub fn struct_find_multiple_attr_field(
    struct_: &syn::DataStruct,
    attr_name: &str,
) -> Vec<syn::Field> {
    let mut attr_fields: Vec<syn::Field> = Vec::new();

    match struct_.fields {
        syn::Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                for attr in field.attrs.iter() {
                    let attr = attr.parse_meta().unwrap();

                    match attr {
                        Meta::NameValue(MetaNameValue { ref ident, .. }) if ident == attr_name => {
                            attr_fields.push(field.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }

    attr_fields
}

pub fn struct_find_arg_fields(struct_: &syn::DataStruct) -> Vec<syn::Field> {
    let mut arg_fields: Vec<syn::Field> = Vec::new();

    match struct_.fields {
        syn::Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                let mut attribute_name: String = String::from("");

                for attr in field.attrs.iter() {
                    let attr = attr.parse_meta().unwrap();

                    match attr {
                        Meta::NameValue(MetaNameValue { ref ident, .. }) => {
                            attribute_name = ident.to_string();
                        }
                        _ => {}
                    }
                }

                if attribute_name != "prefix" && attribute_name != "trailing" {
                    arg_fields.push(field.clone());
                }
            }
        }
        _ => {}
    }

    arg_fields
}
