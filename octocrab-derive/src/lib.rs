extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error, ResultExt};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    visit_mut::VisitMut,
    AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Field, Fields, GenericArgument,
    Ident, Lifetime, Lit, LitStr, Meta, MetaList, MetaNameValue, NestedMeta, Path, PathArguments,
    Token, Type, TypePath, TypeReference,
};

/// Derive for the builder pattern that automatically creates setters for the annotated struct.
///
/// Fields that are an `Option<T>` are considered part of the builder and a setter function is
/// generated. All other _mandatory_ fields are added to a crate local constructor.
///
/// Furthermore, depending on the type of a field the parameter of a setter is changed to improve
/// flexibility and allow many possible inputs that can convert into the type.
///
/// - If the field type is a `&'a str`, the setter type becomes `&'a (impl AsRef<str> + ?Sized)`.
/// - If the field type is a `&'a [T]`, the setter type becomes `&'a (impl AsRef<[T]> + ?Sized)`.
/// - Otherwise the field type `T` becomes `Into<T>` with all lifetimes included if any.
///
/// # Options
///
/// In addition to this default behavior the generated code can be customized by annotating the
/// individual fields of a struct with the `#[builder(...)]` attribute.
///
/// - `#[builder(skip)]` will exclude a field of type `Option<T>` that would usually cause a setter
///   to be generated. Instead the field will be added to the crate local constructor and the
///   argument type won't be modified in any way.
/// - `#[builder(rename = "new_name")]` will change the setter function's name to the given value and
///   the name of its parameter as well. Otherwise the setter name is exactly the same as the field
///   name.
///
/// All options can be combined freely and either defined in separate `#[builder(...)]` attributes,
/// combined into a single one, separated by commas, or both. Therefore, defining two attributes
/// like `#[builder(skip)]` and `#[builder(rename = "value")]` is equivalent to a single attribute
/// like `#[builder(skip, rename = "value")]`.
///
/// If an option is defined multiple times, only the last occurrence is taken into account. Defining
/// first `#[builder(rename = "a")]` and then `#[builder(rename = "b")]` on the same field would
/// result in `"b"` as the effective rename.
///
/// # Documentation
///
/// To add documentation to the generated setters, simply add the docs to the respective fields
/// instead. The macro will copy over any docs from the field to the setter.
///
/// As fields are usually private, the documentation can be very specific and focused to the setter
/// without exposing an _unfitting_ documentation for the field itself.
///
/// # Supported structures
///
/// As of now, the only support struct form is a named struct. Annotating other kinds like a unit
/// struct `struct Sample` or a tuple struct `struct Sample(A)` will result in a compile error.
/// Other elements like enums aren't supported neither.
///
/// # Example
///
/// Given the following struct:
///
/// ```
/// # enum Event {}
/// #[derive(octocrab_derive::Builder)]
/// struct Sample<'id, 'filter, 'events> {
///     url: String,
///     #[builder(skip)]
///     id: Option<&'id str>,
///     /// Additional filter to apply.
///     #[builder(rename = "filter")]
///     name_filter: Option<&'filter str>,
///     /// Events that should be searched for.
///     events: Option<&'events [Event]>,
///     /// The page to retrieve.
///     page: Option<u8>,
/// }
/// ```
///
/// The generated implementation would be:
///
/// ```
/// # enum Event {}
/// # struct Sample<'id, 'filter, 'events> {
/// #     url: String,
/// #     id: Option<&'id str>,
/// #     name_filter: Option<&'filter str>,
/// #     events: Option<&'events [Event]>,
/// #     page: Option<u8>,
/// # }
/// impl<'id, 'filter, 'events> Sample<'id, 'filter, 'events> {
///     pub(crate) fn new(url: String, id: Option<&'id str>) -> Self {
///         Self {
///             url,
///             id,
///             name_filter: None,
///             events: None,
///             page: None,
///         }
///     }
///
///     /// Additional filter to apply.
///     pub fn filter(mut self, filter: &'filter (impl AsRef<str> + ?Sized)) -> Self {
///         self.name_filter = Some(filter.as_ref());
///         self
///     }
///
///     /// Events that should be searched for.
///     pub fn events(mut self, events: &'events (impl AsRef<[Event]> + ?Sized)) -> Self {
///         self.events = Some(events.as_ref());
///         self
///     }
///
///     /// The page to retrieve.
///     pub fn page(mut self, page: impl Into<u8>) -> Self {
///         self.page = Some(page.into());
///         self
///     }
/// }
/// ```
#[proc_macro_error]
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;
    let generics = &input.generics;

    let data = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| FieldInfo {
                    attributes: get_attrs(&field),
                    docs: get_docs(&field),
                    name: field.ident.as_ref().unwrap(),
                    ty: &field.ty,
                    inner: get_option(&field),
                })
                .collect::<Vec<_>>(),
            _ => abort!(data.fields, "only named fields supported"),
        },
        _ => abort!(input, "only structs supported"),
    };

    let args = data.iter().filter_map(|f| {
        (f.attributes.skip || f.inner.is_none()).then(|| {
            let name = f.name;
            let ty = f.ty;
            quote! { #name: #ty }
        })
    });

    let fields = data.iter().map(|f| {
        let name = f.name;
        if f.attributes.skip || f.inner.is_none() {
            quote! { #name }
        } else {
            quote! { #name: None }
        }
    });

    let fns = data.iter().filter(|f| !f.attributes.skip).filter_map(|f| {
        f.inner.as_ref().map(|Inner { ty, lifetime }| {
            let docs = &f.docs;
            let name = f.name;
            let rename = f.attributes.rename.as_ref().unwrap_or(name);

            let lifetime = lifetime.map(|lt| quote! { &#lt }).unwrap_or_default();
            let ref_ty = is_str_ref(ty).or_else(|| is_slice_ref(ty));
            let ty2 = ref_ty
                .map(|ref_ty| quote! { (impl std::convert::AsRef<#ref_ty> + ?std::marker::Sized) })
                .unwrap_or_else(|| quote! { impl std::convert::Into<#ty> });
            let call = ref_ty
                .is_some()
                .then(|| quote! { as_ref() })
                .unwrap_or_else(|| quote! { into() });

            quote! {
                #(#docs)*
                pub fn #rename(mut self, #rename: #lifetime #ty2) -> Self {
                    self.#name = std::option::Option::Some(#rename.#call);
                    self
                }
            }
        })
    });

    let expanded = quote! {
        impl #generics #ident #generics {
            pub(crate) fn new(#(#args),*) -> Self {
                Self {
                    #(#fields),*
                }
            }

            #(#fns)*
        }
    };

    TokenStream::from(expanded)
}

struct FieldInfo<'a> {
    attributes: BuilderAttributes,
    docs: Vec<&'a Attribute>,
    name: &'a Ident,
    ty: &'a Type,
    inner: Option<Inner<'a>>,
}

/// Representation of all the options that can be used to customize the generated code on a
/// per-field level.
#[derive(Default)]
struct BuilderAttributes {
    /// Exclude an option field like `Option<T>` from the generation. It will be added to the
    /// crate local constructor instead and not be exposed publicly.
    skip: bool,
    /// Give the generated setter a different name than the field.
    rename: Option<Ident>,
}

impl BuilderAttributes {
    /// Combine this attributes with another set. It helps to combine all attributes into a single
    /// instance so that:
    ///
    /// ```no_run
    /// #[derive(octocrab_derive::Builder)]
    /// struct Sample {
    ///     #[builder(skip)]
    ///     #[builder(rename = "foo")]
    ///     field: Option<u8>
    /// }
    /// ```
    ///
    /// Will effectively become the same as:
    ///
    /// ```no_run
    /// #[derive(octocrab_derive::Builder)]
    /// struct Sample {
    ///     #[builder(skip, rename = "foo")]
    ///     field: Option<u8>
    /// }
    /// ```
    ///
    /// Additionally, any duplicate options will be combined so that only the last occurrence
    /// counts. For example having `#[builder(rename = "a")]` and `#[builder(rename = "b")]`, the
    /// effective rename would be `"b"`.
    fn merge(self, rhs: Self) -> Self {
        Self {
            skip: rhs.skip || self.skip,
            rename: rhs.rename.or(self.rename),
        }
    }
}

impl Parse for BuilderAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut skip = false;
        let mut rename = None;

        loop {
            let ident = input.parse::<Ident>()?;
            match ident.to_string().as_ref() {
                "skip" => skip = true,
                "rename" => {
                    input.parse::<Token![=]>()?;
                    let name = input.parse::<LitStr>()?;
                    rename = Some(name.parse()?);
                }
                _ => abort!(ident, "invalid option"),
            }

            if input.is_empty() {
                break;
            }

            input.parse::<Token![,]>()?;
        }

        Ok(Self { skip, rename })
    }
}

struct Inner<'a> {
    ty: &'a Type,
    lifetime: Option<&'a Lifetime>,
}

/// Extract builder specific attributes from a field.
///
/// They're in the form of `#[builder(skip, rename = "...")]` and each option is optional and can
/// be freely combined.
fn get_attrs(field: &Field) -> BuilderAttributes {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("builder"))
        .fold(BuilderAttributes::default(), |ba, attr| {
            ba.merge(attr.parse_args().unwrap_or_abort())
        })
}

/// Get all the doc attributes from a field.
fn get_docs(field: &Field) -> Vec<&Attribute> {
    field
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .collect()
}

/// Extract the inner type `T` of a field if it's an `Option<T>`.
fn get_option(field: &Field) -> Option<Inner<'_>> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = &field.ty
    {
        segments
            .last()
            .filter(|seg| seg.ident == "Option")
            .and_then(|seg| match &seg.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
                    (args.len() == 1)
                        .then(|| args.first().unwrap())
                        .and_then(|arg| match arg {
                            GenericArgument::Type(ty) => Some(Inner {
                                ty,
                                lifetime: get_lifetime(ty),
                            }),
                            _ => None,
                        })
                }
                _ => None,
            })
    } else {
        None
    }
}

/// Get the lifetime of a type, if it has one.
fn get_lifetime(ty: &Type) -> Option<&Lifetime> {
    if let Type::Reference(TypeReference { lifetime, .. }) = ty {
        lifetime.as_ref()
    } else {
        None
    }
}

/// Check whether the given type is some kind of `&'a str`.
fn is_str_ref(ty: &Type) -> Option<&Type> {
    if let Type::Reference(TypeReference {
        lifetime: Some(_),
        elem,
        ..
    }) = ty
    {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = elem.as_ref()
        {
            if segments.len() == 1
                && segments
                    .first()
                    .map(|seg| seg.ident == "str" && matches!(seg.arguments, PathArguments::None))
                    .unwrap_or_default()
            {
                return Some(elem);
            }
        }
    }

    None
}

/// Check whether the given type is a slice like `&'a [T]`.
fn is_slice_ref(ty: &Type) -> Option<&Type> {
    if let Type::Reference(TypeReference {
        lifetime: Some(_),
        elem,
        ..
    }) = ty
    {
        if let Type::Slice(_) = elem.as_ref() {
            return Some(elem);
        }
    }

    None
}

/// A shorthand helper macro that reduces boilerplate by automatically adding a
/// `#[serde(skip_serializing_if = "Option::is_none")]` attribute to all fields of type `Option<T>`
/// in a struct.
///
/// # Example
///
/// Given a struct with more than one optional field:
///
/// ```no_run
/// #[derive(serde::Serialize)]
/// struct Sample {
///     id: u64,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     name: Option<String>,
///     #[serde(skip_serializing_if = "Option::is_none")]
///     age: Option<u8>
/// }
/// ```
///
/// Boilerplate can be reduced with this macro:
///
/// ```no_run
/// #[octocrab_derive::serde_skip_none]
/// #[derive(serde::Serialize)]
/// struct Sample {
///     id: u64,
///     name: Option<String>,
///     age: Option<u8>
/// }
/// ```
///
/// After the macro ran, the struct will look exactly like in the initial sample but didn't have
/// to be annotated with attributes manually.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn serde_skip_none(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = syn::parse_macro_input!(item as syn::ItemStruct);

    AttrVisitor.visit_item_struct_mut(&mut item);

    TokenStream::from(item.into_token_stream())
}

/// A visitor that checks whether the `#[serde(skip_serializing_if = "Option::is_none")]` is already
/// present on any fields that are of type `Option<T>` and adds it if missing.
struct AttrVisitor;

impl VisitMut for AttrVisitor {
    fn visit_field_mut(&mut self, i: &mut Field) {
        if get_option(i).is_some() && !i.attrs.iter().any(is_serde_skip_attr) {
            i.attrs.push(parse_quote! {
                #[serde(skip_serializing_if = "Option::is_none")]
            })
        }
    }
}

/// Determine whether the given attribute is in the form
/// `#[serde(skip_serializing_if = "Option::is_none")]`.
fn is_serde_skip_attr(attr: &Attribute) -> bool {
    if let Ok(Meta::List(MetaList { path, nested, .. })) = attr.parse_meta() {
        path.is_ident("serde")
            && nested.len() == 1
            && if let Some(NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(lit_str),
                ..
            }))) = nested.first()
            {
                path.is_ident("skip_serializing_if") && lit_str.value() == "Option::is_none"
            } else {
                false
            }
    } else {
        false
    }
}
