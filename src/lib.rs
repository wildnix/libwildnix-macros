use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let attrs = input.attrs;
    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;

    if sig.ident != "main" {
        return quote! {
            compile_error!("#[wildnix::main] must be used on a function named main");
        }
        .into();
    }

    let expanded = quote! {
        #(#attrs)*
        #vis #sig #block

        #[unsafe(no_mangle)]
        pub extern "C" fn _start() -> ! {
            main();
            wildnix::exit(0);
        }
    };

    expanded.into()
}
