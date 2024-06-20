use std::{collections::HashMap, net::IpAddr};
use local_ip_address::local_ip;
use dotenv::dotenv;

pub fn return_local_ipaddress() ->  Result<IpAddr,String>{
    let paczkomat_ip = local_ip();
    match paczkomat_ip {
        Ok(ip) => Ok(ip),
        Err(err) => Err(format!("Wystąpił błąd: {:?}", err))
    }
}



pub fn ping() {
    dotenv().ok(); 
    
    let mut data = HashMap::new();
    data.insert("id", std::env::var("uuid").expect("Nie znaleziono uuid w pliku .env."));
    data.insert("ip", return_local_ipaddress().unwrap().to_string());

    let client = reqwest::Client::new();
    let res = client.patch({std::env::var("server_url").expect("Nie znaleziono url servera w pliku .env.")} + "/ip/check/")
    .json(&data)
    .send();
    println!("Response: {:?}", res)
}