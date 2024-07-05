use dotenv::dotenv;
use image::Luma;
use libsql::Builder;
use qrcode::QrCode;
use serde::Serialize;
use serde_json;
use std::env;
use tokio;
#[derive(Serialize)]
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
    let get_qr = CustomerInfo {
        first_name: "Emir".to_string(),
        last_name: "Day".to_string(),
        purse_id: "1231hafjasd123124123412847324982".to_string(),
    };
    dotenv().ok();
    let url = env::var("TURSO_DATABASE_URL").expect("URL GIR AQ");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TOKEN GIR AMK");

    let db = libsql::Builder::new_remote_replica("local.db", url, token).build().await.unwrap();
    let conn = db.connect().unwrap();


    let qr_get_bytes = CustomerInfoBytes::new(get_qr);

    // Encode some data into bits.
    let code = QrCode::new(qr_get_bytes).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    // image.save("qrcode.png").unwrap();
}
