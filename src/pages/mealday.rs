use yew::prelude::*;
use crate::models::food::{TotalCalories, NutrientMealInfo};
use reqwest::{Error, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::{collections::HashMap};
use crate::components::{square::Square, mealdisplay::MealCard};


#[derive(Properties, PartialEq)]
pub struct MealDayProp {
    pub mealdate: String,
    pub mealtype: String
}



#[function_component(MealDay)]
pub fn mealday(MealDayProp{mealdate, mealtype}: &MealDayProp) -> Html {

    let nutrient_meals: UseStateHandle<Option<NutrientMealInfo>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);
    let food_search_results: UseStateHandle<Option<HashMap<String, String>>> = use_state(||None);
    let search_term: UseStateHandle<Option<String>> = use_state(|| None);

    let mydate = mealdate.clone();
    //let mydate = "2022-12-12";
    let mealtype = mealtype.clone();
    //let mealtype = "Breakfast";
      
    



    let on_input_change = {
        let search_term = search_term.clone();

        move |_| {

        }

    };


       {
        //create copies of states
        let nutrient_meals = nutrient_meals.clone();
        let error = error.clone();


        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut map = HashMap::new();
                    map.insert("meal_date", mydate);
                    map.insert("meal_type", mealtype);


                    let client = reqwest::Client::new();

                    let response = client.post("http://localhost:5000/usermeals")
                    .header(CONTENT_TYPE, "application/json")
                    .header(AUTHORIZATION, "Bearer eyJhbGciOiJIUzI1NiJ9.eyJpZCI6MX0.17itMtCzPavgIPt0OaPMPEZJiwASqDUppyY5u76GZEE")
                    .header(ACCEPT, "application/json")
                    .json(&map)
                    .send()
                    .await
                    .unwrap();


                  //  let total_calories = response.json::<TotalCalories>().await.unwrap();
                   // calories.set(Some(total_calories));


                     match response.status() {
                        reqwest::StatusCode::OK => {
                            // on success, parse our JSON to an APIResponse
                            match response.json::<NutrientMealInfo>().await {
                                //Ok(parsed) => println!("Success! {:?}", parsed),
                                Ok(parsed) => {

                                    nutrient_meals.set(Some(parsed));
                                },
                                //Err(_) => println!("Hm, the response didn't match the shape we expected."),
                                Err(e) => error.set(Some(e)),
                            };
                        }
                        reqwest::StatusCode::UNAUTHORIZED => {
                            println!("Need to grab a new token");
                        }
                        other => {
                            panic!("Uh oh! Something unexpected happened: {:?}", other);
                        }
                    };

                });
                || ()
            },
            (),
        );
    }

        let nutrient_meals_list_logic = match nutrient_meals.as_ref() {
           
            Some(nutrientinfo) => html! {
                <>
                <div class={"row d-flex justify-content-center"}>

             <div class={"col-4"}>
                <h2 align="center">{"Add Food"}</h2>
                <div class="card m-3 p-4 border rounded d-flex align-items-center">
                <form id="newfoodform">
                    <label class="label">{"Search Food"}</label>
                    <br />
                    <input type="text" name="food_name" id="food_name" placeholder="Search Food..." oninput={on_input_change}  />


                </form>
                <br />    
                <br />    
                <br />    
                <div id="search_results">
                    {"See Search Results here...."}
                </div>

                </div>
            </div>



             <div class={"col-8"}>

                   <h4>{format!("Protein: {} Fat: {} Carbs: {}", nutrientinfo.total_protein, nutrientinfo.total_fat, nutrientinfo.total_carbs)}</h4>

                  <div> 
                  { 
                  nutrientinfo.final_meal_list.iter().map(|food| {
                    html! {
                    <MealCard food={food.clone()}  />
                    }   
                   }).collect::<Html>()
                }
                </div>
            </div>







/* 
                    <div class={"col-md-6 d-flex justify-content-center"}>
                       <Square header={"Lunch"}  body={calories.total_protein} scolor={"msquare lunch"} datestring={mydate} />
                    </div>
 */
                 </div>

             


              
                </>
            },
         None => match error.as_ref() {
            Some(_) => {
                html! {
                   <h5>{"Error...."} </h5>
                }
            }
            None => {
                html! {
                   <h5>{"Loading...."} </h5>
                }
            }
        },
        };


    html! {
        <>
        <h1 align={"center"}>{"Meal Day"}</h1>
        {nutrient_meals_list_logic}
      </>


    }
}
