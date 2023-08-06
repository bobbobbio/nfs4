// copyright 2023 Remi Bernotavicius

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned as _;
use syn::{
    parse_macro_input, parse_quote, Arm, Attribute, ConstParam, Data, DataEnum, DeriveInput, Error,
    Expr, FieldValue, Fields, GenericArgument, GenericParam, Generics, Ident, ItemImpl,
    LifetimeParam, Pat, Result, Type, TypeParam, TypeParamBound, WhereClause,
};

fn find_repr(attrs: Vec<Attribute>) -> Result<Ident> {
    let repr_attr = attrs
        .iter()
        .find(|a| a.path().is_ident("repr"))
        .ok_or(Error::new(Span::call_site(), "missing repr attribute"))?;

    repr_attr.parse_args()
}

fn has_other_attr(attrs: &Vec<Attribute>) -> bool {
    attrs.iter().any(|a| {
        if a.path().is_ident("serde") {
            let mut is_other = false;
            a.parse_nested_meta(|meta| {
                is_other = meta.path.is_ident("other");
                Ok(())
            })
            .unwrap();
            is_other
        } else {
            false
        }
    })
}

fn generics_to_args(generics: &Generics) -> Vec<GenericArgument> {
    generics
        .params
        .iter()
        .map(|p| -> GenericArgument {
            match p {
                GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => parse_quote!(#lifetime),
                GenericParam::Type(TypeParam { ident, .. }) => parse_quote!(#ident),
                GenericParam::Const(ConstParam { ident, .. }) => parse_quote!(#ident),
            }
        })
        .collect()
}

fn type_with_generics(ident: &Ident, generics: &Generics) -> Type {
    let filtered_generics = generics_to_args(generics);
    parse_quote!(#ident <#(#filtered_generics),*>)
}

fn generate_deserialize(
    self_ident: Ident,
    self_generics: Generics,
    en: DataEnum,
    int_type: Ident,
) -> Result<ItemImpl> {
    let self_: Type = type_with_generics(&self_ident, &self_generics);
    let self_name = self_ident.to_string();

    let mut arms: Vec<Arm> = vec![];

    let mut default_arm: Arm = parse_quote! {
        _ => ::std::result::Result::Err(serde::de::Error::custom(
            ::std::format!(
                "unexpected value {disc:?} for {}",
                #self_name
            )
        ))
    };

    for v in &en.variants {
        let name = &v.ident;
        let (_, disc) = v
            .discriminant
            .as_ref()
            .ok_or(Error::new(v.span(), "variant missing discriminant"))?;

        if has_other_attr(&v.attrs) {
            if !matches!(&v.fields, Fields::Unit) {
                return Err(Error::new(v.span(), "other must be used without fields"));
            }
            default_arm = parse_quote! {
                _ => ::std::result::Result::Ok(#self_ident::#name)
            };
            continue;
        }

        match &v.fields {
            Fields::Unit => arms.push(parse_quote! {
                v if v == #disc => ::std::result::Result::Ok(#self_ident::#name)
            }),
            Fields::Named(f) => {
                let fields = f.named.iter().map(|f| -> FieldValue {
                    let ident = &f.ident;
                    parse_quote! {
                        #ident: ::serde::de::SeqAccess::next_element(&mut seq)?
                            .ok_or(::serde::de::Error::custom("expected field"))?
                    }
                });
                arms.push(parse_quote! {
                    v if v == #disc => ::std::result::Result::Ok(#self_ident::#name {
                        #(#fields),*
                    })
                });
            }
            Fields::Unnamed(f) => {
                let fields = f.unnamed.iter().map(|_| -> Expr {
                    parse_quote! {
                        ::serde::de::SeqAccess::next_element(&mut seq)?
                            .ok_or(::serde::de::Error::custom("expected field"))?
                    }
                });
                arms.push(parse_quote! {
                    v if v == #disc => ::std::result::Result::Ok(#self_ident::#name(
                        #(#fields),*
                    ))
                });
            }
        }
    }

    let mut impl_generics = self_generics.clone();
    impl_generics.params.push(parse_quote!('de));

    let impl_where_clause =
        generate_where_clause(&self_generics, parse_quote!(::serde::Deserialize<'de>));

    let visitor_params = self_generics.params.iter().map(|p| -> Type {
        match p {
            GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => parse_quote!(&#lifetime ()),
            GenericParam::Type(TypeParam { ident, .. }) => parse_quote!(#ident),
            GenericParam::Const(ConstParam { ident, .. }) => parse_quote!([(); #ident]),
        }
    });

    let self_generic_args = generics_to_args(&self_generics);

    Ok(parse_quote! {
        impl #impl_generics ::serde::Deserialize<'de> for #self_ #impl_where_clause {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<#self_, D::Error>
                where
                D: ::serde::de::Deserializer<'de>,
            {
                struct Visitor #self_generics (::std::marker::PhantomData<(#(#visitor_params),*)>);

                impl #impl_generics ::serde::de::Visitor<'de> for Visitor <#(#self_generic_args),*>
                    #impl_where_clause
                {
                    type Value = #self_;

                    fn expecting(
                        &self, formatter: &mut ::std::fmt::Formatter
                    ) -> ::std::fmt::Result {
                        ::std::fmt::Formatter::write_str(formatter, #self_name)
                    }

                    fn visit_seq<A>(
                        self, mut seq: A
                    ) -> ::std::result::Result<Self::Value, A::Error>
                        where
                            A: ::serde::de::SeqAccess<'de>
                    {
                        let disc: #int_type = ::serde::de::SeqAccess::next_element(&mut seq)?
                            .ok_or(::serde::de::Error::custom("expected discriminant"))?;
                        match disc {
                            #(#arms,)*
                            #default_arm
                        }
                    }
                }

                ::serde::de::Deserializer::deserialize_struct(
                    deserializer, #self_name, &[], Visitor(::std::marker::PhantomData)
                )
            }
        }
    })
}

fn generate_where_clause(self_generics: &Generics, bound: TypeParamBound) -> WhereClause {
    let predicates = self_generics
        .params
        .iter()
        .filter_map(|p| -> Option<TypeParam> {
            match p {
                GenericParam::Type(TypeParam { ident, .. }) => Some(parse_quote!(#ident: #bound)),
                _ => None,
            }
        });
    let impl_where_clause: WhereClause = parse_quote! {
        where
            #(#predicates),*
    };
    impl_where_clause
}

fn deserialize_with_discriminant_inner(input: DeriveInput) -> Result<ItemImpl> {
    if let Data::Enum(en) = input.data {
        let int_type = find_repr(input.attrs)?;
        generate_deserialize(input.ident, input.generics, en, int_type)
    } else {
        Err(Error::new(
            input.ident.span(),
            "Must be applied to `enum`s only",
        ))
    }
}

#[proc_macro_derive(DeserializeWithDiscriminant, attributes(serde))]
pub fn deserialize_with_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match deserialize_with_discriminant_inner(input) {
        Err(e) => e.into_compile_error().into(),
        Ok(v) => quote!(#v).into(),
    }
}

fn generate_serialize(
    self_ident: Ident,
    self_generics: Generics,
    en: DataEnum,
    int_type: Ident,
) -> Result<ItemImpl> {
    let self_: Type = type_with_generics(&self_ident, &self_generics);
    let self_name = self_ident.to_string();

    let mut match_arms: Vec<Arm> = vec![];

    for v in &en.variants {
        let name = &v.ident;
        let (_, disc) = v
            .discriminant
            .as_ref()
            .ok_or(Error::new(v.span(), "variant missing discriminant"))?;

        let mut num_fields: usize = 1;
        let mut statements: Vec<Expr> = vec![];
        statements.push(parse_quote! {
            ::serde::ser::SerializeStruct::serialize_field::<#int_type>(
                &mut state, "discriminant", &(#disc)
            )?
        });
        let pattern: Pat = match &v.fields {
            Fields::Unit => parse_quote!(#self_ident::#name),
            Fields::Named(fields) => {
                num_fields += fields.named.len();
                for f in &fields.named {
                    let f_ident = f.ident.as_ref().unwrap();
                    let f_ident_name = f_ident.to_string();

                    statements.push(parse_quote! {
                        ::serde::ser::SerializeStruct::serialize_field(
                            &mut state, #f_ident_name, #f_ident
                        )?
                    });
                }
                let field_pattern = fields.named.iter().map(|f| &f.ident);
                parse_quote!(#self_ident::#name { #(#field_pattern),* })
            }
            Fields::Unnamed(fields) => {
                num_fields += fields.unnamed.len();
                let mut field_pattern = vec![];
                for i in 0..fields.unnamed.len() {
                    let f_ident = Ident::new(&format!("field{i}"), Span::call_site());
                    let f_ident_name = f_ident.to_string();

                    statements.push(parse_quote! {
                        ::serde::ser::SerializeStruct::serialize_field(
                            &mut state, #f_ident_name, #f_ident
                        )?
                    });
                    field_pattern.push(f_ident);
                }
                parse_quote!(#self_ident::#name ( #(#field_pattern),* ))
            }
        };

        match_arms.push(parse_quote! {
            #pattern => {
                let mut state = ::serde::ser::Serializer::serialize_struct(
                    serializer, #self_name, #num_fields
                )?;
                #(#statements;)*
                ::serde::ser::SerializeStruct::end(state)
            }
        });
    }

    let impl_where_clause = generate_where_clause(&self_generics, parse_quote!(::serde::Serialize));

    Ok(parse_quote! {
        impl #self_generics ::serde::Serialize for #self_ #impl_where_clause {
            fn serialize<__S>(&self, serializer: __S) -> ::std::result::Result<__S::Ok, __S::Error>
                where
                    __S: ::serde::ser::Serializer
            {
                match self {
                    #(#match_arms),*
                }
            }
        }
    })
}

fn serialize_with_discriminant_inner(input: DeriveInput) -> Result<ItemImpl> {
    if let Data::Enum(en) = input.data {
        let int_type = find_repr(input.attrs)?;
        generate_serialize(input.ident, input.generics, en, int_type)
    } else {
        Err(Error::new(
            input.ident.span(),
            "Must be applied to `enum`s only",
        ))
    }
}

#[proc_macro_derive(SerializeWithDiscriminant, attributes(serde))]
pub fn serialize_with_discriminant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match serialize_with_discriminant_inner(input) {
        Err(e) => e.into_compile_error().into(),
        Ok(v) => quote!(#v).into(),
    }
}
