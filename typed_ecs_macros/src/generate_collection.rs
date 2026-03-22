use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn generate_plugin_collection_impl(
    fields: Vec<syn::Ident>,
    types: Vec<syn::Ident>,
    quote_fields: Vec<TokenStream>,
) -> TokenStream {
    let schedules: &[&'static str] = &[
        // Async
        "AsyncPreStartupRef",
        "AsyncPreStartupMutRef",
        "AsyncStartupRef",
        "AsyncStartupMutRef",
        "AsyncPostStartupRef",
        "AsyncPostStartupMutRef",
        "AsyncPreUpdateRef",
        "AsyncPreUpdateMutRef",
        "AsyncUpdateRef",
        "AsyncUpdateMutRef",
        "AsyncPostUpdateRef",
        "AsyncPostUpdateMutRef",
        "AsyncExitCheck",
        "AsyncOnExit",
        // Sync
        "PreStartupRef",
        "PreStartupMutRef",
        "StartupRef",
        "StartupMutRef",
        "PostStartupRef",
        "PostStartupMutRef",
        "PreUpdateRef",
        "PreUpdateMutRef",
        "UpdateRef",
        "UpdateMutRef",
        "PostUpdateRef",
        "PostUpdateMutRef",
        "ExitCheck",
        "OnExit",
    ];

    let systems: &[&'static str] = &[
        // Async
        "async_pre_startup_ref_sd",
        "async_pre_startup_mutref_sd",
        "async_startup_ref_sd",
        "async_startup_mutref_sd",
        "async_post_startup_ref_sd",
        "async_post_startup_mutref_sd",
        "async_pre_update_ref_sd",
        "async_pre_update_mutref_sd",
        "async_update_ref_sd",
        "async_update_mutref_sd",
        "async_post_update_ref_sd",
        "async_post_update_mutref_sd",
        "async_exit_check",
        "async_on_exit",
        // Sync
        "pre_startup_ref_sd",
        "pre_startup_mutref_sd",
        "startup_ref_sd",
        "startup_mutref_sd",
        "post_startup_ref_sd",
        "post_startup_mutref_sd",
        "pre_update_ref_sd",
        "pre_update_mutref_sd",
        "update_ref_sd",
        "update_mutref_sd",
        "post_update_ref_sd",
        "post_update_mutref_sd",
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

fn generate_schedule(
    fields: Vec<syn::Ident>,
    types: Vec<syn::Ident>,
    schedule_name: &'static str,
    system_name: &'static str,
) -> TokenStream {
    let system_group_name = format!("{}_all", system_name);
    let is_async = schedule_name.contains("Async");
    let is_mut = system_name.contains("mutref");
    let exit_check = system_name.contains("exit_check");

    let q_group = format_ident!("{}", system_group_name);
    let q_schedule = format_ident!("{}", schedule_name);
    let q_system = format_ident!("{}", system_name);

    if is_async != system_name.starts_with("async_") {
        panic!(
            "Mismatch between schedule ({}) and system name ({})",
            schedule_name, system_name
        );
    }

    if is_async {
        if exit_check {
            quote! {
                #[inline(always)]
                async fn #q_group(&mut self, should_exit: &mut ShouldExit, sd: &SD) {
                    let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                    #(
                        {
                            let _sys_guard = Self::on_system_start(
                                stringify!(#q_schedule),
                                stringify!(#types),
                                stringify!(#q_system),
                            );
                            self.#fields.#q_system(should_exit, sd).await;
                        }
                    )*
                }
            }
        } else if is_mut {
            quote! {
                #[inline(always)]
                async fn #q_group(&mut self, sd: &mut SD) {
                    let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                    #(
                        {
                            let _sys_guard = Self::on_system_start(
                                stringify!(#q_schedule),
                                stringify!(#types),
                                stringify!(#q_system),
                            );
                            self.#fields.#q_system(sd).await;
                        }
                    )*
                }
            }
        } else {
            quote! {
                #[inline(always)]
                async fn #q_group(&mut self, sd: &SD) {
                    let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                    let _ = futures_used_by_typed_ecs_macro::join! {
                        #(
                            async {
                                let _sys_guard = Self::on_system_start(
                                    stringify!(#q_schedule),
                                    stringify!(#types),
                                    stringify!(#q_system),
                                );
                                self.#fields.#q_system(sd).await;
                            },
                        )*
                    };
                }
            }
        }
    } else {
        if exit_check {
            quote! {
                #[inline(always)]
                fn #q_group(&mut self, should_exit: &mut ShouldExit, sd: &SD) {
                    let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                    #(
                        {
                            let _sys_guard = Self::on_system_start(
                                stringify!(#q_schedule),
                                stringify!(#types),
                                stringify!(#q_system),
                            );
                            self.#fields.#q_system(should_exit, sd);
                        }
                    )*
                }
            }
        } else if is_mut {
            quote! {
                #[inline(always)]
                fn #q_group(&mut self, sd: &mut SD) {
                    let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                    #(
                        {
                            let _sys_guard = Self::on_system_start(
                                stringify!(#q_schedule),
                                stringify!(#types),
                                stringify!(#q_system),
                            );
                            self.#fields.#q_system(sd);
                        }
                    )*
                }
            }
        } else {
            quote! {
                #[inline(always)]
                fn #q_group(&mut self, sd: &SD) {
                    let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                    #(
                        {
                            let _sys_guard = Self::on_system_start(
                                stringify!(#q_schedule),
                                stringify!(#types),
                                stringify!(#q_system),
                            );
                            self.#fields.#q_system(sd);
                        }
                    )*
                }
            }
        }
    }
}
