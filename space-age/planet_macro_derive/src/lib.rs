use proc_macro::TokenStream;
use quote::quote;
use syn::{self, spanned::Spanned};

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

    orbital_period_attr
        .meta
        .require_name_value()
        .expect("'orbital_period' is not a NameValue attribute");

    let orbital_period_expr = &orbital_period_attr
        .meta
        .require_name_value()
        .expect("'orbital_period' is not a NameValue attribute")
        .value;

    // Try to convert the attribute value expression to a float
    let orbital_period = expr_to_float(orbital_period_expr).unwrap();

    let gen = quote! {
        impl Planet for #name {
            fn orbital_period_in_earth_years() -> f64 {
                #orbital_period
            }
        }
    };
    gen.into()
}

fn expr_to_float(expr: &syn::Expr) -> Result<f64, syn::Error> {
    let literal = match expr {
        syn::Expr::Lit(l) => &l.lit,
        default => {
            return Err(syn::Error::new(
                default.span(),
                "expected a literal expression",
            ))
        }
    };

    match literal {
        syn::Lit::Float(f) => f.base10_parse(),
        syn::Lit::Int(i) => i.base10_parse(),
        default => Err(syn::Error::new(
            default.span(),
            "expected a numerical literal",
        )),
    }
}
