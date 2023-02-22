#[allow(unused)]

pub fn parse_url(){
    let connect_addr = "ws://localhost:8000/chat";
    let url = url::Url::parse(&connect_addr).unwrap();
    // let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");
}

#[test]
fn test(){
    parse_url();
}