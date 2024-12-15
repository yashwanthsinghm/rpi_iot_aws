use aws_iot_device_sdk_rust::{async_event_loop_listener, AWSIoTAsyncClient, AWSIoTSettings};
use rumqttc::{self, Packet, QoS};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let aws_settings = AWSIoTSettings::new(
        "Iot-boschecl2".to_owned(),
        "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/AmazonRootCA3.pem".to_owned(),
        "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/38912c2dc782b6f9e7f9e7f6cc7c1b0beb50a875f4fc171d7f4eb47901049d3a-certificate.pem.crt".to_owned(),
        "/home/santosh/rust development/project_with_yashwanth/aws-iot-device-sdk-rust/38912c2dc782b6f9e7f9e7f6cc7c1b0beb50a875f4fc171d7f4eb47901049d3a-private.pem.key".to_owned(),
        "a35lo5fxmqnlto-ats.iot.us-east-1.amazonaws.com".to_owned(),
        None,
    );

    let (iot_core_client, eventloop_stuff) = AWSIoTAsyncClient::new(aws_settings).await?;

    iot_core_client.subscribe("test".to_string(), QoS::AtMostOnce).await.unwrap();
    iot_core_client.publish("topic".to_string(), QoS::AtMostOnce, "hey").await.unwrap();

    let mut receiver1 = iot_core_client.get_receiver().await;
    let mut receiver2 = iot_core_client.get_receiver().await;

    let recv1_thread = tokio::spawn(async move {
        loop {
            if let Ok(event) = receiver1.recv().await {
                match event {
                    Packet::Publish(p) => {
                        println!("Received message {:?} on topic: {}", p.payload, p.topic)
                    }
                    _ => println!("Got event on receiver1: {:?}", event),
                }
            }
        }
    });

    let recv2_thread = tokio::spawn(async move {
        loop {
            if let Ok(event) = receiver2.recv().await {
                println!("Got event on receiver2: {:?}", event);
            }
        }
    });
    let listen_thread = tokio::spawn(async move {
        async_event_loop_listener(eventloop_stuff).await.unwrap();
        //iot_core_client.listen().await.unwrap();
    });

    //iot_core_client.publish("topic".to_string(), QoS::AtMostOnce, "hey").await.unwrap();
    tokio::join!(recv1_thread, recv2_thread, listen_thread);

    Ok(())
}