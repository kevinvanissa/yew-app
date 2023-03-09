use yew::prelude::*;
use crate::routes::Route;
use yew_router::prelude::*;


#[derive(Properties, PartialEq)]
pub struct SquareProp {
    pub header: String,
    pub body: f64,
    pub scolor: String,
    pub datestring: String,
}

#[function_component(Square)]
pub fn square(SquareProp { header, body, scolor, datestring}: &SquareProp) -> Html {
    html! {

        <div class={scolor.clone()}>


         <Link <Route> to={Route::MealDay {mealdate: datestring.clone(), mealtype: header.clone() }}><h3 class={"mb-5"} align={"center"}>{ header.clone()}</h3></Link<Route>> 


        <h1 class={"mb-5"} align={"center"}> {body.clone()} </h1>
        <h5 align={"center"}> {"Carbs: | Fat: | Protein:"}</h5>
        <h5 align={"center"}> {"Calories: | Burnt: | Left:"}</h5>

        </div>
    }
}
