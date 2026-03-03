use ::proc_macro::TokenStream;
use ::quote::quote;

use ::syn::{punctuated::Punctuated, *};
use quote::format_ident;

#[proc_macro]
pub fn assemble_collection(input: TokenStream) -> TokenStream {
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
                pub #field_name: #ty
            });
            types_s.push(ident_str);
        } else {
            panic!("assemble_collection! only supports simple path types");
        }
    }

    let idents: Vec<syn::Ident> = fields.iter().map(|s| format_ident!("{}", s)).collect();

    let types: Vec<syn::Ident> = types_s.iter().map(|s| format_ident!("{}", s)).collect();

    let expanded = quote! {
        {
            // Inner scope, have to return ready PluginCollection, or buildable phantomdata.
            // Or directly the App? It seems quite complex to keep SD generic.
            struct Helper<SD: SharedData, T: Plugin<SD>>(T);
            trait AssertImpl { fn assert() {} }
            impl<SD: SharedData, T: Plugin<SD>> AssertImpl for Helper<T> {}

            #( Helper::<#types>::assert(); )*

            struct GeneratedPluginCollection<SD: SharedData> {
                __sd: PhantomData<SD>,
                #(#quote_fields,)*
            }

            impl <SD: SharedData>PluginCollection<SD> for GeneratedPluginCollection<SD> {
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
                fn post_ref_sd_update_all(&self, sd: &SD) {
                    #( self.#idents.post_update_ref_sd(sd); )*
                }
                #[inline(always)]
                fn post_mutref_sd_update_all(&self, sd: &mut SD) {
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

            // TODO! Maybe bring plugingroups back - they are very interesting because the SD requirements are enforced by the compiler.
            // In this new way of doing things the collection could have a field, that is a phantom data of all plugins added together, and
            // so their common SD. What is even cooler is that the individual SharedData requirements are saved individually - plugin per plugin.

            PhantomData::new(GeneratedPluginCollection)
        }
    };

    TokenStream::from(expanded)
}
