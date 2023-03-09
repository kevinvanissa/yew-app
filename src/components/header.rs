use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {

     <>
    <div class={"container-fluid p-5 bg-primary text-white text-center"}>
        <h1>{"Manage Your Diet Like a Pro"}</h1>
        <p>{"NyamUp is where you want to be"}</p> 
    </div>

    <nav class={"navbar navbar-dark bg-black"}>
    <div class={"container-fluid"}>


             <Link <Route>  classes={classes!("navbar-brand","text-white")} to={Route::Home}>
             { "NyamUP" }
             </Link<Route>>

  <ul class={"navbar-nav me-auto mb-2 mb-lg-0"}>
        <li class={"nav-item"}>
             <Link <Route>  classes={classes!("nav-link","text-white")} to={Route::Dashboard}>
             { "Dashboard" }
             </Link<Route>>
        </li>
    </ul>

             <Link <Route>  classes={classes!("nav-link","text-white")} to={Route::Home}>
             {"Log In"} {" | "}
             </Link<Route>>

             <Link <Route>  classes={classes!("nav-link","text-white")} to={Route::Home}>
             {" "}{" Log Out" }
             </Link<Route>>

    </div>
    </nav>
    </>
    }
}
