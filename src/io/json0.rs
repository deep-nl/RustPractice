#![allow(unused)]
#![allow(dead_code)]
/** Compare two different approaches to parse a json file
 * A dynamic approach: the assumption here is that we do not fully understand the data present in the JSON file, 
   so our program will have to check the existence and type of any data fields dynamically.
 * A static approach: The assumption here is that we fully understand the data present in the JSON file, 
   so we will use deserialization to check for the existence and type of any field.
 */
use serde_json::{Number, Value, }; // use for dynamic, others are used for static
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Food {
    id: u32,
    name: String,
    missy_likeness: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Schedule {
    date: i64,
    quantity: f64,
    food: u32,
    missy_grumpiness: u32,
}

#[derive(Deserialize, Serialize, Debug)]
struct MissyFoodSchedule {
    food: Vec<Food>,
    missy_food_schedule: Vec<Schedule>,
}

const TEST_JSON_FILE: &str = include_str!("../data/missy_secrets.json");

fn dynamic_json_parse() {
    // Get the filenames from the command line.
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();

    let mut missy_diet = {
        // Load the first file into a string.
        let text = std::fs::read_to_string(&input_path).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    // Get the number of elements in the object 'missy_food_schedule'
    let nb_elements = missy_diet["missy_food_schedule"].as_array().unwrap().len();

    for index in 0..nb_elements{
        if let Value::Number(n) = &missy_diet["missy_food_schedule"][index]["quantity"] {
            // Double the quantity for each element in 'missy_food_schedule'
            missy_diet["missy_food_schedule"][index]["quantity"] =
                Value::Number(Number::from_f64(n.as_f64().unwrap() * 2.).unwrap());
        }
    }

    // Save the JSON structure into the other file.
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&missy_diet).unwrap(),
    )
    .unwrap();
}

fn static_json_parse() -> Result<(), std::io::Error> {
    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();
    let mut missy_secrets = {
        let missy_secrets = std::fs::read_to_string(&input_path)?;

        // Load the MissyFoodSchedule structure from the string.
        serde_json::from_str::<MissyFoodSchedule>(&missy_secrets).unwrap()
    };

    // Double the quantity for each element in 'missy_food_schedule'
    for index in 0..missy_secrets.missy_food_schedule.len() {
        missy_secrets.missy_food_schedule[index].quantity *= 2.;
    }

    // Save the JSON structure into the output file
    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&missy_secrets).unwrap(),
    )?;

    Ok(())
}

#[test]
fn test_dynamic(){
    dynamic_json_parse();
}

#[test]
fn test_static(){
    static_json_parse();

}
