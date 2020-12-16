use json::JsonValue;
use std::fmt;
use std::error;

#[derive(Debug, Eq, PartialEq)]
pub enum JsonError {
    InvalidUTF8(std::str::Utf8Error),
    InvalidJson(json::Error),
    InvalidThinEdgeJson,
}

pub struct ThinEdgeJson {
    values: Vec<ThinEdgeValue>,
}

enum ThinEdgeValue {
    Single(SingleMeasurement),
    Multi(MultiMeasurement),
}

pub struct SingleMeasurement {
    name: String,
    value: json::number::Number,
}

struct MultiMeasurement {
    name: String,
    values: Vec<SingleMeasurement>,
}

#[derive(Debug)]
pub struct CumulocityJson {
    c8yjson: JsonValue,
}

impl error::Error for JsonError {
   fn source(&self) -> Option<&(dyn error::Error + 'static)> {
           match *self {
                  JsonError::InvalidJson(ref e) => Some(e),
                  JsonError::InvalidUTF8(ref e) => Some(e),
                  JsonError::InvalidThinEdgeJson => None,
           }
   }
}

impl From<std::str::Utf8Error> for JsonError {
    fn from(error: std::str::Utf8Error) -> Self {
        JsonError::InvalidUTF8(error)
    }
}

impl From<json::Error> for JsonError {
    fn from(error: json::Error) -> Self {
        JsonError::InvalidJson(error)
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonError::InvalidUTF8(..) => write!(f, "InvalidUTF8 Error"),
            JsonError::InvalidJson(..) => write!(f, "InvalidJson Error"),
            JsonError::InvalidThinEdgeJson => write!(f, "InvalidThinEdgeJson Error"),
        }
    }
}

impl ThinEdgeJson {

    ///From array of bytes->to str->convert then to json
    pub fn from_sting(input: &Vec<u8>) -> Result<ThinEdgeJson, JsonError> {
        let json_string = std::str::from_utf8(input)?;
        match json::parse(&json_string) {
            //Check the object for the thin -edge json template 2 level
            Ok(obj) => ThinEdgeJson::from_json(obj),
            Err(err) => {
                eprintln!("Error while creating the JsonValue");
                Err(JsonError::InvalidJson(err))
            }
        }
    }

    ///Confirms that the json is in thin-edge json format or not
    pub fn from_json(input: json::JsonValue) -> Result<ThinEdgeJson, JsonError> {
        let mut measurements: Vec<ThinEdgeValue> = Vec::new();
        match input.clone() {
            json::JsonValue::Object(obj) => {
                for (k, v) in obj.iter() {
                    match v {
                        JsonValue::Number(num) => {
                            //Single Value object
                            create_single_val_thinedge_struct(&mut measurements, k, *num);
                        }
                        JsonValue::Object(obj) => {
                            //Multi value object
                            create_multi_val_thinedge_struct(&mut measurements, obj.clone(), k);
                        }
                        _ => {
                            eprintln!(" Error: Invalid thin edge json ");
                            return Err(JsonError::InvalidThinEdgeJson);
                        }
                    }
                }
            }
            _ => {
                eprintln!("Error: Not a multi-value object");
                return Err(JsonError::InvalidThinEdgeJson);
            }
        };
        Ok(ThinEdgeJson {
            values: measurements,
        })
    }

    ///Convert from thinedgejson to c8yjson
    pub fn into_cumulocity_json(&self, timestamp: &str) -> CumulocityJson {
        let mut c8yobj = create_c8yjson_object(timestamp, "ThinEdgeJsonMessage");

        for v in self.values.iter() {
            match v {
                ThinEdgeValue::Single(measurement) => {
                    translate_single_value_object(measurement, &mut c8yobj.c8yjson);
                }
                ThinEdgeValue::Multi(measurement) => {
                    translate_multivalue_object(measurement, &mut c8yobj.c8yjson);
                }
            }
        }
         c8yobj
    }
}

fn create_single_val_thinedge_struct(
    tedge_measurements: &mut Vec<ThinEdgeValue>,
    name: &str,
    value: json::number::Number,
) {
    let single_value = SingleMeasurement {
        name: String::from(name),
        value,
    };
    tedge_measurements.push(ThinEdgeValue::Single(single_value));
}

fn create_multi_val_thinedge_struct(
    tedge_measurements: &mut Vec<ThinEdgeValue>,
    obj: json::object::Object,
    name: &str,
) {
    let mut single_value: Vec<SingleMeasurement> = Vec::new();

    for (k, v) in obj.iter() {
        match v {
            JsonValue::Number(num) => {
                //Single Value object
                single_value.push(SingleMeasurement {
                    name: String::from(k),
                    value: *num,
                });
            }
            _ => eprintln!(
                "Failed to translate, value is not a number, related to name {}",
                k
            ),
        }
    }
    let multi_measurement = MultiMeasurement {
        name: String::from(name),
        values: single_value,
    };

    tedge_measurements.push(ThinEdgeValue::Multi(multi_measurement));
}

fn translate_multivalue_object(multi: &MultiMeasurement, c8yjsonobj: &mut JsonValue) {
    let mut complex_obj: JsonValue = JsonValue::new_object();
    for s in multi.values.iter() {
        create_value_obj_and_insert_into_jsonobj(&s.name, s.value, &mut complex_obj);
        complex_obj
            .insert(&s.name, create_value_obj(json::from(s.value)))
            .unwrap();
    }
    c8yjsonobj.insert(&multi.name, complex_obj).unwrap();
}

fn translate_single_value_object(single: &SingleMeasurement, c8yobj: &mut JsonValue) {
    if !single.value.is_empty() {
        let mut single_value_object: JsonValue = JsonValue::new_object();
        single_value_object
            .insert(&single.name, create_value_obj(json::from(single.value)))
            .unwrap();
        match c8yobj.insert(&single.name, single_value_object) {
            Ok(_obj) => _obj,
            Err(_e) => eprintln!("Failed to insert the json object into c8yjson"),
        }
    } else {
        eprintln!("The key or jsonobj or is empty");
    }
}

fn create_value_obj_and_insert_into_jsonobj(
    key: &str,
    num: json::number::Number,
    jsonobj: &mut JsonValue,
) {
    match jsonobj.insert(key, create_value_obj(json::from(num))) {
        Ok(_obj) => _obj,
        Err(_e) => eprintln!("Failed to insert the json object"),
    }
}

fn create_value_obj(value: JsonValue) -> JsonValue {
    let mut valueobj = JsonValue::new_object();
    valueobj.insert("value", value).unwrap();
    valueobj
}

fn create_c8yjson_object(timestamp: &str, c8y_msg_type: &str) -> CumulocityJson {
    let mut c8yobj: JsonValue = JsonValue::new_object();
    c8yobj.insert("type", c8y_msg_type).unwrap();
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

    println!("Tedge_Json: {:#}", tjson1);

    println!(
        "c8yjson: \n {}",
        ThinEdgeJson::from_sting(&String::from(tjson1).into_bytes())
            .unwrap()
            .into_cumulocity_json(time)
    );


    let input = r#"{  
                "temperature": 23 ,
                "location": { 
                      "latitude": 32.54, 
                      "longitude": -117.67, 
                      "altitude": 98.6 
                  }, 
                "pressure": 98 
    }"#;

    println!("Tedge_Json: {:#}", input);
    println!(
        "c8yjson: \n {}",
        ThinEdgeJson::from_sting(&String::from(input).into_bytes())
            .unwrap()
            .into_cumulocity_json(time) //ThinEdgeJson::from_sting(input)?.into_cumulocity_json(time)
    );
}
