use proc_macro::*;

use syn::{parse_macro_input, ItemStruct, parse::Parser, Fields, Field, punctuated::Punctuated};
use ::quote::quote;

//TODO: create traits in traits_p lib folder and implement with the macro

#[proc_macro_attribute]
pub fn scalarify(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as ItemStruct);

    if let Fields::Named(ref mut fields) = ast.fields {
        fields.named.push(Field::parse_named.parse2(quote! {pub magnitude: f64}).unwrap())
    }
    let name = &ast.ident;

    quote! {
        #ast

        impl Scalar for #name {

            fn get_magnitude(&self) -> f64 {
                if self.magnitude < 0.0 {panic!("Scalar values must be non-negative.");}
                self.magnitude
            }
        }
    }.into()

}

#[proc_macro_attribute]
pub fn vectorify(_: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as ItemStruct);
    // let _ = parse_macro_input!(args as Nothing);



    if let Fields::Named(ref mut fields) = ast.fields {
        fields.named.push(Field::parse_named.parse2(quote! {pub magnitude: f64}).unwrap());
        fields.named.push(Field::parse_named.parse2(quote! {pub angle: f64}).unwrap());
    }

    let name = &ast.ident;

    quote! {
        #[derive(Debug)]
        #ast

        impl Vector for #name {
            fn get_x_component (&self) -> f64 {
                f64::cos(self.angle.to_radians())*self.magnitude
            }

            fn get_y_component (&self) -> f64 {
                f64::sin(self.angle.to_radians())*self.magnitude
            }

            fn get_angle (&self) -> f64 {
                self.angle
            }

            fn get_magnitude (&self) -> f64 {
                self.magnitude
            }
        }
    }.into()
    /*
    This actually translates to
    match ast.field => { 
        Field::Named(mut fields) => { 
            //something
         }

    }
     */
}
