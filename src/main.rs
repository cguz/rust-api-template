//! # Reckon API
//!
//! API for requesting data and computing information for the DAO project
//! The API works for both use cases, Space Trafic Management and Spacecraft Resource Management.
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the DAO Data Blockchain API!"
}

// # Space Trafic Management
//
// API for the Space Trafic Management
mod stm {

    use std::fs::File;
    use std::path::Path;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Object {
        name: String,
        owner: String,
        cdm: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CDMShort {
        name: String,
        cdm: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct CatalogSTM{
        objects : Vec<Object>,
        cdm_list : Vec<CDMShort>
    }

    #[derive(Debug)]
    enum CatalogError {
        NotFound,
    }

    fn find_object(json : &CatalogSTM, name_object: &str) -> Result<String, CatalogError> {
        for obj in json.objects.iter(){
            if  obj.name.as_str() == name_object {
                let result = serde_json::to_string(&obj).unwrap();
                return Ok(result);
            }
        }
        Err(CatalogError::NotFound)
    }

    /// retrieve information about the satellite
    #[get("/<name_object>")]
    pub fn satellite(name_object: String) -> String {

        let file = File::open(Path::new("./json/").join("data_stm.json")).expect("file should open read only");
        let json : CatalogSTM = serde_json::from_reader(file).expect("JSON was not well-formatted"); // serde_json::Value

        let object = find_object(&json, &name_object);

        match object {
            Err(CatalogError::NotFound) => serde_json::to_string(&json).unwrap(),
            Ok(obj) => obj,
        }
    }

    /// retrieve possible CDM of a given satellite
    #[get("/cdm/<name_object>")]
    pub fn cdm_satellite(name_object: String) -> String {

        let file = File::open(Path::new("./json/").join("data_stm.json")).expect("file should open read only");
        let json : CatalogSTM = serde_json::from_reader(file).expect("JSON was not well-formatted"); // serde_json::Value

        let object = find_object(&json, &name_object);

        match object {
            Err(CatalogError::NotFound) => serde_json::to_string(&json).unwrap(),
            Ok(obj) => { 
                let json_object : Object = serde_json::from_str(obj.as_str()).expect("JSON was not well-formatted"); 
                serde_json::to_string(&json_object.cdm).unwrap()
            },
        }
    }

    /// retrieve CDM information of two objects
    /// The objects can be either satellites or space debris
    #[get("/cdm/<first_object>/<second_object>")]
    pub fn cdm_two_objects(first_object: String, second_object : String) -> String {
        
        let file = File::open(Path::new("./json/").join("data_stm.json")).expect("file should open read only");
        let json : CatalogSTM = serde_json::from_reader(file).expect("JSON was not well-formatted"); // serde_json::Value

        let object = find_object(&json, &first_object);

        match object {
            Err(CatalogError::NotFound) => serde_json::to_string(&json).unwrap(),
            Ok(obj) => { 
                let json_object : Object = serde_json::from_str(obj.as_str()).expect("JSON was not well-formatted"); 
                for obj in json_object.cdm.iter(){
                    if  obj.as_str() == second_object.as_str() {
                        let str_json_object = serde_json::to_string(&json_object).unwrap();
                        return str_json_object;
                    }
                }
                "not found".to_string()
            },
        }
    }

    /// Retrieve a list of possible CDM
    #[get("/cdm/list")]
    pub fn cdm_list() -> String {
        let file = File::open(Path::new("./json/").join("data_stm.json")).expect("file should open read only");
        let json : CatalogSTM = serde_json::from_reader(file).expect("JSON was not well-formatted"); // serde_json::Value
        
        serde_json::to_string(&json.cdm_list).unwrap()
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).mount("/api/v1/data/stm", routes![stm::satellite, stm::cdm_list, stm::cdm_satellite, stm::cdm_two_objects]).launch();
}
