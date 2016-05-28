

#[macro_export]
macro_rules! resource {
    ($name:ident) => {{
        use $crate::router::Router;
        let mut router = Router::new();
        router.get("/",         $name::index);
        router.get("/new",      $name::new);
        router.post("/",        $name::create);
        router.get("/:id",      $name::show);
        router.get("/:id/edit", $name::edit);
        router.put("/:id",      $name::update);
        router.delete("/:id",   $name::delete);
        router
    }}
}

