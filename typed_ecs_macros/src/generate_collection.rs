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
        "AsyncOnExit",
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
        "async_on_exit",
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

    let expanded = quote! {
        use core::marker::PhantomData as PhantomDataUsedByTypedEcsMacro;
        use typed_ecs::plugin_collection::PluginCollection as PluginCollectionUsedByTypedEcsMacro;
        use typed_ecs::shared_data::SharedData as SharedDataUsedByTypedEcsMacro;
        use typed_ecs::plugin::Plugin as PluginUsedByTypedEcsMacro;
        use typed_ecs::futures as futures_used_by_typed_ecs_macro;

        pub struct GeneratedPluginCollection<SD> {
            #(#quote_fields,)*
            _marker: PhantomDataUsedByTypedEcsMacro<SD>
        }

        impl <SD>PluginCollectionUsedByTypedEcsMacro<SD> for GeneratedPluginCollection<SD>
        where SD: SharedDataUsedByTypedEcsMacro,
        // Even if this appears to do nothing as the hard check is done
        // in build_generated_collection, never remove it: it allows
        // lazy trait evaluation.
        #( #types: PluginUsedByTypedEcsMacro<SD>, )*
        {
            #impl_contents
        }

        pub fn build_generated_collection<SD>()
        -> GeneratedPluginCollection<SD>
        where
        SD: SharedDataUsedByTypedEcsMacro,
            #( #types: PluginUsedByTypedEcsMacro<SD>, )*
        {
            GeneratedPluginCollection::<SD> {
                #(#fields: #types::build(),)*
                _marker: PhantomDataUsedByTypedEcsMacro
            }
        }
    };

    TokenStream::from(expanded)
}
