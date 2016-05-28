

#[macro_export]
macro_rules! resource {
    ($router:ident, $path:expr, $name:ident) => {{
        $router.get(format!("/{}",          &$path), $name::index);
        $router.get(format!("/{}/new",      &$path), $name::new);
        $router.post(format!("/{}",         &$path), $name::create);
        $router.get(format!("/{}/:id",      &$path), $name::show);
        $router.get(format!("/{}/:id/edit", &$path), $name::edit);
        $router.put(format!("/{}/:id",      &$path), $name::update);
        $router.delete(format!("/{}/:id",   &$path), $name::delete);
    }}
}

