// A macro to try to parse a response from the API.
// we pass the response and the expected type of the response
// if parsing fails, we return the string response wrapped in HttpError

#[allow(unused_macros)]
macro_rules! response_or_error_text_blocking {
    ($response:expr, $expected_type:ty) => {
        // first we store the response in a variable as a string
        {
            let status_code = $response.status();
            let response_text = $response.text().unwrap();
            let response_data = serde_json::from_str::<$expected_type>(&response_text);

            match response_data {
                Ok(response_data) => response_data,
                Err(e) => {
                    println!("{}", e);
                    return Err(errors::HttpError::JsonError {
                        status: status_code.as_u16() as usize,
                        message: response_text.to_string(),
                    });
                }
            }
        }
    };
}

macro_rules! response_or_error_text_async {
    ($response:expr, $expected_type:ty) => {{
        let status_code = $response.status();
        let response_text = $response.text().await.unwrap();
        let response_data = serde_json::from_str::<$expected_type>(&response_text);

        match response_data {
            Ok(response_data) => response_data,
            Err(e) => {
                println!("{}", e);
                return Err(errors::HttpError::JsonError {
                    status: status_code.as_u16() as usize,
                    message: response_text.to_string(),
                });
            }
        }
    }};
}
