extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Type)]
pub fn type_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_derive_input(&input.to_string()).unwrap();
    let ident = ast.ident;
    
    let gen = quote! {
        impl #ident {
            pub(crate) unsafe fn from_raw<'a>(ptr: LLVMTypeRef) -> &'a Self {
                ::std::mem::transmute::<LLVMTypeRef, &Self>(ptr)
            }
        }
        
        impl Deref for #ident {
            type Target = Type;

            fn deref(&self) -> &Self::Target {
                unsafe { ::std::mem::transmute::<&Self, &Self::Target>(self) }
            }
        }
    };
    gen.parse().unwrap()
}
