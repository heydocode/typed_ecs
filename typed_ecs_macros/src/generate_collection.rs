use crate::schedule_contents::generate_schedule;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_plugin_collection_impl(
    fields: Vec<syn::Ident>,
    types: Vec<syn::Ident>,
    quote_fields: Vec<TokenStream>,
) -> TokenStream {
    let schedules: &[&'static str] = &[
        // Sync
        "Startup",
        "ApplyStartup",
        "AsyncStartup",
        "ApplyAsyncStartup",
        "PreUpdate",
        "ApplyPreUpdate",
        "Update",
        "ApplyUpdate",
        "PostUpdate",
        "ApplyPostUpdate",
        "AsyncUpdate",
        "ApplyAsyncUpdate",
        "ExitCheck",
        "OnExit",
    ];

    let systems: &[&'static str] = &[
        "startup",
        "apply_startup",
        "async_startup",
        "apply_async_startup",
        "pre_update",
        "apply_pre_update",
        "update",
        "apply_update",
        "post_update",
        "apply_post_update",
        "async_update",
        "apply_async_update",
        "exit_check",
        "on_exit",
    ];

    let mut impl_contents = quote! {};

    for (schedule_name, system_name) in schedules.iter().zip(systems) {
        let generated_schedule =
            generate_schedule(fields.clone(), types.clone(), schedule_name, system_name);
        impl_contents = quote! {
            #impl_contents

            #generated_schedule
        }
    }
    
    let plugin_num: usize = types.len();

    let expanded = quote! {
        pub struct GeneratedPluginCollection<SD> {
            #(#quote_fields,)*
            _marker: ::core::marker::PhantomData<SD>
        }

        impl <SD>::typed_ecs::plugin_collection::PluginCollection<SD> for GeneratedPluginCollection<SD>
        where SD: ::typed_ecs::shared_data::SharedData,
        // Even if this appears to do nothing as the hard check is done
        // in build_generated_collection, never remove it: it allows
        // lazy trait evaluation.
        #( #types: ::typed_ecs::plugin::Plugin<SD>, )*
        {
            const PLUGIN_NUM: usize = #plugin_num;
            
            #impl_contents
        }

        pub fn build_generated_collection<SD>()
        -> GeneratedPluginCollection<SD>
        where
        SD: ::typed_ecs::shared_data::SharedData,
            #( #types: ::typed_ecs::plugin::Plugin<SD>, )*
        {
            GeneratedPluginCollection::<SD> {
                #(#fields: #types::build(),)*
                _marker: ::core::marker::PhantomData
            }
        }
    };

    TokenStream::from(expanded)
}
