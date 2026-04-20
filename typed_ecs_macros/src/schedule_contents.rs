use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub(crate) fn generate_schedule(
    fields: Vec<syn::Ident>,
    types: Vec<syn::Ident>,
    schedule_name: &'static str,
    system_name: &'static str,
) -> TokenStream {
    let system_group_name = format!("{}_all", system_name);
    let is_async = system_name.starts_with("async_");
    let is_mut = system_name.contains("apply");
    let exit_check = system_name.contains("exit_check");

    let q_group = format_ident!("{}", system_group_name);
    let q_schedule = format_ident!("{}", schedule_name);
    let q_system = format_ident!("{}", system_name);

    if is_async != schedule_name.starts_with("Async") {
        panic!(
            "Mismatch between schedule ({}) and system name ({})",
            schedule_name, system_name
        );
    }

    if is_async {
        quote! {
            #[inline(always)]
            async fn #q_group(&mut self, sd: &SD) {
                let _sched_guard = Self::on_schedule_start(stringify!(#q_schedule));
                let _ = ::typed_ecs::futures::join! {
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
    } else {
        if exit_check {
            quote! {
                #[inline(always)]
                fn #q_group<S: ::typed_ecs::should_exit::ShouldExit>(&mut self, should_exit: &mut S, sd: &SD) {
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
            if crate::IS_PARALLEL {
                quote! {
                    #[inline(always)]
                    fn #q_group(&mut self, sd: &SD) {
                        rayon::scope(|s| {
                            #(
                                s.spawn(|_| {
                                    let _sys_guard = Self::on_system_start(
                                        stringify!(#q_schedule),
                                        stringify!(#types),
                                        stringify!(#q_system),
                                    );
                                    self.#fields.#q_system(sd);
                                });
                            )*
                        });
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
}
