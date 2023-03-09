use yew::functional::*;
//use gloo_net::{http::Request, Error};
//use models::user::Users;
use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod models;
mod pages;
mod routes;

use routes::Route;

use components::{card::Card, header::Header, loader::Loader, message::Message};
use pages::{home::Home, dashboard::Dashboard, mealday::MealDay};



#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick_callback = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button onclick={onclick_callback}>{ "Go Home" }</button>
        </div>
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
        <Header />
        <div class={"container mt-5"}>
            <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Dashboard => html!{
            <Dashboard />
        },
        Route::MealDay {mealdate, mealtype} => html!{
            <MealDay mealdate={mealdate.clone()} mealtype={mealtype.clone()} />
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

/* 
#[function_component(App)]
fn app() -> Html {
    let users: UseStateHandle<Option<Users>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        //create copies of states
        let users = users.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Request::get("https://dummyjson.com/users").send().await;

                    match fetched_users {
                        Ok(response) => {
                            let json = response.json::<Users>().await;
                            match json {
                                Ok(json_resp) => {
                                    users.set(Some(json_resp));
                                }
                                Err(e) => error.set(Some(e)),
                            }
                        }
                        Err(e) => error.set(Some(e)),
                    }
                });
                || ()
            },
            (),
        );
    }

    let user_list_logic = match users.as_ref() {
        Some(users) => 
             users
            .users
            .iter()
            .map(|user| {
                html! {
                  <Card user={user.clone() }/>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(_) => {
                html! {
                    <Message text={"Error getting list of users"} css_class={"text-danger"}/>
                }
            }
            None => {
                html! {
                  <Loader />
                }
            }
        },
    };

    html! {
      <>
        <Header />
      //  {user_list_logic}
      </>
    }
} 
 */
fn main() {
    yew::Renderer::<App>::new().render();
}