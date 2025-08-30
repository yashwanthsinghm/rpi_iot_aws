use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
use calamine::{open_workbook, Data, Reader, Xlsx, XlsxError};
use rumqttc::{self, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use tokio::time::{sleep, Duration};
use rppal::gpio::{Gpio, Level, Mode, PullUpDown, InputPin};
use std::thread;
//use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Message {
    value: String,
    count: String,
    status: String,
    cart_id: String,
}

async fn publish_excel_data(
    mut prev_basket_data: HashMap<String, String>,
    iot_core_client: &AWSIoTAsyncClient,
    path: &str,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    // Open the workbook
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut current_basket_data = HashMap::new();
    // Specify the sheet name
    let sheet_name = "SubFrame Data"; // Replace with your sheet name ///home/yashwanth/Desktop/rpi_iot_aws/basket/sub_frames.xlsx
    let mut publish = false;
   
    let gpio = Gpio::new().unwrap();
    let mut pin_red = gpio.get(17).unwrap().into_output();
    let mut pin_green = gpio.get(18).unwrap().into_output();

    // Read data from the Excel sheet
    match workbook.worksheet_range(sheet_name) {
        Ok(range) => {
            println!("Data in '{}':", sheet_name);
            for row in range.rows() {
                if row.len() >= 2 {
                    //let username = row[3].to_string();
                    let count = row[1].to_string();
                    //let sensor_id = row[1].to_string();
                    let cart_id = "cart-1".to_string();
                    let value = row[0].to_string();

                    let value1 = row[0].to_string();
                    let count1 = row[1].to_string();

                    let value2 = row[0].to_string();
                    let count2 = row[1].to_string();
                    current_basket_data.insert(value2, count2);
                    if prev_basket_data.contains_key(&value1) {
                        if !prev_basket_data.get(&value1).unwrap().contains(&count1) {
                            publish = true;
                        }
                    } else {
                        publish = true;
                    }

                    if publish {
                        let mut added = "add".to_string();
                         pin_green.set_low();
                        prev_basket_data.insert(value1, count1);
                        let message = Message {
                            value,
                            count,
                            status: added,
                            cart_id,
                        };
                        let json_message = serde_json::to_string(&message)?;
                       
                        //pin_green.set_mode(Mode::Output);
                        //pin_green.write(Level::High);
                     
                        //let mut pin_green = gpio.get(18)?.into_input_pulldown();
                        // pin_green.set_pullupdown(PullUpDown::PullDown);
                         //  sleep(Duration::from_secs(1));
                        
                        match iot_core_client
                            .publish("basket".to_string(), QoS::AtMostOnce, &*json_message)
                            .await
                        {
                            Ok(_) => println!("Message published successfully"),
                            Err(e) => println!("Error publishing message: {:?}", e),
                        }
                        thread::sleep(Duration::from_millis(1000));
                        pin_green.set_high();
               
                    }
                }
            }
                    for (ID, COUNT) in prev_basket_data.clone() {
                        if current_basket_data.get_key_value(&ID).is_none() {
                            prev_basket_data.remove(&ID);
                            let deleted = "remove".to_string();
                             pin_red.set_low();
                            let cart_id = "cart-1".to_string();
                            let message = Message {
                                value: ID,
                                count: COUNT,
                                status: deleted,
                                cart_id,
                            };
                            let json_message = serde_json::to_string(&message)?;
                            match iot_core_client
                                .publish("basket".to_string(), QoS::AtMostOnce, &*json_message)
                                .await
                            {
                                Ok(_) => println!("Message published successfully"),
                                Err(e) => println!("Error publishing message: {:?}", e),
                            }
                            thread::sleep(Duration::from_millis(1000));
                            pin_red.set_high();


                        }
                    
                
            }

            // Provide some statistics
            let total_cells = range.get_size().0 * range.get_size().1;
            let non_empty_cells: usize = range.used_cells().count();
            println!(
                "Found {} cells in '{}', including {} non-empty cells",
                total_cells, sheet_name, non_empty_cells
            );
        }
        Err(e) => {
            println!("Error reading the workbook: {:?}", e);
            return Err(Box::new(e));
        }
    }

    Ok(prev_basket_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // List of Excel file paths
    let file_paths = vec!["/home/yashwanth/Desktop/test2/rpi_iot_aws/basket/sub_frames.xlsx"];

    let aws_settings = AWSIoTSettings::new(
        "basket".to_owned(),
        "certificate/AmazonRootCA1.pem".to_owned(),
        "certificate/8d2d2aeebf8929b8cefd41847f35f9b6d67005c5a2d793251bf18c41fee23da4-certificate.pem.crt".to_owned(), ///home/yashwanth/projects/rpi_iot_aws/awsiot/certificate/d60e0323f6950461c52cce6502a49cf2f91476675b32e271e2ecbc3e7db73f29-certificate.pem.crt
        "certificate/8d2d2aeebf8929b8cefd41847f35f9b6d67005c5a2d793251bf18c41fee23da4-private.pem.key".to_owned(), ///home/yashwanth/projects/rpi_iot_aws/awsiot/certificate/d60e0323f6950461c52cce6502a49cf2f91476675b32e271e2ecbc3e7db73f29-private.pem.key
        "awxgr2joem983-ats.iot.ap-south-1.amazonaws.com".to_owned(), // awxgr2joem983-ats.iot.eu-north-1.amazonaws.com
        None,//ping awxgr2joem983-ats.iot.ap-south-1.amazonaws.com
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await.unwrap();

    iot_core_client
        .subscribe("basket".to_string(), QoS::AtMostOnce)
        .await
        .unwrap();
   // iot_core_client
     //   .publish("basket".to_string(), QoS::AtMostOnce, "hey")
       // .await
        //.unwrap();

    let mut receiver1 = iot_core_client.get_receiver().await;

    let publish = tokio::spawn(async move {
        let mut prev_basket_data = HashMap::new();
        loop {
            for path in &file_paths {
                match publish_excel_data(prev_basket_data.clone(), &iot_core_client, path).await {
                    Ok(current_basket_data) => {
                        println!("Data from {} published successfully", path);
                        prev_basket_data = current_basket_data;
                    }
                    Err(e) => println!("Error publishing data from {}: {:?}", path, e),
                }
            }
            sleep(Duration::from_secs(1)).await; // Add a delay to avoid rapid continuous publishing
        }
    });

    let recv1_thread = tokio::spawn(async move {
        loop {
            match receiver1.recv().await {
                Ok(event) => match event {
                    Packet::Publish(p) => {
                        println!("Received message {:?} on topic: {}", p.payload, p.topic)
                    }
                    _ => println!("Got event on receiver1: {:?}", event),
                },
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                }
            }
        }
    });

    let listen_thread = tokio::spawn(async move {
        async_event_loop_listener(eventloop_stuff).await.unwrap();
    });

    tokio::join!(recv1_thread, listen_thread, publish);

    Ok(())
}
