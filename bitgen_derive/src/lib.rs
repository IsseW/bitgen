use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BitType)]
pub fn bit_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let generics = input.generics;
    match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                let field_idents: Vec<_> = fields
                    .named
                    .iter()
                    .map(|field| field.ident.clone())
                    .collect();
                let field_types: Vec<_> =
                    fields.named.iter().map(|field| field.ty.clone()).collect();

                quote! {
                    impl #generics BitType for #ident #generics {
                        const BITS: usize = #(<#field_types as BitType>::BITS)+*;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                            #(
                                <#field_types as BitType>::from_aligned(&aligned.#field_idents, &mut slice[offset / 8..=(offset + <#field_types as BitType>::BITS - 1) / 8], offset % 8);
                                offset += <#field_types as BitType>::BITS;
                            )*
                        }

                        fn to_aligned(slice: &[u8], mut offset: usize) -> Self { 
                            Self {
                                #(
                                    #field_idents: {
                                        let res = <#field_types as BitType>::to_aligned(&slice[offset / 8..=(offset + <#field_types as BitType>::BITS - 1) / 8], offset % 8);
                                        offset += <#field_types as BitType>::BITS;
                                        res
                                    }
                                ),*
                            }
                        }
                    }
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_idents: Vec<_> = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| syn::Index::from(i))
                    .collect();

                let field_types: Vec<_> =
                    fields.unnamed.iter().map(|field| field.ty.clone()).collect();
                
                quote! {
                    impl #generics BitType for #ident #generics {
                        const BITS: usize = #(<#field_types as BitType>::BITS)+*;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                            #(
                                <#field_types as BitType>::from_aligned(&aligned.#field_idents, &mut slice[offset / 8..=(offset + <#field_types as BitType>::BITS - 1) / 8], offset % 8);
                                offset += <#field_types as BitType>::BITS;
                            )*
                        }

                        fn to_aligned(slice: &[u8], mut offset: usize) -> Self { 
                            Self(
                                #(
                                    {
                                        let res = <#field_types as BitType>::to_aligned(&slice[offset / 8..=(offset + <#field_types as BitType>::BITS - 1) / 8], offset % 8);
                                        offset += <#field_types as BitType>::BITS;
                                        res
                                    }
                                ),*
                            )
                        }
                    }
                }
            },
            syn::Fields::Unit => {
                quote! {
                    impl #generics BitType for #ident #generics {
                        const BITS: usize = 0;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {}
                        fn to_aligned(slice: &[u8], offset: usize) -> Self { Self }
                    }
                }
            }
        },
        syn::Data::Enum(data) => {
            let num_variants = data.variants.len();
            let bits_to_represent = std::mem::size_of::<usize>() * 8 - num_variants.leading_zeros() as usize;

            let unit_idents: Vec<_> = data.variants.iter()
                .filter(|variant| matches!(variant.fields, syn::Fields::Unit))
                .map(|variant| variant.ident.clone()).collect();
            let unit_idents_index: Vec<quote::__private::TokenStream> = data.variants.iter()
                .enumerate()
                .filter(|(_, variant)| matches!(variant.fields, syn::Fields::Unit))
                .map(|(i, _)| proc_macro::Literal::usize_unsuffixed(i).to_string().parse::<TokenStream>().unwrap().into()).collect();

            let idents: Vec<_> = data.variants.iter()
                .filter(|variant| matches!(variant.fields, syn::Fields::Unnamed(_) | syn::Fields::Named(_)))
                .map(|variant| variant.ident.clone()).collect();
            let idents_index: Vec<quote::__private::TokenStream> = data.variants.iter()
                .enumerate()
                .filter(|(_, variant)| matches!(variant.fields, syn::Fields::Unnamed(_) | syn::Fields::Named(_)))
                .map(|(i, _)| proc_macro::Literal::usize_unsuffixed(i).to_string().parse::<TokenStream>().unwrap().into()).collect();

            let field_idents: Vec<Vec<_>> = data.variants.iter()
                .filter_map(|variant| 
                    match &variant.fields {
                        syn::Fields::Named(fields) => Some(fields.named.iter().map(|field| syn::Member::Named(field.ident.clone().unwrap())).collect()),
                        syn::Fields::Unnamed(fields) => Some(fields.unnamed.iter().enumerate().map(|(i, _)| syn::Member::Unnamed(syn::Index::from(i))).collect()),
                        syn::Fields::Unit => None,
                    }).collect();
            
            let captured_field_idents: Vec<Vec<_>> = data.variants.iter()
                .filter_map(|variant| 
                    match &variant.fields {
                        syn::Fields::Named(fields) => Some(fields.named.iter().map(|field| format_ident!("t{}", syn::Member::Named(field.ident.clone().unwrap()))).collect()),
                        syn::Fields::Unnamed(fields) => Some(fields.unnamed.iter().enumerate().map(|(i, _)| format_ident!("t{}", syn::Member::Unnamed(syn::Index::from(i)))).collect()),
                        syn::Fields::Unit => None,
                    }).collect();
            let field_types: Vec<Vec<_>> = data.variants.iter()
                .filter_map(|variant| 
                    match &variant.fields {
                        syn::Fields::Named(fields) => Some(fields.named.iter().map(|field| field.ty.clone()).collect()),
                        syn::Fields::Unnamed(fields) => Some(fields.unnamed.iter().map(|field| field.ty.clone()).collect()),
                        syn::Fields::Unit => None,
                    }).collect();
           
            let uuid = uuid::Uuid::new_v4();
            let unique_ident = format_ident!("Wrap{}{}", ident, uuid.to_string().replace('-', ""));
            quote! {
                struct #unique_ident(usize);
                impl #unique_ident {
                    const fn max(&self, other: usize) -> Self {
                        #unique_ident([self.0, other][(self.0 < other) as usize])
                    }
                }
                impl #generics BitType for #ident #generics {
                    const BITS: usize = #bits_to_represent + #unique_ident(0)#(
                        .max(0#(
                            + <#field_types as BitType>::BITS
                        )*)
                    )*.0;

                    fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                        match &aligned {
                            #(
                                Self::#unit_idents => {
                                    U::<#bits_to_represent>::from_aligned(&U(#unit_idents_index), &mut slice[0..=(#bits_to_represent - 1) / 8], offset);
                                }
                            ), *
                            #(
                                Self::#idents { #(#field_idents: #captured_field_idents @ _), * } => {
                                    U::<#bits_to_represent>::from_aligned(&U(#idents_index), &mut slice[0..=(#bits_to_represent - 1) / 8], offset);
                                    offset += #bits_to_represent;
                                    #(
                                        <#field_types as BitType>::from_aligned(#captured_field_idents, &mut slice[offset / 8..=(offset + <#field_types as BitType>::BITS - 1) / 8], offset % 8);
                                        offset += <#field_types as BitType>::BITS;
                                    ) *
                                }
                            ), *
                        }
                    }
                    
                    fn to_aligned(slice: &[u8], mut offset: usize) -> Self {
                        let underlying = U::<#bits_to_represent>::to_aligned(&slice[0..=(#bits_to_represent - 1) / 8], offset);
                        offset += #bits_to_represent;
                        match underlying.0 {
                            #(#unit_idents_index => Self::#unit_idents,)*
                            #(#idents_index => Self::#idents {
                                #(#field_idents: {
                                    let res = <#field_types as BitType>::to_aligned(&slice[offset / 8..=(offset + <#field_types as BitType>::BITS - 1) / 8], offset % 8);
                                    offset += <#field_types as BitType>::BITS;
                                    res
                                }), *
                            },)*
                            _ => unreachable!(),
                        }
                    }
                }
            }
        },
        syn::Data::Union(_) => todo!(),
    }
    .into()
}
