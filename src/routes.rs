use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/mealday/:mealdate/:mealtype")]
    MealDay {mealdate: String, mealtype: String},
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}