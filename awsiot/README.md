# Rust AWS IoT Excel Data Publisher

This project demonstrates how to read data from an Excel file and publish it to AWS IoT Core using Rust.

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install)
2. [AWS CLI](https://aws.amazon.com/cli/)
3. AWS IoT Core setup with necessary policies and certificates
4. An Excel file with the required data

## Installation

1. **Clone the Repository**

    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. **Install Dependencies**

    Ensure you have the required dependencies listed in your `Cargo.toml`:

    ```toml
    [dependencies]
    aws-iot-device-sdk-rust = "0.1"
    rumqttc = "0.10"
    serde = { version = "1.0", features = ["derive"] }
    calamine = "0.18"
    tokio = { version = "1", features = ["full"] }
    ```

    Run the following command to install them:

    ```sh
    cargo build
    ```

## Configuration

1. **AWS IoT Core**

    Ensure you have your AWS IoT Core credentials (endpoint, client ID, certificate path, private key path, and root CA path).

2. **Excel File**

    Place your Excel files in the specified paths. Modify the `file_paths` variable in the `main` function with the paths to your Excel files.

## Running the Application

1. **Compile and Run the Application**

    ```sh
    cargo run
    ```

## Project Structure

- `src/main.rs`: Main application code to read Excel data and publish it to AWS IoT.

## Usage

The application reads data from specified Excel files and publishes it to an AWS IoT topic at regular intervals.

1. **Modify the AWS IoT Settings**

    Replace the placeholder values in `aws_settings` with your actual AWS IoT Core credentials.

    ```rust
    let aws_settings = AWSIoTSettings::new(
        "your-endpoint".to_owned(),
        "your-client-id".to_owned(),
        "your-cert-path".to_owned(),
        "your-private-key-path".to_owned(),
        "your-root-ca-path".to_owned(),
        None,
    );
    ```

2. **Modify Excel File Paths**

    Update the `file_paths` variable with the paths to your Excel files.

    ```rust
    let file_paths = vec![
        "path/to/your/excel1.xlsx",
        "path/to/your/excel2.xlsx",
        "path/to/your/excel3.xlsx",
    ];
    ```

## Commands

- **Build the project:**

    ```sh
    cargo build
    ```

- **Run the project:**

    ```sh
    cargo run
    ```

- **Add a new dependency:**

    ```sh
    cargo add <dependency-name>
    ```

- **Update dependencies:**

    ```sh
    cargo update
    ```

## Troubleshooting

- Ensure you have valid AWS IoT Core credentials and your device is registered correctly.
- Verify the paths to your Excel files are correct.
- Check the Rust version compatibility if you encounter compilation issues.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.




### output

```python
Data in 'Sheet1':
Float(123.0)
Found 1 cells in 'Sheet1', including 1 non-empty cells
Message published successfully
Data from /home/ravi/ECL2 Projects/temp/AWS_iot/test1.xlsx published successfully
Data in 'Sheet1':
Float(123.0)
Found 1 cells in 'Sheet1', including 1 non-empty cells
Message published successfully
Data from /home/ravi/ECL2 Projects/temp/AWS_iot/test2.xlsx published successfully
Data in 'Sheet1':
Float(123.0)
Found 1 cells in 'Sheet1', including 1 non-empty cells
Message published successfully
Data from /home/ravi/ECL2 Projects/temp/AWS_iot/test3.xlsx published successfully
Got event on receiver2 : Publish(Topic = test1234, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 25)
Received message b"{\"text\":\"Float(123.0)\\t\"}" on topic: test1234
Got event on receiver2 : Publish(Topic = test1234, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 25)
Received message b"{\"text\":\"Float(123.0)\\t\"}" on topic: test1234
Got event on receiver2 : Publish(Topic = test1234, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 25)
Received message b"{\"text\":\"Float(123.0)\\t\"}" on topic: test1234
Data in 'Sheet1':
Float(123.0)
Found 1 cells in 'Sheet1', including 1 non-empty cells
Message published successfully
Data from /home/ravi/ECL2 Projects/temp/AWS_iot/test1.xlsx published successfully
Data in 'Sheet1':
Float(123.0)
Found 1 cells in 'Sheet1', including 1 non-empty cells
Message published successfully
Data from /home/ravi/ECL2 Projects/temp/AWS_iot/test2.xlsx published successfully
Data in 'Sheet1':
Float(123.0)
Found 1 cells in 'Sheet1', including 1 non-empty cells
Message published successfully
Data from /home/ravi/ECL2 Projects/temp/AWS_iot/test3.xlsx published successfully
Got event on receiver2 : Publish(Topic = test1234, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 25)
Received message b"{\"text\":\"Float(123.0)\\t\"}" on topic: test1234
Got event on receiver2 : Publish(Topic = test1234, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 25)
Received message b"{\"text\":\"Float(123.0)\\t\"}" on topic: test1234
Got event on receiver2 : Publish(Topic = test1234, Qos = AtMostOnce, Retain = false, Pkid = 0, Payload Size = 25)
Received message b"{\"text\":\"Float(123.0)\\t\"}" on topic: test1234

    ```


