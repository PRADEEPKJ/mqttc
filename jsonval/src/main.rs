use json::JsonValue;

fn convert_thinedge_json_to_c8yjson(input: &str) -> JsonValue { 

     let jsonobj = json::parse(input).unwrap();
     println!("thin_edge_obj: \n {:#}",jsonobj);
     let mut c8yobj:JsonValue = JsonValue::new_object();
     c8yobj.insert("type", "ThinEdgeMeasurement").unwrap();
     c8yobj.insert("time", "2013-06-22T17:03:14.000+02:00").unwrap();
     match jsonobj {                                   //First level object
         json::JsonValue::Object(obj) => {
             for (k,v) in obj.iter() {
                 match v {                             //Second Level object
                   JsonValue::Number(num) => {
                      let mut sec_level_obj = JsonValue::new_object();
                      sec_level_obj.insert(k, create_value_obj(*num)).unwrap();
                      c8yobj.insert(k, sec_level_obj).unwrap();
                   },
                   JsonValue::Object(obj) => {
                    let mut third_level_obj:JsonValue = JsonValue::new_object();   
                    for (k,v) in obj.iter() {
                    match v {                          //Third level object
                       JsonValue::Number(num) => {
                           third_level_obj.insert(k, create_value_obj(*num)).unwrap();
                       },
                      _=> println!("Error"),    
                     }
                    }
                    c8yobj.insert(k, third_level_obj).unwrap();
                   },
                   _=> println!("Error"),
                 }
             }
          },
         _=> println!("Error"),
  }
//     println!("c8yobj: \n{:#}",c8yobj);
     c8yobj

}


fn create_value_obj(value: json::number::Number) -> JsonValue {
     let mut valueobj = JsonValue::new_object();
     //println!("value:{}", value);
     valueobj.insert("value", value).unwrap();
     valueobj
}


fn main() {

    let tjson1 = r#"{
                       "temperature": 23,
                        "pressure": 220
                    }"#;


    println!("c8yjson: \n {:#}", convert_thinedge_json_to_c8yjson(tjson1)); 

    let input = r#"{  
                       "temperature": 23,
                "location": { 
                        "latitude": 32.54, 
                                "longitude": -117.67, 
                                        "altitude": 98.6 
                                            }, 
                    "pressure": 98 
    }"#;


    println!("c8yjson: \n {:#}", convert_thinedge_json_to_c8yjson(input)); 

}
