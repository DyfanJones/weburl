
<!-- README.md is generated from README.Rmd. Please edit that file -->

# weburl

<!-- badges: start -->

[![R-CMD-check](https://github.com/DyfanJones/weburl/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/DyfanJones/weburl/actions/workflows/R-CMD-check.yaml)
[![:name status
badge](https://dyfanjones.r-universe.dev/badges/:name)](https://dyfanjones.r-universe.dev/)
[![weburl status
badge](https://dyfanjones.r-universe.dev/badges/weburl)](https://dyfanjones.r-universe.dev/weburl)
[![CRAN
status](https://www.r-pkg.org/badges/version/weburl)](https://CRAN.R-project.org/package=weburl)
<!-- badges: end -->

`{weburl}`wraps the [Rust crate url](https://crates.io/crates/url). This
package is under development.

## Installation

You can install the development version of weburl like so:

``` r
if (!requireNamespace("remotes")) install.packages("remotes")

remotes::install_github("dyfanjones/weburl")
```

r-universe installation:

``` r
install.packages("weburl", repos = c("https://dyfanjones.r-universe.dev", "https://cloud.r-project.org"))
```

## Example

This is a basic example which shows you how to solve a common problem:

``` r
library(weburl)
urls <- c(
  "/",
  "//google.com",
  "file:///",
  "http://google.com/",
  "http://google.com/path",
  "http://google.com/path?a=1&b=2",
  "http://google.com:1234/path?a=1&b=2",
  "http://google.com:8080/path?a=1&b=2#frag",
  "http://google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag",
  "http://user@google.com/path?a=1&b=2",
  "http://user:pass@google.com:8000/path?a=1&b=2",
  "svn+ssh://my.svn.server/repo/trunk",
  "https://google.com:8080"
)
```

Parse all Urls into a data.frame

``` r
url_parse(urls)
#>     scheme          host port        path                     query fragment
#> 1     <NA>          <NA>   NA        <NA>                      <NA>     <NA>
#> 2     <NA>          <NA>   NA        <NA>                      <NA>     <NA>
#> 3     file          <NA>   NA           /                      <NA>     <NA>
#> 4     http    google.com   NA           /                      <NA>     <NA>
#> 5     http    google.com   NA       /path                      <NA>     <NA>
#> 6     http    google.com   NA       /path                   a=1&b=2     <NA>
#> 7     http    google.com 1234       /path                   a=1&b=2     <NA>
#> 8     http    google.com 8080       /path                   a=1&b=2     frag
#> 9     http    google.com 8080       /path a=1&b=2&c=%7B1%7B2%7D3%7D     frag
#> 10    http    google.com   NA       /path                   a=1&b=2     <NA>
#> 11    http    google.com 8000       /path                   a=1&b=2     <NA>
#> 12 svn+ssh my.svn.server   NA /repo/trunk                      <NA>     <NA>
#> 13   https    google.com 8080           /                      <NA>     <NA>
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
#> 13              <NA>
```

Get Url elements:

``` r
get_scheme(urls)
#>  [1] NA        NA        "file"    "http"    "http"    "http"    "http"   
#>  [8] "http"    "http"    "http"    "http"    "svn+ssh" "https"
get_host(urls)
#>  [1] NA              NA              NA              "google.com"   
#>  [5] "google.com"    "google.com"    "google.com"    "google.com"   
#>  [9] "google.com"    "google.com"    "google.com"    "my.svn.server"
#> [13] "google.com"
get_port(urls)
#>  [1]   NA   NA   NA   NA   NA   NA 1234 8080 8080   NA 8000   NA 8080
get_query(urls)
#>  [1] NA                          NA                         
#>  [3] NA                          NA                         
#>  [5] NA                          "a=1&b=2"                  
#>  [7] "a=1&b=2"                   "a=1&b=2"                  
#>  [9] "a=1&b=2&c=%7B1%7B2%7D3%7D" "a=1&b=2"                  
#> [11] "a=1&b=2"                   NA                         
#> [13] NA
get_path(urls)
#>  [1] NA            NA            "/"           "/"           "/path"      
#>  [6] "/path"       "/path"       "/path"       "/path"       "/path"      
#> [11] "/path"       "/repo/trunk" "/"
get_fragment(urls)
#>  [1] NA     NA     NA     NA     NA     NA     NA     "frag" "frag" NA    
#> [11] NA     NA     NA
get_username(urls)
#>  [1] NA     NA     ""     ""     ""     ""     ""     ""     ""     "user"
#> [11] "user" ""     ""
get_password(urls)
#>  [1] NA     NA     NA     NA     NA     NA     NA     NA     NA     NA    
#> [11] "pass" NA     NA
```

Set Url elements:

``` r
set_scheme(urls, "https")
#>  [1] "/"                                                          
#>  [2] "//google.com"                                               
#>  [3] "file:///"                                                   
#>  [4] "https://google.com/"                                        
#>  [5] "https://google.com/path"                                    
#>  [6] "https://google.com/path?a=1&b=2"                            
#>  [7] "https://google.com:1234/path?a=1&b=2"                       
#>  [8] "https://google.com:8080/path?a=1&b=2#frag"                  
#>  [9] "https://google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag"
#> [10] "https://user@google.com/path?a=1&b=2"                       
#> [11] "https://user:pass@google.com:8000/path?a=1&b=2"             
#> [12] "svn+ssh://my.svn.server/repo/trunk"                         
#> [13] "https://google.com:8080/"
set_host(urls, "helloworld")
#>  [1] "/"                                                         
#>  [2] "//google.com"                                              
#>  [3] "file://helloworld/"                                        
#>  [4] "http://helloworld/"                                        
#>  [5] "http://helloworld/path"                                    
#>  [6] "http://helloworld/path?a=1&b=2"                            
#>  [7] "http://helloworld:1234/path?a=1&b=2"                       
#>  [8] "http://helloworld:8080/path?a=1&b=2#frag"                  
#>  [9] "http://helloworld:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag"
#> [10] "http://user@helloworld/path?a=1&b=2"                       
#> [11] "http://user:pass@helloworld:8000/path?a=1&b=2"             
#> [12] "svn+ssh://helloworld/repo/trunk"                           
#> [13] "https://helloworld:8080/"
set_port(urls, 8080)
#>  [1] "/"                                                         
#>  [2] "//google.com"                                              
#>  [3] "file:///"                                                  
#>  [4] "http://google.com:8080/"                                   
#>  [5] "http://google.com:8080/path"                               
#>  [6] "http://google.com:8080/path?a=1&b=2"                       
#>  [7] "http://google.com:8080/path?a=1&b=2"                       
#>  [8] "http://google.com:8080/path?a=1&b=2#frag"                  
#>  [9] "http://google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag"
#> [10] "http://user@google.com:8080/path?a=1&b=2"                  
#> [11] "http://user:pass@google.com:8080/path?a=1&b=2"             
#> [12] "svn+ssh://my.svn.server:8080/repo/trunk"                   
#> [13] "https://google.com:8080/"
set_query(urls, list("foo" = "bar=yo"))
#>  [1] "/"                                                                      
#>  [2] "//google.com"                                                           
#>  [3] "file:///?foo=bar%3Dyo"                                                  
#>  [4] "http://google.com/?foo=bar%3Dyo"                                        
#>  [5] "http://google.com/path?foo=bar%3Dyo"                                    
#>  [6] "http://google.com/path?a=1&b=2&foo=bar%3Dyo"                            
#>  [7] "http://google.com:1234/path?a=1&b=2&foo=bar%3Dyo"                       
#>  [8] "http://google.com:8080/path?a=1&b=2&foo=bar%3Dyo#frag"                  
#>  [9] "http://google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D&foo=bar%3Dyo#frag"
#> [10] "http://user@google.com/path?a=1&b=2&foo=bar%3Dyo"                       
#> [11] "http://user:pass@google.com:8000/path?a=1&b=2&foo=bar%3Dyo"             
#> [12] "svn+ssh://my.svn.server/repo/trunk?foo=bar%3Dyo"                        
#> [13] "https://google.com:8080/?foo=bar%3Dyo"
set_path(urls, c("path", "to", "somewhere"))
#>  [1] "/"                                                                           
#>  [2] "//google.com"                                                                
#>  [3] "file:///path/to/somewhere"                                                   
#>  [4] "http://google.com/path/to/somewhere"                                         
#>  [5] "http://google.com/path/path/to/somewhere"                                    
#>  [6] "http://google.com/path/path/to/somewhere?a=1&b=2"                            
#>  [7] "http://google.com:1234/path/path/to/somewhere?a=1&b=2"                       
#>  [8] "http://google.com:8080/path/path/to/somewhere?a=1&b=2#frag"                  
#>  [9] "http://google.com:8080/path/path/to/somewhere?a=1&b=2&c=%7B1%7B2%7D3%7D#frag"
#> [10] "http://user@google.com/path/path/to/somewhere?a=1&b=2"                       
#> [11] "http://user:pass@google.com:8000/path/path/to/somewhere?a=1&b=2"             
#> [12] "svn+ssh://my.svn.server/repo/trunk/path/to/somewhere"                        
#> [13] "https://google.com:8080/path/to/somewhere"
set_fragment(urls, "cell=4,1-6,2")
#>  [1] "/"                                                                 
#>  [2] "//google.com"                                                      
#>  [3] "file:///#cell=4,1-6,2"                                             
#>  [4] "http://google.com/#cell=4,1-6,2"                                   
#>  [5] "http://google.com/path#cell=4,1-6,2"                               
#>  [6] "http://google.com/path?a=1&b=2#cell=4,1-6,2"                       
#>  [7] "http://google.com:1234/path?a=1&b=2#cell=4,1-6,2"                  
#>  [8] "http://google.com:8080/path?a=1&b=2#cell=4,1-6,2"                  
#>  [9] "http://google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#cell=4,1-6,2"
#> [10] "http://user@google.com/path?a=1&b=2#cell=4,1-6,2"                  
#> [11] "http://user:pass@google.com:8000/path?a=1&b=2#cell=4,1-6,2"        
#> [12] "svn+ssh://my.svn.server/repo/trunk#cell=4,1-6,2"                   
#> [13] "https://google.com:8080/#cell=4,1-6,2"
set_username(urls, "foo")
#>  [1] "/"                                                             
#>  [2] "//google.com"                                                  
#>  [3] "file:///"                                                      
#>  [4] "http://foo@google.com/"                                        
#>  [5] "http://foo@google.com/path"                                    
#>  [6] "http://foo@google.com/path?a=1&b=2"                            
#>  [7] "http://foo@google.com:1234/path?a=1&b=2"                       
#>  [8] "http://foo@google.com:8080/path?a=1&b=2#frag"                  
#>  [9] "http://foo@google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag"
#> [10] "http://foo@google.com/path?a=1&b=2"                            
#> [11] "http://foo:pass@google.com:8000/path?a=1&b=2"                  
#> [12] "svn+ssh://foo@my.svn.server/repo/trunk"                        
#> [13] "https://foo@google.com:8080/"
set_password(urls, "bar")
#>  [1] "/"                                                              
#>  [2] "//google.com"                                                   
#>  [3] "file:///"                                                       
#>  [4] "http://:bar@google.com/"                                        
#>  [5] "http://:bar@google.com/path"                                    
#>  [6] "http://:bar@google.com/path?a=1&b=2"                            
#>  [7] "http://:bar@google.com:1234/path?a=1&b=2"                       
#>  [8] "http://:bar@google.com:8080/path?a=1&b=2#frag"                  
#>  [9] "http://:bar@google.com:8080/path?a=1&b=2&c=%7B1%7B2%7D3%7D#frag"
#> [10] "http://user:bar@google.com/path?a=1&b=2"                        
#> [11] "http://user:bar@google.com:8000/path?a=1&b=2"                   
#> [12] "svn+ssh://:bar@my.svn.server/repo/trunk"                        
#> [13] "https://:bar@google.com:8080/"
```

Building Url by piping:

``` r
url <- "http://example.com"

set_scheme(url, "https") |>
  set_port(1234) |>
  set_path(c("foo", "bar")) |>
  set_query("baz") |>
  set_fragment("quux") |>
  set_username("user") |>
  set_password("pass")
#> [1] "https://user:pass@example.com:1234/foo/bar?baz#quux"
```

Url encoding:

``` r
url_encode("https://example.com/?country=español")
#> [1] "https://example.com/?country=espa%C3%B1ol"
```
