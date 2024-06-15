use proc_macro::TokenStream;
use quote::quote;
use syn::{self};

#[proc_macro_derive(Planet, attributes(orbital_period))]
pub fn planet_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    // Build the trait implementation
    impl_planet_macro(&ast)
}

fn impl_planet_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Iterate over the attributes and do something with them
    let orbital_period_attr = ast
        .attrs
        .iter()
        .find(|&a| a.path().is_ident("orbital_period"))
        .expect("missing the 'orbital_period' attribute");

    let mut orbital_period: f64 = 0.0;
    orbital_period_attr
        .parse_nested_meta(|meta| {
            if !meta.path.is_ident("years") {
                return Err(meta.error("the only supported key is 'years'"));
            }
            let literal: syn::Lit = meta.value()?.parse()?;

            // Try to convert the attribute value expression to a float
            orbital_period = literal_to_float(&literal)?;
            Ok(())
        })
        .expect("failed to parse the 'orbital_period' attribute");

    let gen = quote! {
        impl Planet for #name {
            const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = #orbital_period;
        }
    };
    gen.into()
}

fn literal_to_float(literal: &syn::Lit) -> syn::Result<f64> {
    match literal {
        syn::Lit::Float(f) => f.base10_parse(),
        syn::Lit::Int(i) => i.base10_parse(),
        default => Err(syn::Error::new_spanned(
            default,
            "expected a numerical literal value",
        )),
    }
}
