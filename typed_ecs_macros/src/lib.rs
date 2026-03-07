use ::proc_macro::TokenStream;
use ::quote::quote;

use ::syn::{punctuated::Punctuated, *};
use quote::format_ident;

/// Please see the [`plugin_collection`](https://github.com/heydocode/typed_ecs/blob/main/examples/plugin_collection.rs) for more details on the usage of this macro.
#[proc_macro]
pub fn generate_collection(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
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
            panic!(
                "generate_collection! only supports simple path types\n\nReceived tokens:\n{}",
                input_clone
            );
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
                fn pre_startup_ref_sd_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("PreStartupRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PreStartupRef", stringify!(#types), "pre_startup_ref_sd");
                            self.#idents.pre_startup_ref_sd(sd);
                        }
                    )*
                    // note: _sched_guard drops here → schedule span ends
                }

                #[inline(always)]
                fn pre_startup_mutref_sd_all(&self, sd: &mut SD) {
                    let _sched_guard = self.on_schedule_start("PreStartupMutRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PreStartupMutRef", stringify!(#types), "pre_startup_mutref_sd");
                            self.#idents.pre_startup_mutref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn startup_ref_sd_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("StartupRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("StartupRef", stringify!(#types), "startup_ref_sd");
                            self.#idents.startup_ref_sd(sd);
                        }
                    )*
                    // note: _sched_guard drops here → schedule span ends
                }

                #[inline(always)]
                fn startup_mutref_sd_all(&self, sd: &mut SD) {
                    let _sched_guard = self.on_schedule_start("StartupMutRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("StartupMutRef", stringify!(#types), "startup_mutref_sd");
                            self.#idents.startup_mutref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn post_startup_ref_sd_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("PostStartupRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PostStartupRef", stringify!(#types), "post_startup_ref_sd");
                            self.#idents.post_startup_ref_sd(sd);
                        }
                    )*
                    // note: _sched_guard drops here → schedule span ends
                }

                #[inline(always)]
                fn post_startup_mutref_sd_all(&self, sd: &mut SD) {
                    let _sched_guard = self.on_schedule_start("PostStartupMutRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PostStartupMutRef", stringify!(#types), "post_startup_mutref_sd");
                            self.#idents.post_startup_mutref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn pre_update_ref_sd_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("PreUpdateRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PreUpdateRef", stringify!(#types), "pre_update_ref_sd");
                            self.#idents.pre_update_ref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn pre_update_mutref_sd_all(&self, sd: &mut SD) {
                    let _sched_guard = self.on_schedule_start("PreUpdateMutRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PreUpdateMutRef", stringify!(#types), "pre_update_mutref_sd");
                            self.#idents.pre_update_mutref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn update_ref_sd_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("UpdateRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("UpdateRef", stringify!(#types), "update_ref_sd");
                            self.#idents.update_ref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn update_mutref_sd_all(&self, sd: &mut SD) {
                    let _sched_guard = self.on_schedule_start("UpdateMutRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("UpdateMutRef", stringify!(#types), "update_mutref_sd");
                            self.#idents.update_mutref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn post_update_ref_sd_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("PostUpdateRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PostUpdateRef", stringify!(#types), "post_update_ref_sd");
                            self.#idents.post_update_ref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn post_update_mutref_sd_all(&self, sd: &mut SD) {
                    let _sched_guard = self.on_schedule_start("PostUpdateMutRef");

                    #(
                        {
                            let _sys_guard = self.on_system_start("PostUpdateMutRef", stringify!(#types), "post_update_mutref_sd");
                            self.#idents.post_update_mutref_sd(sd);
                        }
                    )*
                }

                #[inline(always)]
                fn exit_check_all(&self, should_exit: &mut ShouldExit, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("ExitCheck");

                    #(
                        {
                            let _sys_guard = self.on_system_start("ExitCheck", stringify!(#types), "exit_check");
                            self.#idents.exit_check(should_exit, sd);
                        }
                    )*
                }

                #[inline(always)]
                fn on_exit_all(&self, sd: &SD) {
                    let _sched_guard = self.on_schedule_start("OnExit");

                    #(
                        {
                            let _sys_guard = self.on_system_start("OnExit", stringify!(#types), "on_exit");
                            self.#idents.on_exit(sd);
                        }
                    )*
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
