use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, quote};
use venial::{Error, Item, NamedFields, TupleFields};

extern crate proc_macro;

type Result<T> = std::result::Result<T, Error>;

#[proc_macro_derive(OperationSet, attributes(tickflow, tickflow_op, tickflow_arg))]
pub fn derive_operation_set_wrapper(item: TokenStream) -> TokenStream {
    match derive_operation_set(item.into()) {
        Ok(c) => c.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_operation_set(item: TokenStream2) -> Result<TokenStream2> {
    let value = venial::parse_item(item)?;

    let Item::Enum(value) = value else {
        Err(Error::new(
            "derive(OperationSet) is only supported for enums",
        ))?
    };

    let Some(attr) = value.attributes.iter().find(|c| {
        c.get_single_path_segment()
            .is_some_and(|path| *path == "tickflow")
    }) else {
        Err(Error::new(
            "OperationSet requires an attribute named tickflow",
        ))?
    };

    let name_span = attr.get_single_path_segment().unwrap().span();

    let params = kv_parser(attr.get_value_tokens())?;

    let Some(btks_type) = params.get("btks_type") else {
        Err(Error::new_at_span(
            name_span,
            "tickflow attribute missing parameter btks_type",
        ))?
    };

    let Some(endian) = params.get("endian") else {
        Err(Error::new_at_span(
            name_span,
            "tickflow attribute missing parameter endian",
        ))?
    };

    let name = value.name;

    let mut variants = vec![];

    for (variant, _) in value.variants.iter() {
        let Some(attr) = variant.attributes.iter().find(|c| {
            c.get_single_path_segment()
                .is_some_and(|path| *path == "tickflow_op")
        }) else {
            Err(Error::new_at_tokens(
                &variant.name,
                "OperationSet variants must have an attribute named tickflow_op",
            ))?
        };
        let op_params = kv_parser(attr.get_value_tokens())?;

        // TODO: default value
        let Some(op) = op_params.get("val") else {
            Err(Error::new_at_tokens(
                attr.get_single_path_segment().unwrap(),
                "tickflow_op attribute missing parameter op",
            ))?
        };
        let name = &variant.name;
        let fields = match &variant.fields {
            venial::Fields::Unit => quote! {},
            venial::Fields::Tuple(TupleFields { fields, .. }) => {
                let mut field_values = vec![];
                for (i, field) in fields.iter().enumerate() {
                    // TODO: arg0 arguments, optional arguments
                    field_values.push(quote! {todo!()})
                }
                quote! { ( #(#field_values),*) }
            }
            venial::Fields::Named(NamedFields { fields, .. }) => {
                let mut field_values = vec![];
                // TODO: arg0 arguments, optional arguments
                for (i, (field, _)) in fields.iter().enumerate() {
                    let name = &field.name;
                    field_values.push(quote! {#name: todo!()})
                }
                quote! { { #(#field_values),*} }
            }
        };

        variants.push(quote! {tickflow::tf_op!(~#op) => Self::#name #fields});
    }

    // TODO
    Ok(quote! {
        impl tickflow::data::OperationSet for #name {
            const BTKS_TICKFLOW_TYPE: tickflow::data::btks::BtksType = #btks_type;
            const ENDIAN: bytestream::ByteOrder = #endian;

            fn get_operation(op: tickflow::data::RawTickflowOp) -> Self {
                match op.as_definition() {
                    #(#variants,)*
                    _ => todo!(),
                }
            }
            fn get_call_operations() -> Vec<tickflow::data::ArgsTickflowOpDef> {
                todo!()
            }
            fn get_string_operations() -> Vec<tickflow::data::ArgsTickflowOpDef> {
                todo!()
            }
            fn get_array_operations() -> Vec<tickflow::data::ArgsTickflowOpDef> {
                todo!()
            }
            fn get_depth_operations() -> Vec<tickflow::data::TickflowOpDef> {
                todo!()
            }
            fn get_undepth_operations() -> Vec<tickflow::data::TickflowOpDef> {
                todo!()
            }
            fn get_scene_operation() -> tickflow::data::ArgsTickflowOpDef {
                todo!()
            }
            fn get_return_operations() -> Vec<tickflow::data::TickflowOpDef> {
                todo!()
            }
        }
    })
}

fn kv_parser(tokens: &[TokenTree]) -> Result<HashMap<String, TokenStream2>> {
    let pairs: Vec<_> = tokens
        .split(|t| matches!(t, TokenTree::Punct(p) if p.as_char() == ','))
        .collect();

    let mut out = HashMap::new();
    for val_arr in pairs {
        let mut val = val_arr.iter();
        let Some(TokenTree::Ident(ident)) = val.next() else {
            Err(Error::new_at_tokens(
                val_arr.iter().cloned().collect::<TokenStream2>(),
                "attribute arguments must follow a = b, ... pattern",
            ))?
        };
        if !matches!(val.next(), Some(TokenTree::Punct(p)) if p.as_char() == '=') {
            Err(Error::new_at_tokens(
                val_arr.iter().cloned().collect::<TokenStream2>(),
                "attribute arguments must follow a = b, ... pattern",
            ))?
        }

        out.insert(ident.to_string(), val.cloned().collect());
    }

    Ok(out)
}
