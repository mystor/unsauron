#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate unsauron_impl;
#[doc(hidden)]
pub use unsauron_impl::*;

proc_macro_expr_decl! {
    unsauron! => expand_unsauron
}
