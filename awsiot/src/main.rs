
use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
use rumqttc::{self, Packet, QoS};
use serde::{Deserialize, Serialize};
use std::error::Error;
use calamine::{open_workbook, Data, Reader, Xlsx, XlsxError};
use tokio::time::{sleep, Duration};

#[derive(Serialize, Deserialize)]
pub struct Message {
    value: String,
   // sensor_id: String,
    count: String,
    //username:String,

}

async fn publish_excel_data(iot_core_client: &AWSIoTAsyncClient, path: &str) -> Result<(), Box<dyn Error>> {
    // Open the workbook
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    // Specify the sheet name
    let sheet_name = "Sheet1"; // Replace with your sheet name

    // Read data from the Excel sheet
    match workbook.worksheet_range(sheet_name) {
        Ok(range) => {
            println!("Data in '{}':", sheet_name);
            for row in range.rows() {
                if row.len() >= 2 {
                    //let username = row[3].to_string();
                    let count = row[2].to_string();
                    //let sensor_id = row[1].to_string();
                    let value = row[0].to_string();
                    let message = Message {  value, count };
                    let json_message = serde_json::to_string(&message)?;

                    match iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, &*json_message).await {
                        Ok(_) => println!("Message published successfully"),
                        Err(e) => println!("Error publishing message: {:?}", e),
                    }
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // List of Excel file paths
    let file_paths = vec![
        "/home/iyg1kor/projects/rpi_iot_aws/rfid_parser/sub_frames.xlsx",

    ];

    let aws_settings = AWSIoTSettings::new(
        "test".to_owned(),
        "/home/ravi/ECL2 Projects/project yashwanth/AWS_iot/AmazonRootCA1.pem".to_owned(),
        "/home/ravi/ECL2 Projects/project yashwanth/AWS_iot/4fcec45f0562bdfe59fd9b02e6930d2bc7258358d968dd28afcb4044dd8a1c3d-certificate.pem.crt".to_owned(),
        "/home/ravi/ECL2 Projects/project yashwanth/AWS_iot/4fcec45f0562bdfe59fd9b02e6930d2bc7258358d968dd28afcb4044dd8a1c3d-private.pem.key".to_owned(),
        "a1zpb2m9wn3lmq-ats.iot.us-east-1.amazonaws.com".to_owned(),
        None,
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await.unwrap();

    iot_core_client.subscribe("test1234".to_string(), QoS::AtMostOnce).await.unwrap();
    iot_core_client.publish("test1234".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

    let mut receiver1 = iot_core_client.get_receiver().await;

    let publish = tokio::spawn(async move {
        loop {
            for path in &file_paths {
                match publish_excel_data(&iot_core_client, path).await {
                    Ok(_) => println!("Data from {} published successfully", path),
                    Err(e) => println!("Error publishing data from {}: {:?}", path, e),
                }
            }
            sleep(Duration::from_secs(5)).await; // Add a delay to avoid rapid continuous publishing
        }
    });
    
    let recv1_thread = tokio::spawn(async move {
        loop {
            match receiver1.recv().await {
                Ok(event) => {
                    match event {
                        Packet::Publish(p) => {
                            println!("Received message {:?} on topic: {}", p.payload, p.topic)
                        }
                        _ => println!("Got event on receiver1: {:?}", event),
                    }
                }
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                }
            }
        }
    });

    let listen_thread = tokio::spawn(async move {
        async_event_loop_listener(eventloop_stuff).await.unwrap();
    });

    tokio::join!(
        recv1_thread, 
        listen_thread,
        publish

    );

    Ok(())
}
