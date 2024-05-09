use crate::pb::relation;
use prost::Message;
use reqwest::blocking::Client;
use std::error::Error;

// get friend list from backend.
pub fn get_friend_list(owner_id: u64) -> Result<Vec<relation::Friend>,Box<dyn Error>> {
    let client = Client::new();
    let friend_req = relation::GetFriendsRequest{
        owner_id: owner_id,
    };
    let mut buf = Vec::new();
    friend_req.encode(& mut buf)?;

    let res = client.post("http://127.0.0.1:8080/auth/relation/getFriends")
    .body(buf)
    .send()?;
    if res.status()!=200 {
        return Err("get friend list failed".into());
    }
    let get_friend_res = relation::GetFriendsRes::decode(res.bytes()?)?;
    if get_friend_res.code!=200 {
        return Err("get friend list failed".into());
    }
    let friend_list = get_friend_res.friends;
    Ok(friend_list)
    
}
