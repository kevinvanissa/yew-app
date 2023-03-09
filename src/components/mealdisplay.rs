use yew::prelude::*;

use crate::models::food::Food;

#[derive(Properties, PartialEq)]

pub struct MealCardProp {
    pub food: Food,
}

#[function_component(MealCard)]
pub fn card(MealCardProp { food }: &MealCardProp) -> Html {
    html! {
<div class={"row justify-content-center"}>
    <div class="m-3 p-4 border rounded d-flex align-items-center">
        <img src="https://robohash.org/hicveldicta.png?size=50x50&set=set1" class="mr-2" alt="img" />
        <div class="">
            <p class="fw-bold mb-1">{format!("{} {}", food.name.clone(), food.data_type.clone())}</p>
            <p class="fw-normal mb-1">{food.mealtype.clone()}</p>
            <p class="fw-normal mb-1">{food.mealdate.clone()}</p>
        </div>
    </div>
</div>


    }
}
