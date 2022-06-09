use proc_macro::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parser},
    parse_macro_input, Attribute, Data, DeriveInput, Expr, GenericArgument, ItemStruct, Lit,
    PathArguments, Type, TypePath,
};
use verlet_object::VerletObjectBase;

enum VelvetGeneric {
    Num(usize),
    Gen(syn::Ident),
}

#[proc_macro_derive(VerletObject, attributes(verlet_base))]
pub fn derive_verlet_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let base_field = if let Data::Struct(data) = input.data {
        let fields = data
            .fields
            .into_iter()
            .filter(|field| {
                field
                    .attrs
                    .iter()
                    .any(|attr| attr.path.is_ident("verlet_base"))
            })
            .collect::<Vec<_>>();

        if fields.len() > 1 {
            panic!("Only one verlet_base attribute is allowed");
        }

        if fields.is_empty() {
            panic!("verlet_base attribute is required");
        }
        fields.into_iter().next().unwrap()
    } else {
        panic!("Verlet Object Derive macro can only be applied to structs");
    };

    let field_name = base_field
        .ident
        .as_ref()
        .expect("base field is unnamed")
        .clone();
    let generic_arg = if let Type::Path(a) = base_field.ty {
        let segments = a.path.segments;
        let mut type_index = usize::MAX;
        for i in 0..segments.len() {
            let seg = &segments[i];
            if seg.ident == "VerletObjectBase" {
                type_index = i;
                break;
            }
        }
        if type_index == usize::MAX {
            panic!("verlet_base field is not of type VerletObjectBase");
        }

        if let PathArguments::AngleBracketed(a) = segments[type_index].arguments.clone() {
            let args = a.args;
            if args.len() != 1 {
                panic!("invalid number of generic arguments for VerletObjectBase<N>");
            }
            let arg = args.first().unwrap();
            if let GenericArgument::Const(Expr::Lit(lit)) = arg {
                match &lit.lit {
                    Lit::Int(value) => VelvetGeneric::Num(value.base10_digits().parse().unwrap()),
                    _ => panic!(
                        "Invalid generic argument for VerletObjectBase<N>, N must be of type usize"
                    ),
                }
            } else if let GenericArgument::Type(Type::Path(TypePath { ref path, .. })) = arg {
                if let Some(ident) = path.get_ident() {
                    VelvetGeneric::Gen(ident.clone())
                } else {
                    panic!(
                        "Invalid generic argument for VerletObjectBase<N>, N must be of type usize"
                    );
                }
            } else {
                panic!("2 Invalid generic argument for VerletObjectBase<N>");
            }
        } else {
            panic!("VerletObjectBase is a generic")
        }
    } else {
        panic!("vervelt_base field is not of type VerletObjectBase<N>")
    };

    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match generic_arg {
        VelvetGeneric::Gen(ident) => {
            quote! {
                impl #impl_generics VerletObject<#ident> for #name #ty_generics #where_clause {
                    fn get_verlet_infos_mut(&mut self) -> &mut VerletObjectBase<#ident> {
                        &mut self.#field_name
                    }

                    fn get_verlet_infos(&self) -> &VerletObjectBase<#ident> {
                        &self.#field_name
                    }
                }
            }
        }
        VelvetGeneric::Num(num) => {
            quote! {
                impl #impl_generics VerletObject<#num> for #name #ty_generics #where_clause {
                    fn get_verlet_infos_mut(&mut self) -> &mut VerletObjectBase<#num> {
                        &mut self.#field_name
                    }

                    fn get_verlet_infos(&self) -> &VerletObjectBase<#num> {
                        &self.#field_name
                    }
                }
            }
        }
    }
    .into()
}
