use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_str, FieldsNamed, Ident, Type,
};

#[derive(Debug, Clone)]
struct ItemEventDef {
    name: String,
    fields: Vec<(String, Type)>,
}

impl Parse for ItemEventDef {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let fields = input.parse::<FieldsNamed>()?;
        let fields = fields
            .named
            .iter()
            .map(|field| (field.ident.as_ref().unwrap().to_string(), field.ty.clone()));
        Ok(ItemEventDef {
            name: ident.to_string(),
            fields: fields.collect(),
        })
    }
}

#[derive(Debug, Clone)]
enum MacroInput {
    EventDefs {
        system_name: Ident,
        defs: Vec<ItemEventDef>,
    },
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let system_name: Ident = input.parse()?;
        let mut defs = Vec::new();
        while !input.is_empty() {
            let item = input.parse::<ItemEventDef>()?;
            defs.push(item);
        }
        Ok(MacroInput::EventDefs { system_name, defs })
    }
}

#[proc_macro]
pub fn create_event_system(input: TokenStream) -> TokenStream {
    let output = parse_macro_input!(input as MacroInput);

    let (system_name, defs) = match output {
        MacroInput::EventDefs { system_name, defs } => (system_name, defs.clone()),
    };

    let events_data_structs = defs.iter().filter_map(|def| {
        if def.fields.is_empty() {
            return None;
        }

        let struct_ident: Ident =
            parse_str(&format!("Event{}", def.name.to_case(Case::Pascal))).unwrap();

        let fields = def.fields.iter().map(|(name, ty)| {
            let field_ident: Ident = parse_str(name).unwrap();
            quote! {
               #field_ident: #ty
            }
        });

        Some(quote! {
            #[derive(Clone)]
            pub struct #struct_ident {
                #(#fields),*
            }
        })
    });

    let struct_fields = defs.iter().map(|def| {
        let field_name: Ident =
            parse_str(&format!("callbacks_{}", def.name.to_case(Case::Snake))).unwrap();

        let field_type: Type = if def.fields.is_empty() {
            parse_str(&format!("Vec<fn() -> bool>",)).unwrap()
        } else {
            parse_str(&format!(
                "Vec<fn(Event{}) -> bool>",
                def.name.to_case(Case::Pascal)
            ))
            .unwrap()
        };

        quote! {
           #field_name: #field_type,
        }
    });

    let constructors = defs.iter().map(|def| {
        let field_name: Ident =
            parse_str(&format!("callbacks_{}", def.name.to_case(Case::Snake))).unwrap();

        quote! {
           #field_name: Vec::new(),
        }
    });

    let register_fns = defs.iter().map(|def| {
        let fn_ident: Ident =
            parse_str(&format!("register_{}", def.name.to_case(Case::Snake))).unwrap();
        let field_ident: Ident =
            parse_str(&format!("callbacks_{}", def.name.to_case(Case::Snake))).unwrap();
        let callback_type: Type = if def.fields.is_empty() {
            parse_str(&format!("fn() -> bool",)).unwrap()
        } else {
            parse_str(&format!(
                "fn(Event{}) -> bool",
                def.name.to_case(Case::Pascal)
            ))
            .unwrap()
        };

        quote! {
            pub fn #fn_ident(&mut self, callback: #callback_type) {
                self.#field_ident.push(callback);
            }
        }
    });

    let fire_fns = defs.iter().map(|def| {
        let fn_ident: Ident =
            parse_str(&format!("fire_{}", def.name.to_case(Case::Snake))).unwrap();
        let field_ident: Ident =
            parse_str(&format!("callbacks_{}", def.name.to_case(Case::Snake))).unwrap();

        if def.fields.is_empty() {
            quote! {
                pub fn #fn_ident(&self) {
                    for callback in &self.#field_ident {
                        if callback() {
                            return;
                        }
                    }
                }
            }
        } else {
            let packet_type: Type =
                parse_str(&format!("Event{}", def.name.to_case(Case::Pascal))).unwrap();

            quote! {
                pub fn #fn_ident(&self, packet: #packet_type) {
                    for callback in &self.#field_ident {
                        if callback(packet.clone()) {
                            return;
                        }
                    }
                }
            }
        }
    });

    let expanded = quote! {
        #(#events_data_structs)*
        pub struct #system_name {
            #(#struct_fields)*
        }
        impl #system_name {
            pub fn new() -> Self {
                Self {
                    #(#constructors)*
                }
            }
            #(#register_fns)*
            #(#fire_fns)*
        }
    };

    TokenStream::from(expanded)
}
