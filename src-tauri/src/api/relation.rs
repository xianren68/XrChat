use crate::pb::relation;
use protobuf::Message;
use reqwest::Client;
// pub fn get_friend_list(owner_id: u64) -> Result<Vec<relation::Friend>,Error> {
//     let client = Client::new();
//     let body = relation::GetFriendsRequest::new();
//     body.ownerId = owner_id;
//     let body = body.serialize();

//     let response = client
//         .post("http://127.0.0.1:8080/auth/relation/getFriends")
//         .body(body)
//         .send()?;
//     let res_bytes = response.bytes().unwrap();
//     let relation = relation::GetFriendsRes::new();
//     relation.deserialize(res_bytes)?;
//     Ok(relation.friends())
// }
