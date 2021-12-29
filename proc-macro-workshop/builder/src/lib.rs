use proc_macro::TokenStream;
use syn::{self, spanned::Spanned};  // Spanned：A trait that can provide the Span of the complete contents of a syntax tree nod
use quote::{quote};

#[proc_macro_derive(Builder,attributes(builder))]// 注意，这里和第一篇文章里的 #[proc_macro_attribute]不同
pub fn derive(input: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

type StructFields = syn::punctuated::Punctuated<syn::Field,syn::Token![,]>;

fn get_fields_from_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct{
            fields: syn::Fields::Named(syn::FieldsNamed{
            ref named ,..}),
            ..
        }) = st.data {
        return Ok(named)
    };
    Err(syn::Error::new_spanned(st,"Must Define On Struct".to_string()))
}

fn generate_builder_struct_fields_def(
    fields: &StructFields,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // 第八关修改，从这里又可以看出，对于复杂的过程宏，采用迭代器比较麻烦，返回一个错误要费一些周折
    // 这里修改了map中闭包的返回值，使其返回一个syn::Result<T>
    let types: syn::Result<Vec<proc_macro2::TokenStream>> = fields
        .iter()
        .map(|f| {
            if let Some(inner_ty) = get_generic_inner_type(&f.ty,"Option") {
                Ok(quote!(std::option::Option<#inner_ty>))
            } else if get_user_specified_ident_for_vec(f)?.is_some() {
                let origin_ty = &f.ty;
                Ok(quote!(#origin_ty))

            } else {
                let origin_ty = &f.ty;
                Ok(quote!(std::option::Option<#origin_ty>))
            }
        })
        .collect();

    let types = types?;
    let token_stream = quote! {
        #(#idents: #types),*
    };
    Ok(token_stream)
}

fn generate_builder_struct_factory_init_clauses(fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>>{
    // 第八关修改，从闭包中返回错误信息
    let init_clauses: syn::Result<Vec<proc_macro2::TokenStream>> = fields.iter().map(|f| {
        let ident = &f.ident;
        if get_user_specified_ident_for_vec(f)?.is_some() {
            Ok(quote!{
                #ident: std::vec::Vec::new()
            })
        } else {
            Ok(quote!{
                #ident: std::option::Option::None
            })
        }
    }).collect();

    Ok(init_clauses?)
}

fn generate_setter_functions(fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_tokenstream = proc_macro2::TokenStream::new();

    // 第七关修改，这里之前用了zip串联了两个迭代器，现在需要拿到对应的原始field，所以又加了一层`enumerate()`迭代器
    // 这里写成 for idx in 0..fields.len() {let ident = &fields[idx].ident; let type_ = &fields[idx].ty;...} 这种写法或许更优雅一些
    for (idx,(ident, type_)) in idents.iter().zip(types.iter()).enumerate() {
        let mut tokenstream_piece;
        if let Some(inner_ty) = get_generic_inner_type(type_,"Option") {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #inner_ty) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };

            // 下面这个分支是第七关加入的
        } else if let Some(ref user_specified_ident) = get_user_specified_ident_for_vec(&fields[idx]).unwrap() {
            let inner_ty = get_generic_inner_type(type_,"Vec").ok_or(syn::Error::new(fields[idx].span(),"each field must be specified with Vec field"))?;
            tokenstream_piece = quote! {
                fn #user_specified_ident(&mut self, #user_specified_ident: #inner_ty) -> &mut Self {
                    self.#ident.push(#user_specified_ident);
                    self
                }
            };
            // 如果用户指定的setter名字和原始字段的名字不一样，那么产生另一个setter，这个setter是一次性传入一个列表的
            if user_specified_ident != ident.as_ref().unwrap() {
                tokenstream_piece.extend(
                    quote! {
                        fn #ident(&mut self, #ident: #type_) -> &mut Self {
                            self.#ident = #ident.clone();
                            self
                        }
                    }
                );
            }
        } else {
            tokenstream_piece = quote! {
                fn #ident(&mut self, #ident: #type_) -> &mut Self {
                    self.#ident = std::option::Option::Some(#ident);
                    self
                }
            };
        }
        final_tokenstream.extend(tokenstream_piece);
    }

    Ok(final_tokenstream)
}

// 第六关
fn get_generic_inner_type<'a>(ty: &'a syn::Type, outer_ident_name: &str) -> Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {
        // 这里我们取segments的最后一节来判断是不是`T<U>`，这样如果用户写的是`foo:bar::T<U>`我们也能识别出最后的`T<U>`
        if let Some(seg) = path.segments.last() {
            if seg.ident == outer_ident_name {
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                                              ref args,
                                                              ..
                                                          }) = seg.arguments
                {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

fn generate_build_function(
    fields: &StructFields,
    origin_struct_ident: &syn::Ident,
) -> syn::Result<proc_macro2::TokenStream> {
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut checker_code_pieces = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 第七关修改，只对不是`Option`类型且没有指定each属性的字段生成校验逻辑
        if get_generic_inner_type(&types[idx],"Option").is_none() && get_user_specified_ident_for_vec(&fields[idx]).unwrap().is_none() {
            checker_code_pieces.push(quote! {
                if self.#ident.is_none() {
                    let err = format!("{} field missing", stringify!(#ident));
                    return std::result::Result::Err(err.into())
                }
            });
        }
    }

    let mut fill_result_clauses = Vec::new();
    for idx in 0..idents.len() {
        let ident = idents[idx];
        // 第七关，这里需要判断是否有each属性。第一个分支是本关加入的。注意这里几个分支的先后判断顺序
        // 看我写在这里的代码可能没什么感觉，但如果是自己写的话，这几个分支的判断先后顺序是很重要的，否则可能生成出有问题的代码
        // 这里主要的问题是梳理清楚【是否有each属性】和【是否为Option类型】这两个条件的覆盖范围
        if get_user_specified_ident_for_vec(&fields[idx]).unwrap().is_some() {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone()
            });
        } else if get_generic_inner_type(&types[idx],"Option").is_none() {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone().unwrap()
            });
        } else {
            fill_result_clauses.push(quote! {
                #ident: self.#ident.clone()
            });
        }
    }

    let token_stream = quote! {
        pub fn build(&mut self) -> std::result::Result<#origin_struct_ident, std::boxed::Box<dyn std::error::Error>> {
            #(#checker_code_pieces)*
            let ret = #origin_struct_ident{
                #(#fill_result_clauses),*
            };
            std::result::Result::Ok(ret)
        }
    };
    Ok(token_stream)
}

fn get_user_specified_ident_for_vec(field: &syn::Field) -> syn::Result<Option<syn::Ident>> {
    for attr in &field.attrs {
        if let Ok(syn::Meta::List(syn::MetaList {
                                      ref path,
                                      ref nested,
                                      ..
                                  })) = attr.parse_meta()
        {
            if let Some(p) = path.segments.first() {
                if p.ident == "builder" {
                    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(kv))) = nested.first() {
                        if kv.path.is_ident("each") {
                            if let syn::Lit::Str(ref ident_str) = kv.lit {
                                return Ok(Some(syn::Ident::new(
                                    ident_str.value().as_str(),
                                    attr.span(),
                                )));
                            }
                        } else {
                            // 第八关加入，注意这里new_spanned函数的参数，我们需要在语法树中找到一个合适的节点来获取它的span，如果这个语法树节点找的不对，产生出的错误信息就会不一样
                            if let Ok(syn::Meta::List(ref list)) = attr.parse_meta() {
                                return Err(syn::Error::new_spanned(list, r#"expected `builder(each = "...")`"#))
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(None)
}

fn do_expand(st: &syn::DeriveInput)  -> syn::Result<proc_macro2::TokenStream> {
    // eprintln!("{:#?}", st.data);
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builer",struct_name_literal);
    let builder_name_ident = syn::Ident::new(&builder_name_literal,st.span());

    let struct_ident = &st.ident;
    let fields = get_fields_from_derive_input(st)?;

    let builder_struct_field_def  = generate_builder_struct_fields_def(fields)?;
    let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(fields)?;

    // 下面这一行是第三关新加的
    let setter_functions = generate_setter_functions(fields)?;

    // 下面这一行是第四关新加的
    let generated_builder_functions = generate_build_function(fields,struct_ident)?;

    let ret = quote! {
        pub struct #builder_name_ident {  // 创建结构体
            #builder_struct_field_def
        }

        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {  // 实现结构体方法
                #builder_name_ident {
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }

       // 下面这三行是第三关新加的
        impl #builder_name_ident {
            #setter_functions

            // 下面这1行是第四关新加的
            #generated_builder_functions
        }
    };

    Ok(ret)
 }