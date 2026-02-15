// Enhanced version with better error handling and flexibility

use calamine::{Data, Reader, Xlsx, XlsxError, open_workbook};
use reqwest;
use rppal::gpio::{Gpio, InputPin, Level, Mode, PullUpDown};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, thread};
use tokio::time::{Duration, sleep};

#[derive(Debug, Serialize, Deserialize)]
struct RfidRequest {
    cart_code: String,
    rfid_tag: String,
    action: String,
    quantity: i32,
}

impl RfidRequest {
    fn new(cart_code: &str, rfid_tag: &str, action: &str, quantity: i32) -> Self {
        Self {
            cart_code: cart_code.to_string(),
            rfid_tag: rfid_tag.to_string(),
            action: action.to_string(),
            quantity,
        }
    }
}

// === ASYNC VERSION ===

async fn send_rfid_request_async(
    cart_code: &str,
    rfid_tag: &str,
    action: &str,
    quantity: i32,
) -> Result<String, Box<dyn Error>> {
    let url = "https://smartcaddy.store/api/hardware/rfid";
    let api_key = "demo_test_key_smartkart_2025";

    let request_data = RfidRequest::new(cart_code, rfid_tag, action, quantity);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client
        .post(url)
        .header("X-Cart-API-Key", api_key)
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    if status.is_success() {
        Ok(body)
    } else {
        Err(format!("Request failed with status {}: {}", status, body).into())
    }
}

async fn publish_excel_data(
    mut prev_basket_data: HashMap<String, String>,
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
    let mut pin_green = gpio.get(23).unwrap().into_output();
    pin_green.set_high();
    pin_red.set_high();

    // Read data from the Excel sheet added test code
                        //remove till here
    match workbook.worksheet_range(sheet_name) {
        Ok(range) => {
            println!("Data in '{}':", sheet_name);
            for row in range.rows() {
                println!("line 84 {:?}",row);
                if row.len() >= 2 && !row[1].to_string().contains("Count") {
                    //let username = row[3].to_string();
                 
                    let mut AddorRemove = String::from("None");
                    let mut quantity = 0;
                    let value = row[0].to_string();
                    let count = row[1].to_string();
                    let count = row[1].to_string().parse::<i32>().unwrap();
                    let value1 = row[0].to_string();
                    let count1 = row[1].to_string();
                    let value2 = row[0].to_string();
                    let count2 = row[1].to_string();
                    current_basket_data.insert(value2, count2);
                    if prev_basket_data.contains_key(&value1) {
                        if !prev_basket_data.get(&value1).unwrap().contains(&count1) {
                            quantity = prev_basket_data
                                .get(&value1)
                                .unwrap()
                                .to_string()
                                .parse::<i32>()
                                .unwrap();
                            if quantity > count {
                                quantity = quantity - count;
                                AddorRemove = String::from("REMOVE");
                                 
                            } else {
                                quantity = count - quantity;
                                AddorRemove = String::from("ADD");
                                 
                            }
                            publish = true;
                        }
                    } else {
                        AddorRemove = String::from("ADD");
                        quantity = count;
                        publish = true;
                       
                    }
println!("quantity {}",quantity);
                    if publish {
                        publish = false;
                        pin_green.set_high();
                        prev_basket_data.insert(value1, count1);
                        println!("value {:?}",&value);
println!("line {:?}",&AddorRemove);
                        if AddorRemove.contains("ADD")
                        {
                              pin_green.set_low();
                              println!("green");
                        }
                        else
                        {
                            pin_red.set_low();
                            }
                             
                        match send_rfid_request_async("CART001", &value, &AddorRemove, quantity)
                            .await
                        {
                            Ok(response) => {
                                println!("✓ Success!");
                                println!("Response: {}", response);
                                thread::sleep(Duration::from_millis(100));
                               
                            }
                            Err(e) => {
                           if e.to_string().contains("payment done") {
                                    println!("✓ Payment done, cart is ready for checkout!");
                                    thread::sleep(Duration::from_millis(1000));
                                   std::process::Command::new("shutdown").arg("-h").arg("now").status().unwrap();

                                }
                                else {
                                eprintln!("✗ Error: {}", e);
                                std::process::exit(1);}
                            }
                        }
                    }
                }
            }
            for (ID, COUNT) in prev_basket_data.clone() {
                if current_basket_data.get_key_value(&ID).is_none() {
                    prev_basket_data.remove(&ID);
                    let quantity = COUNT.parse::<i32>().unwrap();
                    pin_red.set_low();
                    match send_rfid_request_async("CART001", &ID, "REMOVE", quantity).await {
                        Ok(response) => {
                            println!("✓ Success!");
                            println!("Response: {}", response);
                            thread::sleep(Duration::from_millis(100));
                           //pin_green.set_low(); 
                        }
                        Err(e) => {
                           if e.to_string().contains("payment done") {
                                    println!("✓ Payment done, cart is ready for checkout!");
                                    thread::sleep(Duration::from_millis(1000));
                                   std::process::Command::new("shutdown").arg("-h").arg("now").status().unwrap();

                                }
                                else {
                                eprintln!("✗ Error: {}", e);
                                std::process::exit(1);}
                        }
                    }
                    thread::sleep(Duration::from_millis(1000));
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

    let mut prev_basket_data = HashMap::new();
    loop {
        
   
        for path in &file_paths {
            match publish_excel_data(prev_basket_data.clone(), path).await {
                Ok(current_basket_data) => {
                    println!("Data from {} published successfully", path);
                    prev_basket_data = current_basket_data;
                }
                Err(e) => println!("Error publishing data from {}: {:?}", path, e),
            }
        }
        sleep(Duration::from_secs(1)).await; // Add a delay to avoid rapid continuous publishing
    }
    Ok(())
}


