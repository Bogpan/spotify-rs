use proc_macro::TokenStream;
use quote::quote;
use syn::{Field, ItemStruct, LitStr, parse_macro_input};

#[proc_macro_attribute]
pub fn docs(attr: TokenStream, item: TokenStream) -> TokenStream {
    docs_impl(attr, item)
}

// Macro to return a Some when formatting strings.
macro_rules! format_some {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);

        let res = quote! {
            #[doc = #res]
        };

        Some(res)
    }}
}

pub(crate) fn docs_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemStruct = syn::parse(item.clone()).unwrap();
    let ItemStruct {
        vis,
        ident,
        fields,
        attrs,
        ..
    } = ast;

    // Set the item name to the struct name
    let mut item_name = ident.to_string().to_lowercase();

    // If there's an argument "name", set the item_name to it instead,
    // otherwise return an error.
    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            let value: LitStr = meta.value()?.parse()?;
            item_name = value.value();

            Ok(())
        } else {
            Err(meta.error("unsupported property"))
        }
    });

    parse_macro_input!(attr with parser);

    let (docs, fields): (Vec<_>, Vec<Field>) = fields
        .into_iter()
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap().to_string();
            let struct_name = &item_name;

            // Generate the comment based on the field name,
            // including the struct name in the comment as well.
            let doc = field_documentation(&field_name, struct_name);

            (doc, f)
        })
        .collect();

    // Reconstruct the struct with the documentation applied.
    quote! {
        #(#attrs)*
        #vis struct #ident {
            #(
                #docs
                #fields

            ),*
        }

    }
    .into()
}

fn field_documentation(field_name: &str, name: &str) -> Option<proc_macro2::TokenStream> {
    match field_name {
        "available_markets" => format_some!(
            "The [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) codes for the markets in which the {name} is available."
        ),

        "external_urls" => format_some!("Known external URLs for the {name}."),

        "href" => format_some!(
            "A link to the Spotify Web API endpoint providing full details of the {name}."
        ),

        "id" => format_some!(
            "The [Spotify ID](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the {name}."
        ),

        "r#type" | "type" => format_some!("The object type. Allowed values: `{name}`."),

        "uri" => format_some!(
            "The [Spotify URI](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the {name}."
        ),

        "copyrights" => format_some!("The copyright statements of the {name}."),

        "external_ids" => format_some!("Known external IDs for the {name}."),

        "images" => {
            format_some!("The cover art for the {name} in various sizes, with the widest first.")
        }

        "name" => format_some!("The name of the {name}."),

        "release_date" => format_some!("The date the {name} was first released."),

        "release_date_precision" => {
            format_some!("The precision with which the `release_date` field is known.")
        }

        "description" => format_some!("A text description of the {name}."),

        "html_description" => {
            format_some!("A description of the {name} that may contain HTML tags.")
        }

        "restrictions" => {
            format_some!("Included in the response when a content restriction is applied.")
        }

        "explicit" => format_some!(
            "Whether or not the {name} contains explicit content.\n\nNote: `false` can also mean *unknown*."
        ),

        "languages" => format_some!(
            "A list of [ISO 639](https://en.wikipedia.org/wiki/ISO_639) codes for the languages spoken in the {name}."
        ),

        "media_type" => format_some!("The type of the media of the {name}."),

        "publisher" => format_some!("The publisher of the {name}."),

        "duration_ms" => format_some!("The duration of the {name} in miliseconds."),

        "is_playable" => {
            format_some!("Indicates whether or not the {name} is playable in the current market.")
        }

        "resume_point" => format_some!(
            "The user's latest position in the {name}.\n\nNote: this field is only available if the user is authorised with the `user-read-playback-position` scope."
        ),

        _ => None,
    }
}
