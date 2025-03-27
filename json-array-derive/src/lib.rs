use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Attribute, Lit};

/// A derive macro that allows a struct to be deserialized from a JSON array
/// with explicit field ordering.
/// 
/// This macro allows fields to specify which array index they should deserialize from
/// using the `#[order(n)]` attribute.
#[proc_macro_derive(DeserializeOrdered, attributes(order))]
pub fn deserialize_ordered(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the name of the struct
    let name = &input.ident;
    
    // Extract the fields from the struct
    let fields = match &input.data {
        Data::Struct(data_struct) => {
            match &data_struct.fields {
                Fields::Named(fields_named) => &fields_named.named,
                _ => panic!("DeserializeOrdered only supports structs with named fields"),
            }
        },
        _ => panic!("DeserializeOrdered only supports structs"),
    };
    
    // Extract field names, types and their order attributes
    let mut field_info = Vec::new();
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        
        // Find the order attribute
        let order_attr = field.attrs.iter()
            .find(|attr| attr.path().is_ident("order"))
            .unwrap_or_else(|| panic!("Field `{}` is missing #[order(n)] attribute", field_name));
        
        // Extract the index from the order attribute
        let index = extract_order_index(order_attr)
            .unwrap_or_else(|| panic!("Invalid #[order(n)] attribute for field `{}`", field_name));
        
        field_info.push((field_name, field_type, index));
    }
    
    // Find the maximum array index we need to access
    let max_index = field_info.iter()
        .map(|(_, _, index)| *index)
        .max()
        .unwrap_or(0);
    
    // Create field mapping expressions for each field
    let field_mapping = field_info.iter().map(|(field_name, field_type, index)| {
        let idx = *index; // Dereference here to use the actual usize value
        quote! {
            #field_name: {
                let value = &array_elements[#idx];
                serde_json::from_value::<#field_type>(value.clone())
                    .map_err(|e| serde::de::Error::custom(format!("Failed to deserialize field `{}` at index {}: {}", 
                        stringify!(#field_name), #idx, e)))?
            }
        }
    });
    
    // Generate the implementation
    let expanded = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::{self, Visitor, SeqAccess};
                use std::fmt;
                use std::marker::PhantomData;
                
                struct ArrayVisitor<'de> {
                    marker: PhantomData<#name>,
                    lifetime: PhantomData<&'de ()>,
                }
                
                impl<'de> Visitor<'de> for ArrayVisitor<'de> {
                    type Value = #name;
                    
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(&format!("a JSON array with at least {} elements", #max_index + 1))
                    }
                    
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        // Store all elements up to max_index
                        let mut array_elements: Vec<serde_json::Value> = vec![serde_json::Value::Null; #max_index + 1];
                        
                        // Collect array elements
                        let mut index = 0;
                        while let Some(value) = seq.next_element::<serde_json::Value>()? {
                            if index <= #max_index {
                                array_elements[index] = value;
                            }
                            index += 1;
                        }
                        
                        // Check that we have all required indices
                        if index <= #max_index {
                            return Err(serde::de::Error::invalid_length(index, &self));
                        }
                        
                        // Deserialize each field from its corresponding array element
                        Ok(#name {
                            #(#field_mapping),*
                        })
                    }
                }
                
                deserializer.deserialize_seq(ArrayVisitor {
                    marker: PhantomData,
                    lifetime: PhantomData,
                })
            }
        }
    };
    
    // Return the generated code as a token stream
    TokenStream::from(expanded)
}

// Helper function to extract the order index from an attribute
fn extract_order_index(attr: &Attribute) -> Option<usize> {
    // Parse the attribute meta
    match attr.meta.require_list().ok()?.parse_args::<Lit>().ok() {
        Some(Lit::Int(lit_int)) => lit_int.base10_parse::<usize>().ok(),
        _ => None,
    }
}
