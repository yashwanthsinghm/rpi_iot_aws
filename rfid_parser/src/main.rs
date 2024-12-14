use std::collections::HashMap;
use std::time::{Duration, Instant};
use simple_excel_writer::{row, Row, Workbook};
use calamine::{open_workbook_auto, Xlsx, Reader, Range};
use std::thread::sleep;
use std::fs::File;
use std::io::BufReader;

const VALID_HEADER: [u8; 5] = [0x3a, 0x50, 0x2, 0x0, 0x0];
const ERROR_HEADER: [u8; 6] = [0x5, 0x50, 0x2, 0x1, 0xfb, 0x81];
const SUB_FRAME_START_BYTE: u8 = 0xc;
const SUB_FRAME_LENGTH: usize = 12;
const CRC_LENGTH: usize = 2;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SubFrame {
    data: Vec<u8>,
}

impl SubFrame {
    fn new(data: &[u8]) -> Self {
        SubFrame { data: data.to_vec() }
    }
}

fn parse_data(bytes: &[u8]) -> Option<Vec<SubFrame>> {
    println!("{:?}",bytes);
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
}

fn load_excel_counts() -> HashMap<SubFrame, u32> {
    let path = "sub_frames.xlsx";
    let mut excel_counts = HashMap::new();

    if let Ok(mut workbook) = open_workbook_auto(path) {
        if let Some(Ok(range)) = workbook.worksheet_range("SubFrame Data") { // 1
            for row in range.rows().skip(1) {
                if let (Some(sub_frame_str), Some(count)) = (row.get(0), row.get(1)) {
                    let sub_frame_data = parse_hex_string(sub_frame_str.get_string().unwrap());
                    let sub_frame = SubFrame { data: sub_frame_data };
                    excel_counts.insert(sub_frame, count.get_string().unwrap().parse::<u32>().unwrap_or(0));
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
fn update_excel_if_needed(local_counts: &HashMap<SubFrame, u32>, excel_counts: &HashMap<SubFrame, u32>) {
    // Check if counts are different
    if local_counts != excel_counts {
        println!("Changes detected, updating Excel file...");
        let path = "sub_frames.xlsx";
        let mut workbook = Workbook::create(path);

        let mut sheet = workbook.create_sheet("SubFrame Data");
        workbook.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row!["SubFrame", "Count"])?;

            for (sub_frame, count) in local_counts {
                sw.append_row(row![
                    format!("{:x?}", sub_frame.data),
                    count.to_string()
                ])?;
            }
            Ok(())
        }).expect("Error writing to Excel file.");

        workbook.close().expect("Error saving Excel file.");
    } else {
        println!("No changes detected. Excel file not updated.");
    }
}

fn main() {
    let mut last_update = Instant::now();
    let mut previous_local_counts: HashMap<SubFrame, u32> = HashMap::new();

    loop {
        let data = receive_data(); // store the received data from uart to data

        if let Some(sub_frames) = parse_data(&data) {
            println!("{:?}",sub_frames);
            let mut local_counts = HashMap::new();
            
            // Count all subframes in the current data
            for frame in sub_frames {
                *local_counts.entry(frame).or_insert(0) += 1;
            }

            println!("{:?}", local_counts);
            // Load the current Excel counts
            let excel_counts = load_excel_counts();

            // Only update if the counts have changed (comparing current and previous counts)
            if local_counts != previous_local_counts {
                update_excel_if_needed(&local_counts, &excel_counts);
                previous_local_counts = local_counts.clone(); // Update the previous counts
            } else {
                println!("Input data has not changed. No update to Excel.");
            }
        }
        
        sleep(Duration::from_secs(1)); // Wait for 1 second
    }
}

// Simulating data reception
fn receive_data() -> Vec<u8> {
    // Simulate a frame with multiple subframes
    vec![
        0x3a, 0x50, 0x2, 0x0, 0x0, 0x04, 
        0xc, 0xe2, 0x0, 0x68, 0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 
        0xc, 0xe2, 0x80, 0x11, 0x70, 0x0, 0x0, 0x2, 0xc, 0x45, 0x6b, 0x7a, 0x2e, 
        0xc, 0xe2, 0x80, 0x11, 0x70, 0x0, 0x0, 0x2, 0xc, 0x45, 0x6b, 0x7a, 0x3e, 
        0xc, 0xe2, 0x0, 0x68, 0xa, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 
        0x90, 0xba
    ]
}
