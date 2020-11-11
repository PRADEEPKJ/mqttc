//#![no_std]

use rumqttc::{MqttOptions,Client,Event, QoS, Incoming};


fn main() {

            let mut mqtt_options = MqttOptions::new("sub-1","localhost", 1883);
            mqtt_options.set_keep_alive(5); 
            let (mut client, mut connection) = Client::new(mqtt_options, 10);
            client.subscribe("hello/world", QoS::ExactlyOnce).unwrap();
        //    println!("Subscriber mode---------------------------");
            
            for (i, notification) in connection.iter().enumerate() {
                if i == 15 {
                    break;
                }

                //println!("Received Notification = {:?}", notification);
                if let Ok(Event::Incoming(Incoming::Publish(publish))) = notification {
          //          println!("Payload = {:?}", publish.payload);
                } 
           }

           //println!("Iterator 1 done!!");
            
           for notification in connection.iter() { 
                println!("Received Notification = {:?}", notification);
                if let Ok(Event::Incoming(Incoming::Publish(publish))) = notification {
            //            println!("Payload = {:?}", publish.payload);
                 }
               }
           client.unsubscribe("hello/world").unwrap();
           client.disconnect().unwrap();
      }
