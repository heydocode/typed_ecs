use ::proc_macro::TokenStream;
use ::quote::quote;

use ::syn::{punctuated::Punctuated, *};
use quote::format_ident;


#[proc_macro]
pub fn generate_collection(input: TokenStream) -> TokenStream {
    let types = parse_macro_input!(
        input with Punctuated::<Type, Token![,]>::parse_terminated
    );

    let mut quote_fields = Vec::new();
    let mut fields = Vec::new();
    let mut types_s = Vec::new();
    for ty in &types {
        if let Type::Path(type_path) = ty {
            let segment = type_path.path.segments.last().unwrap();
            let ident = &segment.ident;
            let ident_str = ident.to_string();
            let field_name = format_ident!("{}", ident_str.to_lowercase());
            fields.push(field_name.clone());
            quote_fields.push(quote! {
                #field_name: #ty
            });
            types_s.push(ident_str);
        } else {
            panic!("assemble_collection! only supports simple path types");
        }
    }

    let idents: Vec<syn::Ident> = fields.iter().map(|s| format_ident!("{}", s)).collect();

    let types: Vec<syn::Ident> = types_s.iter().map(|s| format_ident!("{}", s)).collect();

    let expanded = quote! {
            use core::marker::PhantomData;
        
            struct GeneratedPluginCollection<SD> {
                #(#quote_fields,)*
                _marker: PhantomData<SD>
            }

            impl <SD>PluginCollection<SD> for GeneratedPluginCollection<SD>
            where SD: SharedData, 
            // Even if this appears to do nothing as the hard check is done
            // in build_generated_collection, never remove it: it allows
            // lazy trait evaluation.
            #( #types: Plugin<SD>, )*
            
            {
                    
                #[inline(always)]
                fn startup_ref_sd_all(&self, sd: &SD) {
                    #( self.#idents.startup_ref_sd(sd); )*
                }
                #[inline(always)]
                fn startup_mutref_sd_all(&self, sd: &mut SD) {
                    #( self.#idents.startup_mutref_sd(sd); )*
                }
                #[inline(always)]
                fn pre_update_ref_sd_all(&self, sd: &SD) {
                    #( self.#idents.pre_update_ref_sd(sd); )*
                }
                #[inline(always)]
                fn pre_update_mutref_sd_all(&self, sd: &mut SD) {
                    #( self.#idents.pre_update_mutref_sd(sd); )*
                }
                #[inline(always)]
                fn update_ref_sd_all(&self, sd: &SD) {
                    #( self.#idents.update_ref_sd(sd); )*
                }
                #[inline(always)]
                fn update_mutref_sd_all(&self, sd: &mut SD) {
                    #( self.#idents.update_mutref_sd(sd); )*
                }
                #[inline(always)]
                fn post_update_ref_sd_all(&self, sd: &SD) {
                    #( self.#idents.post_update_ref_sd(sd); )*
                }
                #[inline(always)]
                fn post_update_mutref_sd_all(&self, sd: &mut SD) {
                    #( self.#idents.post_update_mutref_sd(sd); )*
                }
                #[inline(always)]
                fn update_exit_status_with_sd_all(&self, should_exit: &mut ShouldExit, sd: &SD) {
                    #( self.#idents.update_exit_status_with_sd(should_exit, sd); )*
                }
                #[inline(always)]
                fn on_exit_all(&self, sd: &SD) {
                    #( self.#idents.on_exit(sd); )*
                }

            }

            const fn build_generated_collection<SD>()
            -> GeneratedPluginCollection<SD> 
            where 
            SD: SharedData,
                #( #types: Plugin<SD>, )*
            {
                GeneratedPluginCollection::<SD> {
                    #(#quote_fields,)*
                    _marker: PhantomData
                }
            }
    };

    TokenStream::from(expanded)
}
