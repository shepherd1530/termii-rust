
# Termii Rust

This is a Rust SDK for Termii's messaging api. The

- Send a message
- Send a one time token
- Verify a one time token
- Get your messaging history
- Verify a phone number
- Detects fake or ported numbers
- Request a sender ID and many more termii features


## Example

### Sending a quick message

This crate asynchronous client is the default client enabled by the default feature.

```toml
[dependencies]
termii_rust = { version = "0.1", features = ["default"] }
```

```rust
use  termii_rust::{
    async_impl::rest::termii,
    common::switch::messaging::{Channel, MessageRequest, MessageType},
};

let client =  termii::Termii::new("Your API key");
let _message =  MessageRequest::new(
	"234XXXXXXXXXX".to_string(),
	"FromYourOrg".to_string(),
	"Hello from Rust Termii. 😎".to_string(),
	MessageType::Plain,
	Channel::Dnd,
);

let message = client.switch.messaging.send(_message).await;
println!("{:?}", message);
```

The blocking client is also available, and can be enabled by the `blocking` feature.

```toml
[dependencies]
termii_rust = { version = "0.1", features = ["blocking"] }
```

```rust
use  termii_rust::{
    blocking::rest::termii,
    common::switch::messaging::{Channel, MessageRequest, MessageType},
}

let client =  termii::Termii::new("Your  API key");
let _message =  MessageRequest::new(
	"234XXXXXXXXXX".to_string(),
	"FromYourOrg".to_string(),
	"Hello from Rust Termii. 😎".to_string(),
	MessageType::Plain,
	Channel::Dnd,
);

let message = client.switch.messaging.send(_message).unwrap();
println!("{:?}", message);
```