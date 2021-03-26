A straightforward util serving contents of a single file over HTTP on requests to any path with the `content-type`
header value dictated by the `type` query param.

Written to do some testing how some programs react to given content-types.

To run it:

```
cargo run -- /path/to/file [port]
```

if port is not provided, the file is by default served on port 8585.

After starting it will accept requests in the form of
`http://localhost:8585/any-path-string?type=requested/type` and respond
to them with the `content-type` set to the value of `type` param and
the contents of the file in the body.

Written using [`hyper`](https://hyper.rs/), [`routerify`](https://github.com/routerify/routerify),
and [`form_urlencoded`](https://crates.io/crates/form_urlencoded).
