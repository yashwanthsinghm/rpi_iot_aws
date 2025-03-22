import json
import psycopg2
import paho.mqtt.client as mqtt

# RDS PostgreSQL connection parameters
host = "database.cfsg8aggwxm9.us-east-1.rds.amazonaws.com"
username = "postgres"
password = "Sbikrc21916"
database = "postgres"

def on_connect(client, userdata, flags, rc):
    print("Connected with result code " + str(rc))
    client.subscribe("basket")

def on_message(client, userdata, msg):
    print(msg.topic + " " + str(msg.payload))

    message = msg.payload.decode()  # Decode bytes to string
    print(f"Received message: {message}")

    iot_message = json.loads(msg.payload)
    print(type(iot_message))
    
    #name = iot_message.get('name')
    count = iot_message.get('count')
    sensor_id = iot_message.get('value')
    #value = iot_message.get('value')
    
    if sensor_id is None or count is None:
        print("Missing required fields in the IoT message.")
        return
    
    try:
        sensor_id = int(sensor_id)  # Convert sensor_id to integer
        count = int(count)  # Convert count to integer
    except ValueError:
        print("Invalid sensor_id or count format. They must be integers.")
        return

    try:
        # Connect to the PostgreSQL database using fixed credentials
        conn = psycopg2.connect(
            host=host,
            database=database,
            user=username,
            password=password
        )
        cur = conn.cursor()
        
        insert_query = """
        INSERT INTO sensor (sensor_id, count) 
        VALUES (%s, %s)
        """
        cur.execute(insert_query, (sensor_id, count))

        conn.commit()
        cur.close()
        conn.close()
        print("Data inserted successfully.")
    except Exception as e:
        print(f"Failed to connect to the database: {e}")

client = mqtt.Client()
client.on_connect = on_connect
client.on_message = on_message

# Set the path to your CA certificate, client certificate, and private key
ca_path = "pythonec2/certificate/AmazonRootCA1.pem"
cert_path = "pythonec2/certificate/8d2d2aeebf8929b8cefd41847f35f9b6d67005c5a2d793251bf18c41fee23da4-certificate.pem.crt"
key_path = "pythonec2/certificate/8d2d2aeebf8929b8cefd41847f35f9b6d67005c5a2d793251bf18c41fee23da4-private.pem.key"

client.tls_set(ca_certs=ca_path, certfile=cert_path, keyfile=key_path)

# Replace 'your-iot-endpoint.amazonaws.com' with your actual AWS IoT endpoint
client.connect("awxgr2joem983-ats.iot.ap-south-1.amazonaws.com", 8883, 60)
client.loop_forever()