extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn timed(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(mut fun) = parse::<ItemFn>(item.clone()) {
        let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &mut fun.block.stmts);
        fun.block.stmts = new_stmts;
        return quote!(#fun).into();
    }

    if let Ok(mut fun) = parse::<TraitItemMethod>(item.clone()) {
        if let Some(block) = fun.default.as_mut() {
            let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &mut block.stmts);
            block.stmts = new_stmts;
            return quote!(#fun).into();
        }
    }

    if let Ok(mut fun) = parse::<ImplItemMethod>(item) {
        let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &mut fun.block.stmts);
        fun.block.stmts = new_stmts;
        return quote!(#fun).into();
    }

    panic!("`funtime::timed` only works on functions")
}

fn rewrite_stmts(name: String, stmts: &mut Vec<Stmt>) -> Vec<Stmt> {
    let setup: Block = parse_quote! {{
        struct FuntimeTimer {
            start: std::time::Instant,
            name: &'static str,
            buffer: String,
            prev_mark: Option<std::time::Duration>,
        }


        impl Drop for FuntimeTimer {
            fn drop(&mut self) {
                use std::fmt::Write;
                writeln!(&mut self.buffer, "funtime end: `{}` took {:?}", self.name, self.start.elapsed()).unwrap();
                print!("{}", &self.buffer);
            }
        }

        impl FuntimeTimer {
            fn new(name: &'static str) -> Self {
                use std::fmt::Write;
                let mut buffer = String::new();
                writeln!(&mut buffer, "funtime start: `{}`", name).unwrap();
                FuntimeTimer {
                    start: std::time::Instant::now(),
                    name,
                    buffer,
                    prev_mark: None,
                }
            }

            fn mark_elapsed(&mut self, short: &str) {
                use std::fmt::Write;
                let mut elapsed = self.start.elapsed();
                if let Some(prev) = self.prev_mark.replace(elapsed) {
                    elapsed = elapsed - prev;
                }
                writeln!(&mut self.buffer, "  took {:?}: `{}`", elapsed, short).unwrap();
            }
        }

        let mut funtime_timer = FuntimeTimer::new(#name);

    }};

    let mut new_stmts = setup.stmts;

    let last = stmts.pop();

    for stmt in stmts.drain(..) {
        let short =
            format!("{}", quote::ToTokens::to_token_stream(&stmt)).chars().collect::<Vec<_>>();

        let short = if short.len() > 40 {
            let mut short = short[..37].into_iter().collect::<String>();
            short.push_str("...");
            short
        } else {
            short.into_iter().collect::<String>()
        };

        let next_stmt = parse_quote!(funtime_timer.mark_elapsed(#short););

        new_stmts.push(stmt);
        new_stmts.push(next_stmt);
    }

    if let Some(stmt) = last {
        new_stmts.push(stmt);
    }

    new_stmts
}
