extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn timed(attrs: TokenStream, item: TokenStream) -> TokenStream {

    if let Ok(mut fun) = parse::<ItemFn>(item.clone()) {
        let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &fun.block.stmts);
        fun.block.stmts = new_stmts;
        return quote!(#fun).into();
    }

    if let Ok(mut fun) = parse::<TraitItemMethod>(item.clone()) {
        if let Some(block) = fun.default.as_mut() {
            let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &block.stmts);
            block.stmts = new_stmts;
            return quote!(#fun).into();
        }        
    }

    if let Ok(mut fun) = parse::<ImplItemMethod>(item) {
        let new_stmts = rewrite_stmts(fun.sig.ident.to_string(), &fun.block.stmts);
        fun.block.stmts = new_stmts;
        return quote!(#fun).into();
    }

    panic!("`funtime::timed` only works on functions")
}

fn rewrite_stmts(name: String, stmts: &[Stmt]) -> Vec<Stmt> {

    let setup: Block = parse_quote! {{
        struct FuntimeTimer(std::time::Instant, String);

        impl Drop for FuntimeTimer {
            fn drop(&mut self) {
                println!("Took {:?}: `{}` complete", self.0.elapsed(), &self.1);
            }
        }

        impl FuntimeTimer {
            fn print_elapsed(&self, short: &str) {
                println!("Took {:?}: `{}`", self.0.elapsed(), short);
            }
        }

        let funtime_timer = FuntimeTimer(std::time::Instant::now(), String::from(#name));

    }};

    let mut new_stmts = setup.stmts;

    for stmt in stmts {
        new_stmts.push(stmt.clone());
        
        if let Stmt::Expr(..) = stmt {
            continue;
        }
        
        let mut short = format!("{}", quote::ToTokens::to_token_stream(stmt)).chars().collect::<Vec<_>>();

        let short = if short.len() > 40 {
            let mut short = short[..37].into_iter().collect::<String>();
            short.push_str("...");
            short
        } else {
            short.into_iter().collect::<String>()
        };

        new_stmts.push(parse_quote!(funtime_timer.print_elapsed(#short);));
    }

    new_stmts
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
