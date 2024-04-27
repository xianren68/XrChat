use super::*;
#[test]
fn test_tcp_packt() {
    let body: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let tcp_packt = TcpPacket {
        length: body.len() as u32 + 12,
        message_id: 1,
        header_length: 0,
        body_length: body.len() as u32,
        header: Vec::new(),
        body,
    };
    let bytes = tcp_packt.to_bytes();
    let tcp_packt2 = TcpPacket::from_bytes(&bytes).unwrap();
    assert_eq!(tcp_packt2, tcp_packt);
}
