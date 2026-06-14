use proc_macro::TokenStream;

enum VarName {
    Name(syn::Ident),
    Names(proc_macro2::TokenStream),
}

impl quote::ToTokens for VarName {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Name(ident) => ident.to_tokens(tokens),
            Self::Names(stream) => stream.to_tokens(tokens),
        }
    }
}

struct ExprBody {
    var: VarName,
    filter: Option<syn::Expr>,
    source: syn::Expr,
}

impl syn::parse::Parse for ExprBody {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![for]>()?;
        let var = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let stream = content.parse::<proc_macro2::TokenStream>()?;
            VarName::Names(quote::quote! {
                (#stream)
            })
        } else {
            VarName::Name(input.parse::<syn::Ident>()?)
        };

        input.parse::<syn::Token![in]>()?;
        let source = input.parse::<syn::Expr>()?;

        let filter = if input.peek(syn::Token![if]) {
            input.parse::<syn::Token![if]>()?;
            Some(input.parse::<syn::Expr>()?)
        } else {
            None
        };
        Ok(Self {
            var,
            source,
            filter,
        })
    }
}

struct ListExpr {
    bodys: Vec<ExprBody>,
    value: syn::Expr,
}

impl syn::parse::Parse for ListExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<syn::Expr>()?;
        let mut bodys = vec![];
        while !input.is_empty() {
            bodys.push(input.parse::<ExprBody>()?);
        }
        Ok(Self { value, bodys })
    }
}

struct DictExpr {
    key: syn::Expr,
    value: syn::Expr,
    bodys: Vec<ExprBody>,
}

impl syn::parse::Parse for DictExpr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse::<syn::Expr>()?;
        input.parse::<syn::Token![:]>()?;
        let value = input.parse::<syn::Expr>()?;
        let mut bodys = vec![];
        while !input.is_empty() {
            bodys.push(input.parse::<ExprBody>()?);
        }
        Ok(Self { key, value, bodys })
    }
}

fn build(bodys: &[ExprBody], mut value: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    for (index, body) in bodys.iter().rev().enumerate() {
        let var = &body.var;
        let source = &body.source;
        let filter = body.filter.iter();
        let func_name = if index == 0 { "map" } else { "flat_map" };
        let func_name = syn::Ident::new(func_name, proc_macro2::Span::call_site());

        value = if index + 1 != bodys.len() {
            quote::quote! {
                #source
                    #(.filter(move |#var| #filter))*
                    .#func_name(move |#var| #value)
            }
        } else {
            quote::quote! {
                #source
                    #(.filter(|#var| #filter))*
                    .#func_name(|#var| #value)
            }
        };
    }
    value
}

fn expr_build(input: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let expr = syn::parse::<ListExpr>(input)?;
    let value = expr.value;
    let result = quote::quote! { #value };
    Ok(build(&expr.bodys, result))
}

#[proc_macro]
pub fn list(input: TokenStream) -> TokenStream {
    let result = match expr_build(input) {
        Ok(r) => r,
        Err(e) => return e.to_compile_error().into(),
    };
    quote::quote! {
        #result.collect::<Vec<_>>()
    }
    .into()
}

#[proc_macro]
pub fn iter(input: TokenStream) -> TokenStream {
    match expr_build(input) {
        Ok(r) => r,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro]
pub fn set(input: TokenStream) -> TokenStream {
    let result = match expr_build(input) {
        Ok(r) => r,
        Err(e) => return e.to_compile_error().into(),
    };
    quote::quote! {
        #result.collect::<::std::collections::HashSet<_>>()
    }
    .into()
}

#[proc_macro]
pub fn dict(input: TokenStream) -> TokenStream {
    let expr = syn::parse_macro_input!(input as DictExpr);
    let key = expr.key;
    let value = expr.value;
    let result = quote::quote! { (#key, #value) };
    let result = build(&expr.bodys, result);
    quote::quote! {
        #result.collect::<::std::collections::HashMap<_, _>>()
    }
    .into()
}
