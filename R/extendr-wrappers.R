# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_weburl_wrappers", use_symbols = TRUE, package_name = "weburl")

#' @usage NULL
#' @useDynLib weburl, .registration = TRUE
NULL

#' Parse url
#' @param url character vector of urls
#' @return a data.frame consisting of the columns scheme, host, port, path, query and fragment.
#' @examples
#' library(weburl)
#'
#' url <- "https://user:pass@example.com:1234/foo/bar?baz#quux"
#'
#' url_parse(url)
#' @export
url_parse <- function(url) .Call(wrap__url_parse, url)

#' Get URL elements
#' @param url character vector of urls
#' @return a vector of characters
#' @name get_url_elements
#' @examples
#' library(weburl)
#'
#' url <- "https://user:pass@example.com:1234/foo/bar?baz#quux"
#'
#' get_scheme(url)
#' get_host(url)
#' get_port(url)
#' get_path(url)
#' get_query(url)
#' get_fragment(url)
#' get_username(url)
#' get_password(url)
#' @export
get_scheme <- function(url) .Call(wrap__get_scheme, url)

#' @rdname get_url_elements
#' @export
get_host <- function(url) .Call(wrap__get_host, url)

#' @rdname get_url_elements
#' @export
get_port <- function(url) .Call(wrap__get_port, url)

#' @rdname get_url_elements
#' @export
get_query <- function(url) .Call(wrap__get_query, url)

#' @rdname get_url_elements
#' @export
get_path <- function(url) .Call(wrap__get_path, url)

#' @rdname get_url_elements
#' @export
get_fragment <- function(url) .Call(wrap__get_fragment, url)

#' @rdname get_url_elements
#' @export
get_username <- function(url) .Call(wrap__get_username, url)

#' @rdname get_url_elements
#' @export
get_password <- function(url) .Call(wrap__get_password, url)

#' Set URL elements
#' @param url character vector of urls
#' @param scheme character
#' @param host character
#' @param port integer
#' @param path character vector
#' @param query named list
#' @param fragment character
#' @param username character
#' @param password character
#' @examples
#' library(weburl)
#'
#' url <- "https://example.com"
#'
#' set_scheme(url, "http")
#' set_host(url, "dummy.com")
#' set_port(url, 1234)
#' set_path(url, c("foo", "bar"))
#' set_query(url, list("baz" = "zar"))
#' set_fragment(url, "quux")
#' set_username(url, "user")
#' set_password(url, "pass")
#'
#' @return a vector of characters
#' @rdname set_url_elements
#' @export
set_scheme <- function(url, scheme) .Call(wrap__set_scheme, url, scheme)

#' @rdname set_url_elements
#' @export
set_host <- function(url, host) .Call(wrap__set_host, url, host)

#' @rdname set_url_elements
#' @export
set_port <- function(url, port) .Call(wrap__set_port, url, port)

#' @rdname set_url_elements
#' @export
set_path <- function(url, path) .Call(wrap__set_path, url, path)

#' @rdname set_url_elements
#' @export
set_query <- function(url, query) .Call(wrap__set_query, url, query)

#' @rdname set_url_elements
#' @export
set_fragment <- function(url, fragment) .Call(wrap__set_fragment, url, fragment)

#' @rdname set_url_elements
#' @export
set_username <- function(url, username) .Call(wrap__set_username, url, username)

#' @rdname set_url_elements
#' @export
set_password <- function(url, password) .Call(wrap__set_password, url, password)

#' Encode Url
#' @param url character vector of urls
#' @return a vector of characters
#' @examples
#' library(weburl)
#'
#' url <- "https:www.google.com/dummy=path yo"
#'
#' url_encode(url)
#' @export
url_encode <- function(url) .Call(wrap__url_encode, url)


# nolint end
