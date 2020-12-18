pub mod routeguide {
    tonic::include_proto!("msg");
}

use frame::frame_server::{Frame, FrameServer};
use frame::{IdData, SendData, RecvData, Response};

fn main() {
    println!("Hello, world!");
}
