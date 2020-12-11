use json::JsonValue;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum JsonError {
    InvalidUTF8,
    InvalidJson,
    InvalidThinEdgeJson,
}

pub struct ThinEdgeJson {
    thinedge_json: JsonValue,
}

#[derive(Debug)]
pub struct CumulocityJson {
    c8yjson: JsonValue,
}

impl std::error::Error for JsonError {}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonError::InvalidUTF8 => write!(f, "InvalidUTF8 Error"),
            JsonError::InvalidJson => write!(f, "InvalidJson Error"),
            JsonError::InvalidThinEdgeJson => write!(f, "InvalidThinEdgeJson Error"),
        }
    }
}

impl ThinEdgeJson {
    pub fn from_sting(input: &str) -> Result<ThinEdgeJson, JsonError> {
        match json::parse(input) {
            Ok(obj) => return Ok(ThinEdgeJson { thinedge_json: obj }),
            Err(InvalidJson) => {
                eprintln!("Error while creating the JsonValue");
                return Err(JsonError::InvalidJson);
            }
        };
    }

    pub fn from_json(input: json::JsonValue) -> ThinEdgeJson {
        ThinEdgeJson {
            thinedge_json: input,
        }
        /*
        match input {
           json::JsonValue::Object(obj) => {
               Ok(input) => return Ok(ThinEdgeJson { thinedge_json: input}),
               Err(InvalidJson) => {
                   eprintln!("Not a valid json");
                   return Err(JsonError::InvalidJson);
               },
           },
             _=>eprintln!("Error"),
        };
        */
    }


           
    pub fn into_cumulocity_json(&self, timestamp: &str) -> CumulocityJson {
        println!("thin_edge_obj: \n {:#}", self.thinedge_json);
        let mut c8yobj = creat_c8yjson_object(timestamp, "ThinEdgeJsonMessage");
        match self.thinedge_json.clone() {
            //First level object
            json::JsonValue::Object(obj) => {
                for (k, v) in obj.iter() {
                    match v {
                        //Second Level object
                        JsonValue::Number(num) => {
                            let mut sec_level_obj = JsonValue::new_object();
                            sec_level_obj.insert(k, create_value_obj(*num)).unwrap();
                            insert_into_c8yjson(k, sec_level_obj, &mut c8yobj);
                        }
                        JsonValue::Object(obj) => {
                            /*
                            translate_complex_object(obj);
                            let mut third_level_obj: JsonValue = JsonValue::new_object();
                            for (k, v) in obj.iter() {
                                match v {
                                    //Third level object
                                    JsonValue::Number(num) => {
                                        third_level_obj.insert(k, create_value_obj(*num)).unwrap();
                                    }
                                    _ => println!("Error"),
                                }
                            }*/
                            insert_into_c8yjson(k, translate_complex_object(obj), &mut c8yobj);
                        }
                        _ => println!("Error"),
                    }
                }
            }
            _ => println!("Error"),
        }
        //     println!("c8yobj: \n{:#}",c8yobj);
        c8yobj
    }
}

fn translate_complex_object(obj: &json::object::Object) -> JsonValue {
    let mut complex_obj: JsonValue = JsonValue::new_object();
    for (k, v) in obj.iter() {
       match v {
            JsonValue::Number(num) => {
                  complex_obj.insert(k, create_value_obj(*num)).unwrap();
            }
            _ => println!("Error"),
           }
    } 
    complex_obj
} 


fn insert_into_c8yjson(key: &str, jsonobj: JsonValue, c8yobj: &mut CumulocityJson) {
        if ! key.is_empty() &&  !jsonobj.is_null()  {
            match c8yobj.c8yjson.insert(key, jsonobj) {
                Ok(_obj) => println!("Inserted successfully"),
                Err(_e) => eprintln!("Failed to insert the json object into c8yjson"), 
            }
      } else {
          eprintln!("The key or jsonobj or is empty");
        }
}


/*
fn convert_thinedge_json_to_c8yjson(input: &str) -> JsonValue {
    let jsonobj = json::parse(input).unwrap();
    println!("thin_edge_obj: \n {:#}", jsonobj);
    let mut c8yobj: JsonValue = JsonValue::new_object();
    c8yobj.insert("type", "ThinEdgeMeasurement").unwrap();
    c8yobj
        .insert("time", "2020-06-22T17:03:14.000+02:00")
        .unwrap();
    match jsonobj {
        //First level object
        json::JsonValue::Object(obj) => {
            for (k, v) in obj.iter() {
                match v {
                    //Second Level object
                    JsonValue::Number(num) => {
                        let mut sec_level_obj = JsonValue::new_object();
                        sec_level_obj.insert(k, create_value_obj(*num)).unwrap();
                        c8yobj.insert(k, sec_level_obj).unwrap();
                    }
                    JsonValue::Object(obj) => {
                        let mut third_level_obj: JsonValue = JsonValue::new_object();
                        for (k, v) in obj.iter() {
                            match v {
                                //Third level object
                                JsonValue::Number(num) => {
                                    third_level_obj.insert(k, create_value_obj(*num)).unwrap();
                                }
                                _ => println!("Error"),
                            }
                        }
                        c8yobj.insert(k, third_level_obj).unwrap();
                    }
                    _ => println!("Error"),
                }
            }
        }
        _ => println!("Error"),
    }
    //     println!("c8yobj: \n{:#}",c8yobj);
    c8yobj
}
*/

fn create_value_obj(value: json::number::Number) -> JsonValue {
    let mut valueobj = JsonValue::new_object();
    //println!("value:{}", value);
    let num: f64 = value.into();
    if num == 0.0 || num.is_normal() {
        valueobj.insert("value", value).unwrap();
        valueobj
    } else {
        JsonValue::Null
    }
}

fn creat_c8yjson_object(timestamp: &str, c8y_msg_name: &str) -> CumulocityJson {
    let mut c8yobj: JsonValue = JsonValue::new_object();
    c8yobj.insert("type", c8y_msg_name).unwrap();
    c8yobj.insert("time", timestamp).unwrap();
    CumulocityJson { c8yjson: c8yobj }
}


impl fmt::Display for CumulocityJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#}", self.c8yjson)
    }
}

fn main() {
    let tjson1 = r#"{
                       "temperature": 23,
                        "pressure": 220
                    }"#;

    let time = "2020-06-22T17:03:14.000+02:00";

    println!(
        "c8yjson: \n {}",
        ThinEdgeJson::from_sting(tjson1)
            .unwrap()
            .into_cumulocity_json(time)
    );
    //        convert_thinedge_json_to_c8yjson(tjson1));

    let input = r#"{  
                       "temperature": 0 ,
                "location": { 
                        "latitude": 32.54, 
                                "longitude": -117.67, 
                                        "altitude": 98.6 
                                            }, 
                    "pressure": 98 
    }"#;

    println!(
        "c8yjson: \n {}",
        ThinEdgeJson::from_sting(input)
            .unwrap()
            .into_cumulocity_json(time) //ThinEdgeJson::from_sting(input)?.into_cumulocity_json(time)
    );
    //   println!("c8yjson: \n {:#}", convert_thinedge_json_to_c8yjson(input));
}
