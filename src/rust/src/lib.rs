use extendr_api::prelude::*;
use url::Url;

/// Parse url
/// @param url character vector of urls
/// @return a data.frame consisting of the columns scheme, host, port, path, query and fragment.
/// @export
#[extendr]
fn url_parse(url: Strings) -> Robj {
    let all_parsed_url = url
        .into_iter()
        .map(|urli| {
            if urli.is_na() {
                ParseUrl::default()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => ParseUrl::from(u),
                    Err(_) => ParseUrl::default(),
                }
            }
        })
        .collect::<Vec<ParseUrl>>();
    all_parsed_url.into_dataframe().unwrap().as_robj().clone()
}

#[derive(Debug, Default, Clone, IntoDataFrameRow)]
struct ParseUrl {
    scheme: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

impl From<Url> for ParseUrl {
    fn from(value: Url) -> Self {
        ParseUrl {
            scheme: Some(value.scheme().to_owned()),
            host: value.host_str().to_owned().map(str::to_string),
            port: value.port().to_owned(),
            path: Some(value.path().to_owned()),
            query: value.query().to_owned().map(str::to_string),
            fragment: value.fragment().to_owned().map(str::to_string),
            username: Some(value.username().to_owned()),
            password: value.password().to_owned().map(str::to_string),
        }
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod weburl;
    fn url_parse;
}
