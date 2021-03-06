# trek-router

Routing

[![Build Status](https://travis-ci.org/trek-rs/router.svg?branch=master)](https://travis-ci.org/trek-rs/router)
[![Latest version](https://img.shields.io/crates/v/trek-router.svg)](https://crates.io/crates/trek-router)
[![Documentation](https://docs.rs/trek-router/badge.svg)](https://docs.rs/trek-router)
![License](https://img.shields.io/crates/l/trek-router.svg)

## Features

- Supports `get` `post` `delete` `patch` `put` `options` `head` `connect` `trace`.

- Supports `any` for above APIs.

- Supports `scope` for scope routes.

- Supports `resource` and `resources` for resourceful routes.

- Supports `middleware` **WIP**

## Usage

```rust
extern crate trek_router;

use trek_router::Router;

type F = fn() -> usize;
let mut router = Router::<F>::new();

// scope v1
router.scope("/v1", |v1| {
    v1.get("/login", || 0);
    v1.post("/submit", || 1);
    v1.delete("/read", || 2);
});

// scope v2
router.scope("/v2", |v2| {
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

### [Hello](examples/hello.rs)

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

type Handler = fn(Request<Body>, Params) -> Body;

fn v1_login(_: Request<Body>, _: Params) -> Body {
    Body::from("v1 login")
}

fn v1_submit(_req: Request<Body>, _: Params) -> Body {
    Body::from("v1 submit")
}

fn v1_read(_req: Request<Body>, _: Params) -> Body {
    Body::from("v1 read")
}

fn v2_login(_: Request<Body>, _: Params) -> Body {
    Body::from("v2 login")
}

fn v2_submit(_req: Request<Body>, _: Params) -> Body {
    Body::from("v2 submit")
}

fn v2_read(_req: Request<Body>, _: Params) -> Body {
    Body::from("v2 read")
}

fn users(_req: Request<Body>, _: Params) -> Body {
    Body::from("users")
}

fn foo(_: Request<Body>, _: Params) -> Body {
    Body::from("foo")
}

fn bar(_req: Request<Body>, _: Params) -> Body {
    Body::from("bar")
}

fn baz(_req: Request<Body>, _: Params) -> Body {
    Body::from("baz")
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let mut router = Router::<Handler>::new();

    router
        // scope v1
        .scope("/v1", |v1| {
            v1.get("/login", v1_login)
                .post("/submit", v1_submit)
                .delete("/read", v1_read);
        })
        // scope v2
        .scope("/v2", |v2| {
            v2.get("/login", v2_login)
                .post("/submit", v2_submit)
                .delete("/read", v2_read)
                // scope users
                .scope("users", |u| {
                    u.any("", users);
                });
        })
        .get("/foo", foo)
        .post("/bar", bar)
        .delete("/baz", baz);

    let router = Arc::new(router);

    let routing = move || {
        let router = Arc::clone(&router);

        service_fn_ok(move |req| {
            let method = req.method().to_owned();
            let path = req.uri().path().to_owned();

            match router.find(&method, &path) {
                Some((handler, params)) => Response::new(handler(req, params)),
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

### [RESTful Example](examples/rest.rs)

| HTTP Verb | Path                 | Action  |
| --------- | -------------------- | ------- |
| GET       | /geocoder/new        | new     |
| POST      | /geocoder            | create  |
| GET       | /geocoder            | show    |
| GET       | /geocoder/edit       | edit    |
| PATCH/PUT | /geocoder            | update  |
| DELETE    | /geocoder            | destroy |
| GET       | /users               | index   |
| GET       | /users/new           | new     |
| POST      | /users               | create  |
| GET       | /users/:user_id      | show    |
| GET       | /users/:user_id/edit | edit    |
| PATCH/PUT | /users/:user_id      | update  |
| DELETE    | /users/:user_id      | destroy |

```rust
extern crate futures;
extern crate hyper;
extern crate trek_router;

use futures::Future;
use hyper::server::Server;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;
use trek_router::{Resource, ResourceOptions, Resources, Router};

type Params = Vec<(String, String)>;
type Handler = fn(Context) -> Body;

struct Context {
    request: Request<Body>,
    params: Params,
}

struct Geocoder {}

impl Resource for Geocoder {
    type Context = Context;
    type Body = Body;

    fn show(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Show!");
        Body::from(s)
    }

    fn create(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Create!");
        Body::from(s)
    }

    fn update(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Update!");
        Body::from(s)
    }

    fn delete(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Delete!");
        Body::from(s)
    }

    fn edit(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder Edit!");
        Body::from(s)
    }

    fn new(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Geocoder New!");
        Body::from(s)
    }
}

struct Users {}

impl Resources for Users {
    type Context = Context;
    type Body = Body;

    fn index(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("Users Index!");
        Body::from(s)
    }

    fn create(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Create!");
        Body::from(s)
    }

    fn new(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User New!");
        Body::from(s)
    }

    fn show(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Show, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }

    fn update(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Update, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }

    fn delete(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Delete, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }

    fn edit(ctx: Self::Context) -> Self::Body {
        let mut s = String::new();
        s.push_str(&ctx.request.uri().path().to_owned());
        s.push_str("\n");
        s.push_str("User Edit, ");
        for (k, v) in ctx.params {
            s.push_str(&format!("{} = {}", k, v));
        }
        s.push_str("!");
        Body::from(s)
    }
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let mut router = Router::<Handler>::new();

    router.resource("/geocoder", Geocoder::build(ResourceOptions::default()));
    router.resources("/users", Users::build(ResourceOptions::default()));

    let router = Arc::new(router);

    let routing = move || {
        let router = Arc::clone(&router);

        service_fn_ok(move |request| {
            let method = request.method().to_owned();
            let path = request.uri().path().to_owned();

            match router.find(&method, &path) {
                Some((handler, params)) => Response::new(handler(Context {
                    request,
                    params: params
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                })),
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
