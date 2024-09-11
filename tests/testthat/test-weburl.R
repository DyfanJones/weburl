test_that("check setting everything", {
  url <- "http://example.com"

  actual <- set_scheme(url, "https") |>
    set_port(1234) |>
    set_path(c("foo", "bar")) |>
    set_query("baz") |>
    set_fragment("quux") |>
    set_username("user") |>
    set_password("pass")
  expect_equal(actual, "https://user:pass@example.com:1234/foo/bar?baz#quux")
})
