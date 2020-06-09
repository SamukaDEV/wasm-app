// use crate::pages::{About, Home};
use yew_router::{switch::Permissive, Switch};

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/about"]
    AboutPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}