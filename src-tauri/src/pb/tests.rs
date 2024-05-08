

use prost::Message;

use super::*;

#[test]
fn test_pb() { 
    let friend = relation::Friend{
        id: 1,
        name: "test".to_string(),
        remark: "testre".to_string(),
        avatar: "testavatar".to_string(),
        email: "testemail".to_string(),
        phone: "testphone".to_string(),
        gender: true,
        line: "testline".to_string()
    };
    let mut buf = Vec::new();
    let _ = friend.encode(&mut buf);
    println!("{:?}", buf);
    
    let friend2 = relation::Friend::decode(&buf[..]).unwrap();
    println!("{:?}", friend2);
}