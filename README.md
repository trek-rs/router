# trek-router

A flexible router for RESTful APIs. Powered by [path-tree].

[![Build Status](https://travis-ci.org/trek-rs/router.svg?branch=master)](https://travis-ci.org/trek-rs/router)
[![Latest version](https://img.shields.io/crates/v/trek-router.svg)](https://crates.io/crates/trek-router)
[![Documentation](https://docs.rs/trek-router/badge.svg)](https://docs.rs/trek-router)
![License](https://img.shields.io/crates/l/trek-router.svg)

## Features

- Supports `get` `post` `delete` `patch` `put` `options` `head` `connect` `trace`.

- Supports `any` for above APIs.

- Supports `group` for scope router.

- Supports `middleware` **WIP**

## Usage

```rust
extern crate trek_router;

use trek_router::Router;

type F = fn() -> usize;
let mut router = Router::<F>::new();

// Simple group: v1
router.group("/v1", |v1| {
    v1.get("/login", || 0);
    v1.post("/submit", || 1);
    v1.delete("/read", || 2);
});

// Simple group: v2
router.group("/v2", |v2| {
    v2.get("/login", || 0);
    v2.post("/submit", || 1);
    v2.delete("/read", || 2);
});

router.get("/foo", || 3);
router.post("/bar", || 4);
router.delete("/baz", || 5);

dbg!(&router);
```

## Examples

```rust
extern crate futures;
extern crate hyper;
extern crate trek_router;

use futures::Future;
use hyper::server::Server;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;
use trek_router::Router;

type Params<'a> = Vec<(&'a str, &'a str)>;

type Handler = fn(Request<Body>, Params) -> Response<Body>;

fn v1_login(_: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("v1 login"))
}

fn v1_submit(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("v1 submit"))
}

fn v1_read(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("v1 read"))
}

fn v2_login(_: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("v2 login"))
}

fn v2_submit(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("v2 submit"))
}

fn v2_read(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("v2 read"))
}

fn foo(_: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("foo"))
}

fn bar(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("bar"))
}

fn baz(_req: Request<Body>, _: Params) -> Response<Body> {
    Response::new(Body::from("baz"))
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let mut router = Router::<Handler>::new();

    // Simple group: v1
    router.group("/v1", |v1| {
        v1.get("/login", v1_login);
        v1.post("/submit", v1_submit);
        v1.delete("/read", v1_read);
    });

    // Simple group: v2
    router.group("/v2", |v2| {
        v2.get("/login", v2_login);
        v2.post("/submit", v2_submit);
        v2.delete("/read", v2_read);
    });

    router.get("/foo", foo);
    router.post("/bar", bar);
    router.delete("/baz", baz);

    let router = Arc::new(router);

    let routing = move || {
        let router = Arc::clone(&router);

        service_fn_ok(move |req| {
            let method = req.method().to_owned();
            let path = req.uri().path().to_owned();

            match router.find(&method, &path) {
                Some((handler, params)) => handler(req, params),
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap(),
            }
        })
    };

    let server = Server::bind(&addr)
        .serve(routing)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
```

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

[path-tree]: https://github.com/trek-rs/path-tree
