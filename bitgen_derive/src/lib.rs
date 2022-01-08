use const_fnv1a_hash::fnv1a_hash_str_64;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(BitType)]
pub fn bit_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let vis = input.vis;
    let ident = input.ident;
    let generics = input.generics;
    match input.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                let field_idents: Vec<_> = fields
                    .named
                    .iter()
                    .map(|field| field.ident.clone().unwrap())
                    .collect();
                
                let field_ident_id: Vec<_> = field_idents.iter().map(|field| fnv1a_hash_str_64(field.to_string().as_str()) as usize).collect();
                let field_types: Vec<_> =
                    fields.named.iter().map(|field| field.ty.clone()).collect();
                let field_type_offsets: Vec<Vec<_>> = field_types.iter().enumerate().map(|(i, _)| field_types.iter().take(i).collect()).collect();


                quote! {
                    #(
                        impl #generics TupleAccess<#field_ident_id> for #ident #generics {
                            type Element = #field_types;
                            const BIT_OFFSET: usize = 0#(+<#field_type_offsets as BitType>::BITS)*;
                        }
                    )*

                    impl #generics BitType for #ident #generics {
                        const BITS: usize = 0#(+<#field_types as BitType>::BITS)*;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                            #(
                                <#field_types as BitType>::from_aligned(&aligned.#field_idents, &mut slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                offset += <#field_types as BitType>::BITS;
                            )*
                        }

                        fn to_aligned(slice: &[u8], mut offset: usize) -> Self { 
                            Self {
                                #(
                                    #field_idents: {
                                        let res = <#field_types as BitType>::to_aligned(&slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
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
                let field_type_offsets: Vec<Vec<_>> = field_types.iter().enumerate().map(|(i, _)| field_types.iter().take(i).collect()).collect();
                
                quote! {
                    #(
                        impl #generics TupleAccess<#field_idents> for #ident #generics {
                            type Element = #field_types;
                            const BIT_OFFSET: usize = 0#(+<#field_type_offsets as BitType>::BITS)*;
                        }
                    )*
                    impl #generics BitType for #ident #generics {
                        const BITS: usize = 0#(+<#field_types as BitType>::BITS)*;

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
            if num_variants == 0 {
                panic!("Cannot implement on an enum with no variants");
            }
            let bits_to_represent = if num_variants == 0 { 0 } else { std::mem::size_of::<usize>() * 8 - (num_variants - 1).leading_zeros() as usize };

            let variant_fields: Vec<_> = data.variants.iter()
                .filter_map(|variant| {
                    match &variant.fields {
                        syn::Fields::Named(fields) => Some(quote!{#fields}),
                        syn::Fields::Unnamed(fields) => Some(quote!{#fields;}),
                        syn::Fields::Unit => None,
                    }
                }).collect();

            let unit_idents: Vec<_> = data.variants.iter()
                .filter(|variant| matches!(variant.fields, syn::Fields::Unit))
                .map(|variant| variant.ident.clone()).collect();

            let unit_ident_ids: Vec<_> = unit_idents.iter().map(|ident| fnv1a_hash_str_64(ident.to_string().as_str()) as usize).collect();
                
            let unit_idents_index: Vec<quote::__private::TokenStream> = data.variants.iter()
                .enumerate()
                .filter(|(_, variant)| matches!(variant.fields, syn::Fields::Unit))
                .map(|(i, _)| proc_macro::Literal::usize_unsuffixed(i).to_string().parse::<TokenStream>().unwrap().into()).collect();

            let idents: Vec<_> = data.variants.iter()
                .filter(|variant| matches!(variant.fields, syn::Fields::Unnamed(_) | syn::Fields::Named(_)))
                .map(|variant| variant.ident.clone()).collect();
            let ident_ids: Vec<_> = idents.iter().map(|ident| fnv1a_hash_str_64(ident.to_string().as_str()) as usize).collect();
                
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
            
            let field_types: Vec<Vec<_>> = data.variants.iter()
                .filter_map(|variant| 
                    match &variant.fields {
                        syn::Fields::Named(fields) => Some(fields.named.iter().map(|field| field.ty.clone()).collect()),
                        syn::Fields::Unnamed(fields) => Some(fields.unnamed.iter().map(|field| field.ty.clone()).collect()),
                        syn::Fields::Unit => None,
                    }).collect();
           
            let uuid = uuid::Uuid::new_v4();
            let unique_wrapper_ident = format_ident!("Wrap{}{}", ident, uuid.to_string().replace('-', ""));
            let unique_idents = idents.iter().map(|id| format_ident!("{}{}{}", ident, id, uuid.to_string().replace('-', "")));

            let captured_field_idents: Vec<Vec<_>> = data.variants.iter()
                .filter_map(|variant| 
                    match &variant.fields {
                        syn::Fields::Named(fields) => Some(fields.named.iter().map(|field| format_ident!("t_{}_{}", syn::Member::Named(field.ident.clone().unwrap()), uuid.to_string().replace('-', "_"))).collect()),
                        syn::Fields::Unnamed(fields) => Some(fields.unnamed.iter().enumerate().map(|(i, _)| format_ident!("t_{}_{}", syn::Member::Unnamed(syn::Index::from(i)), uuid.to_string().replace('-', "_"))).collect()),
                        syn::Fields::Unit => None,
                    }).collect();

            let field_ident_id: Vec<Vec<_>> = field_idents.iter().map(|fields| fields.iter().map(|field| match field {
                syn::Member::Named(ident) => fnv1a_hash_str_64(ident.to_string().as_str()) as usize,
                syn::Member::Unnamed(index) => index.index as usize,
            }).collect()).collect();
            let field_type_offsets: Vec<Vec<Vec<_>>> = field_types.iter().map(|types| types.iter().enumerate().map(|(i, _)| {
                types.iter().take(i).collect()
            }).collect()).collect();

            let implementation = if num_variants == 1 {
                quote! {
                    impl #generics BitType for #ident #generics {
                        const BITS: usize = #unique_wrapper_ident(0)#(
                            .max(0#(
                                + <#field_types as BitType>::BITS
                            )*)
                        )*.0;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                            #(
                                if let Self::#idents { #(#field_idents: #captured_field_idents @ _,)* } = aligned {
                                    #(
                                        <#field_types as BitType>::from_aligned(#captured_field_idents, &mut slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                        offset += <#field_types as BitType>::BITS;
                                    )*
                                }
                            )*
                        }

                        fn to_aligned(slice: &[u8], mut offset: usize) -> Self { 
                            #(
                                Self::#unit_idents
                            )*
                            #(
                                Self::#idents {
                                    #(
                                        #field_idents: {
                                            let res = <#field_types as BitType>::to_aligned(&slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                            offset += <#field_types as BitType>::BITS;
                                            res
                                        },
                                    )*
                                }
                            )*
                        }
                    }
                }
            } else {
                quote! {
                    
                    impl #generics BitType for #ident #generics {
                        const BITS: usize = #bits_to_represent + #unique_wrapper_ident(0)#(
                            .max(0#(
                                + <#field_types as BitType>::BITS
                            )*)
                        )*.0;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                            match &aligned {
                                #(
                                    Self::#unit_idents => {
                                        U::<#bits_to_represent>::from_aligned(&U(#unit_idents_index), &mut slice[internal::get_byte_range(offset, #bits_to_represent)], offset);
                                    },
                                )*
                                #(
                                    Self::#idents { #(#field_idents: #captured_field_idents @ _,)* } => {
                                        let range = internal::get_byte_range(offset, #bits_to_represent);
                                        U::<#bits_to_represent>::from_aligned(&U(#idents_index), &mut slice[range], offset);
                                        offset += #bits_to_represent;
                                        #(
                                            <#field_types as BitType>::from_aligned(#captured_field_idents, &mut slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                            offset += <#field_types as BitType>::BITS;
                                        )*
                                    },
                                )*
                            }
                        }
                        
                        fn to_aligned(slice: &[u8], mut offset: usize) -> Self {
                            let underlying = U::<#bits_to_represent>::to_aligned(&slice[internal::get_byte_range(offset, #bits_to_represent)], offset);
                            offset += #bits_to_represent;
                            match underlying.0 {
                                #(#unit_idents_index => Self::#unit_idents,)*
                                #(#idents_index => Self::#idents {
                                    #(#field_idents: {
                                        let res = <#field_types as BitType>::to_aligned(&slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                        offset += <#field_types as BitType>::BITS;
                                        res
                                    }), *
                                },)*
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            };

            quote! {
                #(
                    impl #generics MaybeAccess<#unit_ident_ids> for #ident #generics {
                        type Element = ();
                        const BIT_OFFSET: usize = #bits_to_represent;
                        const EXPECTED: u32 = #unit_idents_index;
                    }
                )*
                #(
                    #vis struct #unique_idents #generics #variant_fields
                    #(
                        impl #generics TupleAccess<#field_ident_id> for #unique_idents #generics {
                            type Element = #field_types;
                            const BIT_OFFSET: usize = 0 #( + <#field_type_offsets as BitType>::BITS)*;
                        }
                    )*
                    impl #generics MaybeAccess<#ident_ids> for #ident #generics {
                        type Element = #unique_idents #generics;
                        const BIT_OFFSET: usize = #bits_to_represent;
                        const EXPECTED: u32 = #idents_index;
                    }
                    impl #generics BitType for #unique_idents #generics {
                        const BITS: usize = 0#(+<#field_types as BitType>::BITS)*;

                        fn from_aligned(aligned: &Self, slice: &mut [u8], mut offset: usize) {
                            #(
                                <#field_types as BitType>::from_aligned(&aligned.#field_idents, &mut slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                offset += <#field_types as BitType>::BITS;
                            )*
                        }

                        fn to_aligned(slice: &[u8], mut offset: usize) -> Self { 
                            Self {
                                #(
                                    #field_idents: {
                                        let res = <#field_types as BitType>::to_aligned(&slice[internal::get_byte_range(offset, <#field_types as BitType>::BITS)], offset % 8);
                                        offset += <#field_types as BitType>::BITS;
                                        res
                                    },
                                )*
                            }
                        }
                    }
                )*
                struct #unique_wrapper_ident(usize);
                impl #unique_wrapper_ident {
                    const fn max(&self, other: usize) -> Self {
                        #unique_wrapper_ident([self.0, other][(self.0 < other) as usize])
                    }
                }
                #implementation
            }
        },
        syn::Data::Union(_) => todo!(),
    }
    .into()
}
