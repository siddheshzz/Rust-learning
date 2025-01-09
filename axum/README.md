# Learning Axum
- Hello route=> Query and Path params
- routes_static
- LOGIN API
- `tower_cookies::CookieManagerLayer` to manage cookies.
- `tower_http::services::ServeDir` to serve the static files in the `static` directory.
- `axum::middleware::map_response` to map the responses from the routes to the final response sent to the client.

The main file depicts the server of the login application.
The quick_dev which is a test file acts as a client for the testing purpose.

Command to run =>

**test**
```
cargo watch -q -c -w tests -x "test -q quick_dev -- --nocapture"
```


**server**
```
cargo watch -q -c -w src/ -x run
```


Significance of flags =>
```
 -c, --clear               Clear the screen before each run
 -q, --quiet               Suppress output from cargo-watch itself
 -x, --exec <cmd>...       Cargo command(s) to execute on changes [default: check]
 -w, --watch <watch>...    Watch specific file(s) or folder(s). Disables finding and watching local
                                     dependencies.

```


Create a first layer