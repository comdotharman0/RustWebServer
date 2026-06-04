//use std::io::error;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;
use std::net::TcpStream;
#[derive(Debug)]
pub struct Request{
pub method:RequestMethod,
pub url:String,
pub parameters: Option<HashMap<String,String>>,
pub fragements: Option<String>

}
#[derive(Debug)]
pub enum RequestMethod{
Get,
Post
}


impl Request{

pub fn parse<R>(buf_reader: BufReader<R>)
->Result<Request, std::io::Error>
where
R: std::fmt::Debug+ std::io::Read
{
let mut request_to_return = Request{
method: RequestMethod::Get,
url: "/".to_string(),
parameters: None,
fragements: None,
};
let mut  request =  buf_reader.lines()
.next()
.unwrap()?;
let request2 = request
.split_whitespace().take(2).collect::<Vec<_>>();
println!("{:#?}",request2);
match request2[0]{
"GET" => {
let has_params= request2[1].contains("?");
if has_params{
let mut query = request2[1].split("?");
request_to_return.url= query.next().unwrap().to_string();
let param_string=query
.next().unwrap();
let mut  params = HashMap::new();
let params_list = param_string.split("&")
.for_each(|key_value|{
let mut  kv_iter = key_value.split("=");
params.insert(
kv_iter.next().unwrap().to_string(),
kv_iter.next().unwrap().to_string());
});
request_to_return.parameters = Some(params);
//println!("params_list {:#?}",params);
}else{
request_to_return.url = request2[1].to_string();
}
},
_=>{}
}

Ok(request_to_return)
}

}
