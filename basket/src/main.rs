// uart_blocking_read.rs - Blocks while waiting for incoming serial data.

use rppal::uart::{Parity, Uart};

use calamine::{open_workbook_auto, Reader};
use simple_excel_writer::{row, Row, Workbook};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};

const VALID_HEADER: [u8; 5] = [0x3a, 0x50, 0x2, 0x0, 0x0];
const ERROR_HEADER: [u8; 6] = [0x5, 0x50, 0x2, 0x1, 0xfb, 0x81];
const SUB_FRAME_START_BYTE: u8 = 0xc;
const SUB_FRAME_LENGTH: usize = 12;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubFrame {
    data: Vec<u8>,
}

impl SubFrame {
    fn new(data: &[u8]) -> Self {
        SubFrame {
            data: data.to_vec(),
        }
    }
}

/*
fn parse_data(bytes: &[u8]) -> Option<Vec<SubFrame>> {
    println!("{:?}", bytes);
    if bytes.starts_with(&VALID_HEADER) {
        let sub_frame_count = bytes[5] as usize;
        let mut sub_frames = Vec::new();
        let mut i = 6;

        for _ in 0..sub_frame_count {
            if bytes.get(i) == Some(&SUB_FRAME_START_BYTE) {
                if let Some(data) = bytes.get(i + 1..i + 1 + SUB_FRAME_LENGTH) {
                    sub_frames.push(SubFrame::new(data));
                    i += 1 + SUB_FRAME_LENGTH;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(sub_frames)
    } else if bytes.starts_with(&ERROR_HEADER) {
        println!("Error frame received, ignoring.");
        None
    } else {
        println!("Invalid frame format.");
        None
    }
}*/


fn parse_data(bytes: &[u8]) -> Option<Vec<SubFrame>> {
    println!("{:?}", bytes);
    if bytes.starts_with(&ERROR_HEADER) {
    println!("Error frame received, ignoring.");
        None
       
    } else{
  let sub_frame_count = bytes[5] as usize;
        let mut sub_frames = Vec::new();
        let mut i = 6;

        for _ in 0..sub_frame_count {
            if bytes.get(i) == Some(&SUB_FRAME_START_BYTE) {
                if let Some(data) = bytes.get(i + 1..i + 1 + SUB_FRAME_LENGTH) {
                    sub_frames.push(SubFrame::new(data));
                    i += 1 + SUB_FRAME_LENGTH;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(sub_frames)
}
      






   
}
fn load_excel_counts() -> HashMap<SubFrame, u32> {
    let path = "sub_frames.xlsx";
    let mut excel_counts = HashMap::new();

    if let Ok(mut workbook) = open_workbook_auto(path) {
        if let Some(Ok(range)) = workbook.worksheet_range("SubFrame Data") {
            // 1
            for row in range.rows().skip(1) {
                if let (Some(sub_frame_str), Some(count)) = (row.get(0), row.get(1)) {
                    let sub_frame_data = parse_hex_string(sub_frame_str.get_string().unwrap());
                    let sub_frame = SubFrame {
                        data: sub_frame_data,
                    };
                    excel_counts.insert(
                        sub_frame,
                        count.get_string().unwrap().parse::<u32>().unwrap_or(0),
                    );
                }
            }
        }
    }
    excel_counts
}

// Helper function to parse hex string representation into Vec<u8>
fn parse_hex_string(hex_str: &str) -> Vec<u8> {
    hex_str
        .split(|c| c == '[' || c == ']' || c == ',' || c == ' ')
        .filter_map(|s| u8::from_str_radix(s, 16).ok())
        .collect()
}

// Only update Excel file if there is a change in counts
fn update_excel_if_needed(
    local_counts: &HashMap<SubFrame, u32>,
    excel_counts: &HashMap<SubFrame, u32>,
) {
    // Check if counts are different
    if local_counts != excel_counts {
        println!("Changes detected, updating Excel file...");
        let path = "sub_frames.xlsx";
        let mut workbook = Workbook::create(path);

        let mut sheet = workbook.create_sheet("SubFrame Data");
        workbook
            .write_sheet(&mut sheet, |sheet_writer| {
                let sw = sheet_writer;
                sw.append_row(row!["SubFrame", "Count"])?;

                for (sub_frame, count) in local_counts {
                    sw.append_row(row![parse_hex_string2(&format!("{:x?}", sub_frame.data)), count.to_string()])?;
                }
                Ok(())
            })
            .expect("Error writing to Excel file.");

        workbook.close().expect("Error saving Excel file.");
    } else {
        println!("No changes detected. Excel file not updated.");
    }
}

fn receive_packet(data: &mut [u8]) -> (&mut [u8], u8) {
    let mut uart = Uart::new(115200, Parity::None, 8, 1).unwrap();

    // Configure read() to block until at least 1 byte is received.
    let _ = uart.set_read_mode(1, Duration::ZERO); //Duration::default()
    let cmd: [u8; 5] = [0x03, 0x50, 0x02, 0x3E, 0xC5];
    let _ = uart.write(&cmd);
    //println!("mode : {}", uart.is_read_blocking());
    let mut buffer = [0u8; 1];
    // let mut data = [0u8; 200];
    let mut i = 0;
    //let mut i = count.lock().unwrap();
    let res = uart.read(&mut buffer).is_ok();
    let mut len = 0;
    if res {
        len = buffer[0] + 2;
        data[i] = buffer[0];
        //println!("len: {}", len);
        i += 1;
    }
    while i < len as usize {
        let res = uart.read(&mut buffer).is_ok();
         //println!("res : {} {}",res,i);
        // Fill the buffer variable with any incoming data.
        if res {
            //println!("Received byte: {:x?}", buffer);
            data[i] = buffer[0];
            i += 1;
            //println!("i: {}", i);
        }
    }
    //println!("end of while");
    (data, len)
}

fn parse_hex_string2(input: &str) -> String {
    input.trim_matches(|c| c == '[' || c == ']').split(',').map(|s| s.trim()).collect::<Vec<&str>>().join("")
}

fn main() -> ! {
    let mut data = [0u8; 200];
    let mut final_data = [0u8; 200];

    let mut prev_tags = 0;
    let mut checker = 0;
    let mut invalid_packet=0;

    let mut previous_local_counts: HashMap<SubFrame, u32> = HashMap::new();
    println!("before loop");
    loop {
        data = [0u8; 200];
        let (raw_packet, tags) = receive_packet(&mut data);
        println!("Received data byte: {:x?}", raw_packet);
        println!("checker: {},tags :{}", checker,tags);
        if (tags != prev_tags) && (tags > 0x07) {
           // final_data.copy_from_slice(&raw_packet);
            checker = 0;
            invalid_packet = 0;
            prev_tags = tags;
        } 
        else if (tags == prev_tags) && (tags > 0x07){
            checker += 1;
            if (checker > 3){
                final_data.copy_from_slice(&raw_packet);
                checker = 0;
                prev_tags = tags;
                println!("Received final data byte: {:x?}", final_data);
                            }
        }
        else if (tags == 0x7) {
            println!("invalid_packet: {}", invalid_packet);
            invalid_packet += 1;
           // empty_check += 1;
            if (invalid_packet  > 10)  {
                final_data = [0u8; 200];
               invalid_packet = 0;
                
            }
            }
        //println!("Received data byte: {:x?}", raw_packet);
        

        if tags != 0x07{
        if let Some(sub_frames) = parse_data(&final_data) {
            //empty_check = 0;
             
            println!("sub frames {:?}",sub_frames);
            let mut local_counts = HashMap::new();
            
            // Count all subframes in the current data
            for frame in sub_frames {
                *local_counts.entry(frame).or_insert(0) += 1;
            }

            println!("local counts {:?}", local_counts);
            // Load the current Excel counts
            let excel_counts = load_excel_counts();
            println!("excel = {:?},local_counts ={:?},previous_local_counts = {:?}",excel_counts,local_counts,previous_local_counts );

            // Only update if the counts have changed (comparing current and previous counts)
            if local_counts != previous_local_counts {
                update_excel_if_needed(&local_counts, &excel_counts);
                previous_local_counts = local_counts.clone(); // Update the previous counts
            } else {
                println!("Input data has not changed. No update to Excel.");
            }
        }
          }
          else
          {       
              if invalid_packet >= 10 {
                  println!(" inside else invalid_packet: {}", invalid_packet);
                    let mut local_counts = HashMap::new();
                    let excel_counts = load_excel_counts();
                    update_excel_if_needed(&local_counts, &excel_counts);
                    previous_local_counts = local_counts.clone();
                    sleep(Duration::from_millis(10));
                                 }
        }
        
         sleep(Duration::from_millis(100));
         
    }
}

