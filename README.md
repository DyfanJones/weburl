
<!-- README.md is generated from README.Rmd. Please edit that file -->

# weburl

<!-- badges: start -->
<!-- badges: end -->

`{weburl}`wraps the [Rust crate url](https://crates.io/crates/url). This
package is under development.

## Installation

You can install the development version of weburl like so:

``` r
if (!requireNamespace("remotes")) install.packages("remotes")

remotes::install_github("dyfanjones/weburl")
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(weburl)
## basic example code

urls <- c(
  "/",
  "//google.com",
  "file:///",
  "http://google.com/",
  "http://google.com/path",
  "http://google.com/path?a=1&b=2",
  "http://google.com:80/path?a=1&b=2",
  "http://google.com:80/path?a=1&b=2#frag",
  "http://google.com:80/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag",
  "http://user@google.com:80/path?a=1&b=2",
  "http://user:pass@google.com:80/path?a=1&b=2",
  "svn+ssh://my.svn.server/repo/trunk"
)
weburl::url_parse(urls)
#>     scheme          host port        path                     query fragment
#> 1     <NA>          <NA>   NA        <NA>                      <NA>     <NA>
#> 2     <NA>          <NA>   NA        <NA>                      <NA>     <NA>
#> 3     file          <NA>   NA           /                      <NA>     <NA>
#> 4     http    google.com   NA           /                      <NA>     <NA>
#> 5     http    google.com   NA       /path                      <NA>     <NA>
#> 6     http    google.com   NA       /path                   a=1&b=2     <NA>
#> 7     http    google.com   NA       /path                   a=1&b=2     <NA>
#> 8     http    google.com   NA       /path                   a=1&b=2     frag
#> 9     http    google.com   NA       /path a=1&b=2&c=%7B1%7B2%7D3%7D     frag
#> 10    http    google.com   NA       /path                   a=1&b=2     <NA>
#> 11    http    google.com   NA       /path                   a=1&b=2     <NA>
#> 12 svn+ssh my.svn.server   NA /repo/trunk                      <NA>     <NA>
#>    username password
#> 1      <NA>     <NA>
#> 2      <NA>     <NA>
#> 3               <NA>
#> 4               <NA>
#> 5               <NA>
#> 6               <NA>
#> 7               <NA>
#> 8               <NA>
#> 9               <NA>
#> 10     user     <NA>
#> 11     user     pass
#> 12              <NA>
```
