pub mod msg_broker {
    tonic::include_proto!("msgbroker");
}

use msg_broker::broker_client::BrokerClient;
use msg_broker::IdData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut recv = BrokerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(IdData {
	length: 1,
	command: 2,
	dest: 3,
	msgid: 4,
	});

    let response = recv.check_id(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}