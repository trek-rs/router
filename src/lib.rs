extern crate http;
extern crate path_tree;

use http::Method;
use path_tree::PathTree;
use std::collections::HashMap;

pub type Trees<H> = HashMap<Method, PathTree<H>>;

#[derive(Clone, Debug)]
pub struct Router<H> {
    path: String,
    trees: Trees<H>,
    middleware: Vec<H>,
}

impl<H> Router<H>
where
    H: Clone,
{
    pub fn new() -> Self {
        Router {
            trees: Trees::new(),
            path: "/".to_owned(),
            middleware: Vec::new(),
        }
    }

    // middleware
    pub fn middleware(&mut self, handler: H) -> &mut Self {
        self.middleware.push(handler);
        self
    }

    // scope with prefix
    pub fn scope(&mut self, path: &str, builder: impl FnOnce(&mut Router<H>)) -> &mut Self {
        let mut group = Router {
            trees: self.trees.clone(),
            path: join_paths(&self.path, path),
            middleware: self.middleware.clone(),
        };
        builder(&mut group);
        self.trees = group.trees;

        // let parent_path = self.path.to_owned();
        // self.path = join_paths(&self.path, path);
        // build(self);
        // self.path = parent_path;

        self
    }

    fn _handle(&mut self, method: Method, path: &str, handler: H) -> &mut Self {
        // TODO: combine middleware + handler to finally handler
        self.trees
            .entry(method)
            .or_insert_with(|| PathTree::new())
            .insert(path, handler);
        self
    }

    pub fn handle(&mut self, method: Method, path: &str, handler: H) -> &mut Self {
        self._handle(method, &join_paths(&self.path, path), handler)
    }

    pub fn get(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::GET, path, handler)
    }

    pub fn post(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::POST, path, handler)
    }

    pub fn delete(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::DELETE, path, handler)
    }

    pub fn patch(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::PATCH, path, handler)
    }

    pub fn put(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::PUT, path, handler)
    }

    pub fn options(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::OPTIONS, path, handler)
    }

    pub fn head(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::HEAD, path, handler)
    }

    pub fn connect(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::CONNECT, path, handler)
    }

    pub fn trace(&mut self, path: &str, handler: H) -> &mut Self {
        self.handle(Method::TRACE, path, handler)
    }

    pub fn any(&mut self, path: &str, handler: H) -> &mut Self {
        let path = &join_paths(&self.path, path);
        self._handle(Method::GET, path, handler.clone());
        self._handle(Method::POST, path, handler.clone());
        self._handle(Method::DELETE, path, handler.clone());
        self._handle(Method::PATCH, path, handler.clone());
        self._handle(Method::PUT, path, handler.clone());
        self._handle(Method::OPTIONS, path, handler.clone());
        self._handle(Method::HEAD, path, handler.clone());
        self._handle(Method::CONNECT, path, handler.clone());
        self._handle(Method::TRACE, path, handler.clone())
    }

    pub fn resources(&mut self, path: &str) -> &mut Self {
        self
    }

    pub fn find<'a>(
        &'a self,
        method: &'a Method,
        path: &'a str,
    ) -> Option<(&'a H, Vec<(&'a str, &'a str)>)> {
        let tree = self.trees.get(method)?;
        tree.find(path)
    }
}

fn join_paths(a: &str, mut b: &str) -> String {
    if b.is_empty() {
        return a.to_owned();
    }
    b = b.trim_start_matches('/');
    a.trim_end_matches('/').to_owned() + "/" + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_router() {
        type F = fn() -> usize;
        let mut router = Router::<F>::new();

        router
            // scope v1
            .scope("/v1", |v1| {
                v1.get("/login", || 0)
                    .post("/submit", || 1)
                    .delete("/read", || 2);
            })
            // scope v2
            .scope("/v2", |v2| {
                v2.get("/login", || 0)
                    .post("/submit", || 1)
                    .delete("/read", || 2);
            })
            .get("/foo", || 3)
            .post("/bar", || 4)
            .delete("/baz", || 5);

        dbg!(&router);
    }
}
