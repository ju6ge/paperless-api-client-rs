# `paperless-api-client`

Paperless-ngx API client

[![docs.rs](https://docs.rs/paperless-api-client/badge.svg)](https://docs.rs/paperless-api-client)

## API Details

OpenAPI Spec for Paperless-ngx






## Client Details



The documentation for the crate is generated
along with the code to make this library easy to use.


To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
paperless-api-client = "6.0.1"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```rust,no_run
use paperless_api_client::Client;

let client = Client::new(
    String::from("api-key"),
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `PAPERLESS_API_CLIENT_API_TOKEN`


And then you can create a client from the environment.

```rust,no_run
use paperless_api_client::Client;

let client = Client::new_from_env();
```
