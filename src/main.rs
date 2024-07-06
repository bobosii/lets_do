use dotenv::dotenv;
use image::Luma;
use libsql::{de, Builder};
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use tokio;
#[derive(Serialize, Deserialize, Debug)]
struct CustomerInfo {
    first_name: String,
    last_name: String,
    purse_id: String,
}
#[warn(dead_code)]
struct CustomerInfoBytes {
    customer_info: CustomerInfo,
    bytes: Vec<u8>,
}
impl CustomerInfoBytes {
    fn new(customer_info: CustomerInfo) -> Self {
        let bytes = serde_json::to_vec(&customer_info).unwrap();
        Self {
            customer_info,
            bytes,
        }
    }
}

impl AsRef<[u8]> for CustomerInfoBytes {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}
#[tokio::main]
async fn main() {
    //Database proccesses
    dotenv().ok();

    let url = env::var("TURSO_DATABASE_URL").expect("URL GIR AQ");

    let token = env::var("TURSO_AUTH_TOKEN").expect("TOKEN GIR AMK");

    println!("Database url : {}", url);
    println!("Database token : {}", token);

    let db = Builder::new_remote_replica("local.db", url, token)
        .build()
        .await
        .unwrap();

    db.sync().await.unwrap();
    //We have connection
    println!("Connected");

    let conn = db.connect().unwrap();
    let mut statement = conn.prepare("SELECT * FROM customer").await.unwrap();
    //We have the value
    println!("We can get the value");

    let row = statement
        .query([""])
        .await
        .unwrap()
        .next()
        .await
        .unwrap()
        .unwrap();
    let get_customer = de::from_row::<CustomerInfo>(&row).unwrap();
    println!("Customer infos {:?}", get_customer);

    let get_qr = CustomerInfo {
        first_name: get_customer.first_name,
        last_name: get_customer.last_name,
        purse_id: get_customer.purse_id,
    };

    let qr_get_bytes = CustomerInfoBytes::new(get_qr);

    // Encode some data into bits.
    let code = QrCode::new(qr_get_bytes).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    image.save("qrcode.png").unwrap();
}
