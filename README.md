## What is this?

A client library for the [Supernova](https://gitlab.com/claudiop/Supernova/) REST API.

## Usage

```rust
use iron_planet::{Supernova, RequestConfig};

fn main() {
    // Instantiate an unprivileged client
    let sn = Supernova::new();
    let conf = RequestConfig::default();
    // One can use it to query endpoints that require no auth
    let result = sn.get_buildings(&conf); // Will succeed, hopefully
    // But not to query endpoints that require user auth
    let result = sn.get_class_inst(123, &conf); // Will fail, hopefully
    // It is possible to enable authentication in two different ways
    // One can provide authentication credentials in the form of a user-pass
    // A successful login will return a result with the session token
    let token = sn.login("user", "pass").unwrap();
    sn.logout();
    // The second one is to provide an existing token
    sn.set_auth_token(token);
}
```

## Important notes

- Every response is cached and currently there is no way to bypass cache other than instantiating a new client.
- One can call `warmup()` upon instantiation to eagerly load the buildings, courses, classes, departments and places.
  This might be desirable to speed up subsequent access in exchange for a startup penalty (between 5 and 15 seconds).
- If one doesn't desire all of these, it is possible to simply call the `get_[collection]` functions independently.
- Most relations are lazily loaded through a special type of pointer that is bound to the client. These can be loaded
  concurrently.
- Yes, the library **is** thread-safe. An async port might happen, but for now this is it.