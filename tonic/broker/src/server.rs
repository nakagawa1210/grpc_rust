use tonic::{transport::Server, Request, Response, Status};

pub mod msg_broker {
    tonic::include_proto!("msgbroker");
}

use msg_broker::broker_server::{Broker, BrokerServer};
use msg_broker::{IdData, SendData, RecvData, MsgResponse};

#[derive(Debug, Default)]
pub struct MsgBroker {}

#[tonic::async_trait]
impl Broker for MsgBroker {
    async fn check_id(&self,iddata: Request<IdData>,)
    	  -> Result<Response<MsgResponse>, Status> {
          println!("Got a request: {:?}", iddata);

	  let data = iddata.into_inner();

	  let length = data.length;
	  let command = data.command;
	  let dest = data.dest;
	  let msgid = data.msgid;

        let reply = msg_broker::MsgResponse{
	    length: length,
	    command: command,
	    dest: dest,
	    msgid: msgid,
	    rescode: 7,
        };

        Ok(Response::new(reply)) 
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = MsgBroker::default();

    Server::builder()
        .add_service(BrokerServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}
