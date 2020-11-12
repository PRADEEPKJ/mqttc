use rumqttc::{MqttOptions,QoS,Client};
use std::{thread, time::Duration};
use clap::{Arg, App};
use std::io;

fn main() {
       let matches = App::new("MQTT Client")
                     .arg(Arg::with_name("host")
                              .short("h")
                              .long("host/broker")
                              .takes_value(true)
                              .help("Provide the ip with port"))
                      .arg(Arg::with_name("topic")
                              .short("t")
                              .long("topicname")
                              .takes_value(true)
                              .help("Provide the topic name to publish/subscribe"))
                       .arg(Arg::with_name("message")
                              .short("m")
                              .long("message")
                              .takes_value(true)
                              .help("Provide the message to be published/subscribed"))
                        .arg(Arg::with_name("frequency")
                              .short("f")
                              .long("frequency")
                              .takes_value(true)
                              .help("Provide the number of times  message to be published"))
                        .get_matches();

        let host = matches.value_of("host");
        let topic = matches.value_of("topic");
        let message = matches.value_of("message");
        let frequency = matches.value_of("frequency");

        let mut broker  =  String::from("localhost");
        let mut mtopic  = String::from(" ");
        let mut msg  =  String::from("");
        let mut freq = String::from("1");

        match host  {
            None=> println!("Provide host name"),
            Some(s) => {
                match s.parse::<String>(){
                    Ok(c) => {
                        println!("Host name {}", c);
                        broker = c;
                    },
                    Err(_) => println!("No broker ip specified"),
                }
            }
        }

        match topic  {
            None=> println!("Provide topic"),
            Some(s) => {
                match s.parse::<String>(){
                    Ok(c) => {
                        println!("Topic name {}", c);
                        mtopic = c;
                    },
                    Err(_) => println!("No topic specified"),
                }
            }
        }

         match message  {
                    None=> println!("Provide message to be sent"),
                    Some(s) => {
                        match s.parse::<String>(){
                            Ok(c) => {
                                 println!("Message to publish {}", c);
                                 msg = c;
                            },
                            Err(_) => println!("No message specified"),
                        }
                  }
         }

         match frequency  {
                    None=> println!("Provide mumber of times messages to be sent"),
                    Some(s) => {
                        match s.parse::<String>(){
                            Ok(c) => {
                                 println!("Number of times Message to publish {}", c);
                                 freq  = c;
                            },
                            Err(_) => println!("No frequency specified, uses defautl value"),
                        }
                  }
         }

          
 
    let mut mqttoptions = MqttOptions::new("pub", broker, 1883);
    mqttoptions.set_keep_alive(10);

    let (mut client, mut connection) = Client::new(mqttoptions, 1);
    thread::spawn(move || publish(client, msg, mtopic, freq));

            for __notification in connection.iter().enumerate() {
               //  if i == 1 {
               //  println!
               //      break;
                // }
             } 
    
}

fn publish(mut client: Client,  msg: String,  topic: String, freq: String) {
       let n = freq.parse::<i32>().unwrap();
        for i in 0..10 {
            //let payload = format!("Hello Rustadfafaf {:?} ", i);
            //println!("payload published: {:?}",msg.clone());
            let topic = format!("hello/world");
            let message = get_input("Provide message to be sent$");
            let qos = QoS::AtMostOnce;
             
            thread::sleep(Duration::from_secs(1));
            client.publish(topic.clone(), qos, true, message).unwrap();
        }
         thread::sleep(Duration::from_secs(1));
}

 fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
              Ok(_goes_into_input_above) => {},
              Err(_no_updates_is_fine) => {},
        }
        input.trim().to_string()
 }     
