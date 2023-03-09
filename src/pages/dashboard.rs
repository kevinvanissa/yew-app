//use wasm_bindgen::JsValue;
use yew::prelude::*;
use crate::models::food::TotalCalories;
//use gloo_net::{http::Request, Error};
//use wasm_bindgen::prelude::*;
//use serde::Serialize;
//use  gloo_utils::format::JsValueSerdeExt;
use reqwest::{Error, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use std::{collections::HashMap};
//use gloo_console::log;
//use yew::services::fetch::Request;
use crate::components::{square::Square};
use chrono::{prelude::*, Duration};


#[function_component(Dashboard)]
pub fn dashboard() -> Html {

    let calories: UseStateHandle<Option<TotalCalories>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);
    //let app_date : UseStateHandle<Option<String>> = use_state(|| Local::now().format("%Y-%m-%d").to_string());
    //let app_date : UseStateHandle<Option<String>> = use_state(||  Some(Local::now().format("%Y-%m-%d").to_string()) );
    let app_date : UseStateHandle<Option<DateTime<Local>>> = use_state(||  Some(Local::now()));


    //let mydate = "2022-12-12";
       
     let dt: DateTime<Local> = Local::now();
     //let mydate = dt.format("%Y-%m-%d").to_string();
     let mdate = dt.format("%Y-%m-%d").to_string();
     //let mdate = mydate.clone();


//       let next_date = | | {
        //                 app_date.set(Some(app_date.unwrap() + Duration::days(1)));
       //};

       let next = {
          let calories = calories.clone();
          let error = error.clone();
          let app_date = app_date.clone();

            move |_| {

                 let calories = calories.clone();
                 let error = error.clone();

                let value = app_date.unwrap().clone() + Duration::days(1);
                app_date.set(Some(value));
       
                wasm_bindgen_futures::spawn_local(async move {
                    let mut map = HashMap::new();
                    //map.insert("meal_date", mdates);
                    map.insert("meal_date", value.format("%Y-%m-%d").to_string());

                   // let object = JsValue::from("any JsValue can be logged");
                    //log!("text", object);

                    let client = reqwest::Client::new();

                    let response = client.post("http://localhost:5000/usermealtotals")
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
                            match response.json::<TotalCalories>().await {
                                //Ok(parsed) => println!("Success! {:?}", parsed),
                                Ok(parsed) => {

                                    calories.set(Some(parsed));
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
    }
    }; 

   let  prev =  {
     
    let calories = calories.clone();
          let error = error.clone();
          let app_date = app_date.clone();

            move |_| {

                 let calories = calories.clone();
                 let error = error.clone();

                let value = app_date.unwrap().clone() - Duration::days(1);
                app_date.set(Some(value));
       
                wasm_bindgen_futures::spawn_local(async move {
                    let mut map = HashMap::new();
                    //map.insert("meal_date", mdates);
                    map.insert("meal_date", value.format("%Y-%m-%d").to_string());

                   // let object = JsValue::from("any JsValue can be logged");
                    //log!("text", object);

                    let client = reqwest::Client::new();

                    let response = client.post("http://localhost:5000/usermealtotals")
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
                            match response.json::<TotalCalories>().await {
                                //Ok(parsed) => println!("Success! {:?}", parsed),
                                Ok(parsed) => {

                                    calories.set(Some(parsed));
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
    }
   };




       {
        //create copies of states
        let calories = calories.clone();
        let error = error.clone();
        let app_date = app_date.clone();

        let mdates = mdate.clone();



        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let mut map = HashMap::new();
                    //map.insert("meal_date", mdates);
                    map.insert("meal_date", app_date.as_ref().unwrap().format("%Y-%m-%d").to_string());

                   // let object = JsValue::from("any JsValue can be logged");
                    //log!("text", object);

                    let client = reqwest::Client::new();

                    let response = client.post("http://localhost:5000/usermealtotals")
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
                            match response.json::<TotalCalories>().await {
                                //Ok(parsed) => println!("Success! {:?}", parsed),
                                Ok(parsed) => {

                                    calories.set(Some(parsed));
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

        let calories_list_logic = match calories.as_ref() {
           
            Some(calories) => html! {
                <>
                <div class={"row d-flex justify-content-center"}>
                        <h4 align={"center"} class="mb-4">
                           <button class="btn btn-primary btn-sm" onclick={prev}>{ "<< PREV" }</button> 
                          <span>{" "}{app_date.as_ref().unwrap().format("%a, %b %e %Y")} {" "}</span>
                           <button class="btn btn-primary btn-sm" onclick={next}>{ "NEXT >>" }</button> 
                        </h4>

                    <div class={"col-md-6 d-flex justify-content-center"}>
                        <Square header={"Breakfast"}  body= {calories.total_calories_breakfast} scolor={"msquare breakfast"} datestring={app_date.as_ref().unwrap().format("%Y-%m-%d").to_string().clone()} />
                    </div>


                    <div class={"col-md-6 d-flex justify-content-center"}>
                    <Square header={"Lunch"}  body={calories.total_calories_lunch} scolor={"msquare lunch"} datestring={app_date.as_ref().unwrap().format("%Y-%m-%d").to_string().clone()} />
                    </div>

                 </div>

                <div class={"row d-flex justify-content-center"}>

                    <div class={"col-md-6 d-flex justify-content-center"}>
                    <Square header={"Exercise"}  body={calories.total_calories_burnt} scolor={"msquare exercise"} datestring={app_date.as_ref().unwrap().format("%Y-%m-%d").to_string().clone()} />
                    </div>
                </div>


                <div class={"row d-flex justify-content-center"}>
                    <div class={"col-md-6 d-flex justify-content-center"}>
                        <Square header={"Dinner"}  body={calories.total_calories_dinner} scolor={"msquare dinner"} datestring={app_date.as_ref().unwrap().format("%Y-%m-%d").to_string().clone()} />
                    </div>

                    <div class={"col-md-6 d-flex justify-content-center"}>
                    <Square header={"Snack"}  body={calories.total_calories_snack} scolor={"msquare snack"} datestring={app_date.as_ref().unwrap().format("%Y-%m-%d").to_string().clone()} />
                    </div>
                
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
        <h1 align={"center"}>{"Dashboard"}</h1>
        {calories_list_logic}
      </>


    }
}
