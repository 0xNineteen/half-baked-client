#[macro_export]
macro_rules! convert_to_websocket {
    ($test:expr) => {
        if $test.contains("https") {
            $test.replace("https://", "wss://")
        } else {
            String::from("ws://0.0.0.0:8900")
        }
    };
}
#[macro_export]
macro_rules! block_on {
    ($func:expr,$error:expr) => {
        let rt = Runtime::new().unwrap();
        rt.handle().block_on($func).expect($error);
    };
}
#[macro_export]
macro_rules! try_coerce_shred {
    ($response:expr) => {{
        let shred = if let Some(response) = $response.clone() {
            let shred = match (response.shred_data, response.shred_code) {
                (Some(data_shred), None) => {
                    Some(Shred::ShredData(data_shred))
                    // Some(data_shred. ().expect("num data shreds error"))
                }
                (None, Some(coding_shred)) => Some(Shred::ShredCode(coding_shred)),
                _ => None,
            };
            shred
        } else {
            None
        };
        shred
    }};
}
#[macro_export]
macro_rules! send_rpc_call {
    ($url:expr, $body:expr) => {{
        use reqwest::header::{ACCEPT, CONTENT_TYPE};
        let req_client = reqwest::Client::new();
        let res = req_client
            .post($url)
            .body($body)
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            .expect("error")
            .text()
            .await
            .expect("error");
        res
    }};
}
