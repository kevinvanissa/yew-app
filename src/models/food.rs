use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct TotalCalories {
    pub total_calories_breakfast: f64,
    pub total_calories_lunch: f64,
    pub total_calories_dinner: f64,
    pub total_calories_snack: f64,
    pub total_calories_burnt: f64
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct Food {
    pub name: String,
    pub data_type: String,
    pub fdc_id: i64,
    pub mealid: i64,
    pub mealtype: String,
    pub mealdate: String,
    pub nutrient_list: Vec<Nutrient>
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct NutrientMealInfo {
    pub total_protein: f64,
    pub total_fat: f64,
    pub total_carbs: f64,
    pub total_calories: f64,
    pub final_meal_list: Vec<Food>
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
pub struct Nutrient {
    pub name: String,
    pub amount: f64
}
