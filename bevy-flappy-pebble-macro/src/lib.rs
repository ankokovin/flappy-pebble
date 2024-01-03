use proc_macro::*;
use indoc::indoc;
use quote::quote;
use syn::{DeriveInput, Meta, Token, parse_macro_input, Ident};
use syn::spanned::Spanned;
use syn::punctuated::Punctuated;


#[proc_macro_derive(
ChangeStateButton, attributes(keyboard, target_state))]
pub fn button(input: TokenStream) -> TokenStream {
    let decl = parse_macro_input!(input as DeriveInput);

    let mut key_codes = None;
    let mut target_state = None;

    let mut error = None;

    for attr in decl.attrs.iter() {
        match &attr.meta {
            Meta::List(list) if list.path.is_ident("keyboard") => {
                let result = list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated);
                if result.is_err() {
                    error = Some(syn::Error::new(
                        list.span(),
                        indoc! {r#"
                                The `keyboard` attribute expects idents to be comma separated

                                = help: use `#[keyboard(Space, Return)]`
                            "#}
                    ));
                    continue;
                }

                key_codes = Some(result.unwrap());
            },
            Meta::List(list) if list.path.is_ident("target_state") => {
                let result: syn::Result<Ident> = list.parse_args();
                if result.is_err() {
                    eprintln!("{}", result.unwrap_err());
                    error = Some(syn::Error::new(
                        list.span(),
                        indoc! {r#"
                                The `target_state` attribute expects a single ident

                                = help: use `#[target_state(Playing)]`
                            "#}
                    ));
                    continue;
                }

                target_state = Some(result.unwrap());
            },
            _=> {}
        }
    }

    if error.is_none() && target_state.is_none() {
        error = Some(syn::Error::new(decl.span(), "Expected `target_state` attribute"));
    }

    if let Some(error) = error {
        return syn::Error::into_compile_error(error).into();
    }

    let struct_name = decl.ident;
    let name = struct_name.to_string();
    let key_codes = key_codes.unwrap_or_default();
    let key_codes: Vec<_> = key_codes.iter().collect();


    let expanded = quote! {
        impl ChangeStateButton for #struct_name {
            fn name(&self) -> String {
                #name.to_string()
            }

            fn should_change_state_keyboard(input: Res<Input<KeyCode>>) -> bool {
                input.any_just_pressed(vec![#(KeyCode::#key_codes,)*])
            }

            fn target_state() -> GameState {
                GameState::#target_state
            }
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod test {
    use trybuild;

    #[test]
    fn test() {
        let t = trybuild::TestCases::new();
        t.pass("tests/test.rs");
    }
}