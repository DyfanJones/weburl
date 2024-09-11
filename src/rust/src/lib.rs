use extendr_api::prelude::*;
use itertools::Either;
use url::Url;

/// Get URL elements
/// @param url character vector of urls
/// @return a vector of characters
/// @name get_url_elements
/// @examples
/// library(weburl)
///
/// url <- "https://user:pass@example.com:1234/foo/bar?baz#quux"
///
/// get_scheme(url)
/// get_host(url)
/// get_port(url)
/// get_path(url)
/// get_query(url)
/// get_fragment(url)
/// get_username(url)
/// get_password(url)
/// @export
#[extendr]
fn get_scheme(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.scheme()),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_host(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.host_str().map(str::to_string)),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_port(url: Strings) -> Integers {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rint::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rint::from(u.port().into_robj().as_integer()),
                    Err(_) => Rint::na(),
                }
            }
        })
        .collect::<Integers>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_path(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.path()),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_query(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.query().map(str::to_string)),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_fragment(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.fragment().map(str::to_string)),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_username(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.username()),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname get_url_elements
/// @export
#[extendr]
fn get_password(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                Rstr::na()
            } else {
                let url_parse = Url::parse(urli);
                match url_parse {
                    Ok(u) => Rstr::from(u.password().map(str::to_string)),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

/// Parse url
/// @param url character vector of urls
/// @return a data.frame consisting of the columns scheme, host, port, path, query and fragment.
/// @examples
/// library(weburl)
///
/// url <- "https://user:pass@example.com:1234/foo/bar?baz#quux"
///
/// url_parse(url)
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
            host: value.host_str().map(str::to_string),
            port: value.port(),
            path: Some(value.path().to_owned()),
            query: value.query().map(str::to_string),
            fragment: value.fragment().map(str::to_string),
            username: Some(value.username().to_owned()),
            password: value.password().map(str::to_string),
        }
    }
}

/// Set URL elements
/// @param url character vector of urls
/// @param scheme character
/// @param host character
/// @param port integer
/// @param path character vector
/// @param query named list
/// @param fragment character
/// @param username character
/// @param password character
/// @examples
/// library(weburl)
///
/// url <- "https://example.com"
///
/// set_scheme(url, "http")
/// set_host(url, "dummy.com")
/// set_port(url, 1234)
/// set_path(url, c("foo", "bar"))
/// set_query(url, list("baz" = "zar"))
/// set_fragment(url, "quux")
/// set_username(url, "user")
/// set_password(url, "pass")
///
/// @return a vector of characters
/// @rdname set_url_elements
/// @export
#[extendr]
fn set_scheme(url: Strings, scheme: &str) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() | scheme.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        let _result = u.set_scheme(scheme);
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_host(url: Strings, host: &str) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() | host.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        let _result = u.set_host(Some(host));
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_port(url: Strings, port: i32) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() | port.is_na() {
                urli.clone()
            } else {
                let uport = u16::try_from(port).unwrap();
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        let _result = u.set_port(Some(uport));
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_path(url: Strings, path: Strings) -> Strings {
    let paths = path.to_vec();
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        u.path_segments_mut()
                            .map_err(|_| "cannot be base")
                            .unwrap()
                            .extend(&paths);
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_query(url: Strings, query: Either<List, Strings>) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                urli.clone()
            } else {
                match &query {
                    Either::Left(l) => url_query_list(urli, l.clone()),
                    Either::Right(s) => url_query_str(urli, s.clone()),
                }
            }
        })
        .collect::<Strings>()
}

fn url_query_list(urli: &Rstr, query: List) -> Rstr {
    if query.is_empty() {
        urli.clone()
    } else {
        let build_query: Vec<_> = query
            .iter()
            .map(|xi| (xi.0, xi.1.as_str().unwrap()))
            .collect();
        let url_update = Url::parse_with_params(urli, &build_query);
        match url_update {
            Ok(u) => Rstr::from(u.as_str()),
            Err(_) => urli.clone(),
        }
    }
}

fn url_query_str(urli: &Rstr, query: Strings) -> Rstr {
    let url_update = Url::parse(urli);
    match url_update {
        Ok(mut u) => {
            for i in 0..query.len() {
                let queryi = &query[i];
                if !queryi.is_na() {
                    let _result = u.set_query(Some(queryi));
                }
            }
            Rstr::from(u.as_str())
        }
        Err(_) => urli.clone(),
    }
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_fragment(url: Strings, fragment: &str) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() | fragment.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        let _result = u.set_fragment(Some(fragment));
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_username(url: Strings, username: &str) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() | username.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        let _result = u.set_username(username);
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// @rdname set_url_elements
/// @export
#[extendr]
fn set_password(url: Strings, password: &str) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() | password.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(mut u) => {
                        let _result = u.set_password(Some(password));
                        Rstr::from(u.as_str())
                    }
                    Err(_) => urli.clone(),
                }
            }
        })
        .collect::<Strings>()
}

/// Encode Url
/// @param url character vector of urls
/// @return a vector of characters
/// @examples
/// library(weburl)
///
/// url <- "https:www.google.com/dummy=path yo"
///
/// url_encode(url)
/// @export
#[extendr]
fn url_encode(url: Strings) -> Strings {
    url.into_iter()
        .map(|urli| {
            if urli.is_na() {
                urli.clone()
            } else {
                let url_update = Url::parse(urli);
                match url_update {
                    Ok(u) => Rstr::from(u.as_str()),
                    Err(_) => Rstr::na(),
                }
            }
        })
        .collect::<Strings>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod weburl;
    fn url_parse;
    fn get_scheme;
    fn get_host;
    fn get_port;
    fn get_query;
    fn get_path;
    fn get_fragment;
    fn get_username;
    fn get_password;
    fn set_scheme;
    fn set_host;
    fn set_port;
    fn set_path;
    fn set_query;
    fn set_fragment;
    fn set_username;
    fn set_password;
    fn url_encode;
}
