use std::io::{Read, Write};
use std::net::TcpStream;
use std::error::Error;

pub (crate) enum Method {
    GET,
    POST,
    DELETE,
}
pub enum LocatorStrategy{
    CSS(&'static str),
    LINKTEXT(&'static str),
    PARTLINKTEXT(&'static str),
    TAGNAME(&'static str),
    XPATH(&'static str)
}
pub(crate) fn send_request(ip:&str,port:&str,method: Method, path: &str, headers: Vec<String>, body: &str)->Result<String,Box<dyn Error>> {
    let request = create_req(method, path, headers, body);
    let address = format!("{}:{}",ip,port);
    let mut connection = TcpStream::connect(address)?;
    connection.write(request.as_bytes())?;
    connection.flush()?;
    let buf = read_response_to_vec_u8(connection).unwrap();
    let st = String::from_utf8(buf).unwrap();
    Ok(st)    
}
/*
pub(crate) fn send_request_remote(ip:&str,method: Method, path: &str, headers: Vec<String>, body: &str)->Result<String,Box<dyn Error>> {
    let request = create_remote_req(ip,method, path, headers, body);
    println!("{}",request);
    let address = format!("{}:4444",ip);
    let mut connection = TcpStream::connect(address)?;
    connection.write(request.as_bytes())?;
    connection.flush()?;
    let buf = read_response_to_vec_u8(connection).unwrap();
    let st = String::from_utf8(buf).unwrap();
    Ok(st)    
}*/

pub(crate) fn send_request_screensh(method: Method, path: &str, headers: Vec<String>, body: &str)->Result<Vec<u8>,Box<dyn Error>> {
    let request = create_req(method, path, headers, body);
    let mut connection = TcpStream::connect("127.0.0.1:4444")?;
    connection.write(request.as_bytes())?;
    connection.flush()?;
    Ok(read_response_screensh(connection)?)
    
}

pub(crate) fn resp_body(response: String)->Result<String,&'static str>{
    let mut a = response.split("\r\n\r\n");
    a.next();
    if let Some(result) = a.next(){
        return Ok(result.to_string())
    } else {Err("Can't get the response body")}

}
pub (crate) fn send_and_read_body(ip:&str,port:&str,method: Method, path: &str, headers: Vec<String>, body: &str)->String{
    resp_body(send_request(ip,port,method, path, headers, body).unwrap()).unwrap()
}

fn create_req(method: Method, path: &str, headers: Vec<String>, body: &str) -> String {
    let mut request = match &method {
        Method::GET => String::from("GET /"),
        Method::POST => String::from("POST /"),
        Method::DELETE => String::from("DELETE /"),
    };
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: 127.0.0.1\r\n");
    if headers.len()>0{
        for h in headers {
            request.push_str(&h);
            request.push_str("\r\n");
        }
    }
    request.push_str("\r\n\r\n");
    match method {
        Method::POST => request.push_str(body),
        _ => (),
    }
    request
}
fn create_remote_req(ip:&str, method: Method, path: &str, headers: Vec<String>, body: &str)->String {
    let mut request = match &method {
        Method::GET => String::from("GET /"),
        Method::POST => String::from("POST /"),
        Method::DELETE => String::from("DELETE /"),
    };
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(ip);
    request.push_str("\r\n");

    if headers.len()>0{
        for h in headers {
            request.push_str(&h);
            request.push_str("\r\n");
        }
    }
    request.push_str("\r\n\r\n");
    match method {
        Method::POST => request.push_str(body),
        _ => (),
    }
    request
}

fn read_response_to_vec_u8(mut stream: TcpStream)->Result<Vec<u8>,Box<dyn Error>>{
    use std::thread;
    use std::sync::mpsc;
    let mut result_buf = vec![];
    let mut temp_buf = vec![];
    let (sender,receiver) = mpsc::channel();
    std::thread::spawn(move || {
        loop{
            let mut b = vec![0;262144];
            let bytes_num = stream.peek(&mut b).unwrap_or(0);
            let mut buff = vec![0;bytes_num];
            let _ = stream.read(&mut buff);
            let _ = sender.send(buff);
        }
    });    
    
    let mut counter = 0;
    let mut started = false;
    loop{
        if counter ==3 {
            break;
        }
        if let Ok(a) = receiver.try_recv(){
            temp_buf.push(a);
            started = true;
        }else if !started {
            continue;
        }else {
            thread::sleep(std::time::Duration::from_millis(10));
            counter+=1;}
    }    
    
    for v in temp_buf{
        for b in v{
            result_buf.push(b);
        }
    }
    Ok(result_buf)
}
fn read_response_screensh(mut stream: TcpStream) -> Result<Vec<u8>,Box<dyn Error>> {
    use std::thread;
    use std::sync::mpsc;
    let mut result_buf = vec![];
    let mut temp_buf = vec![];
    let (sender,receiver) = mpsc::channel();
    std::thread::spawn(move || {
        loop{
            let mut b = vec![0;262144];
            let bytes_num = stream.peek(&mut b).unwrap_or(0);
            let mut buff = vec![0;bytes_num];
            let _ = stream.read(&mut buff);
            let _ = sender.send(buff);
        }
    });    
    
    let mut counter = 0;
    let mut started = false;
    loop{
        if counter ==3 {
            break;
        }
        if let Ok(a) = receiver.try_recv(){
            temp_buf.push(a);
            started = true;
        }else if !started {
            continue;
        }else {
            thread::sleep(std::time::Duration::from_millis(10));
            counter+=1;}
    }    
    
    for v in temp_buf{
        for b in v{
            result_buf.push(b);
        }
    }
    let len = result_buf.len();
    let mut index = 0;
    for i in 0..400{
        if result_buf[i]==b"{"[0]&&
            result_buf[i+1]==b"\""[0]&&
            result_buf[i+2]==b"v"[0]&&
            result_buf[i+3]==b"a"[0]&&
            result_buf[i+4]==b"l"[0]{
            index = i+9; 
            break;
        }
    }
    let mut res = vec![];
    res.extend_from_slice(&result_buf[index+1..len-2]);
    Ok(res)
}
pub (crate) fn cont_length_header(content:&str)->Vec<String>{
    vec![format!("Content-Length:{}",content.len()+2)]
}
pub (crate) fn body_for_find_element(loc_strategy:LocatorStrategy)->String{
    match loc_strategy{
        LocatorStrategy::CSS(selector)=>format!(r#"{{"using":"css selector","value":"{}"}}"#,selector),
        LocatorStrategy::LINKTEXT(selector)=>format!(r#"{{"using":"link text","value":"{}"}}"#,selector),
        LocatorStrategy::PARTLINKTEXT(selector)=>format!(r#"{{"using":"partial link text","value":"{}"}}"#,selector),
        LocatorStrategy::TAGNAME(selector)=>format!(r#"{{"using":"tag name","value":"{}"}}"#,selector),
        LocatorStrategy::XPATH(selector)=>format!(r#"{{"using":"xpath","value":"{}"}}"#,selector)
    }
}
pub (crate) fn parse_value(body: &str)->String{
    let resp = body.replace("\n","").replace(" ","").replace(r#"{"value":"#,"");
    let mut resp_vec: Vec<char> = resp.chars().collect();
    resp_vec.pop();
    let result: String = resp_vec.iter().collect();
    result
}
pub(crate) fn from_str_vec_to_str(vec: Vec<&str>)->String{
    if vec.len() == 0 {return String::from("[]");}
    let mut string_args = String::from("[");
    for st in vec{
        string_args.push('"');
        string_args.push_str(st);
        string_args.push('"');
        string_args.push(',');
    }
    string_args.pop();
    string_args.push(']');
    string_args.push(',');
    string_args
}

//TESTS FOR PRIVATE FUNCTIONS
#[test]
fn resp_body_extraction() {
    let response = "I am the metadata\r\n\r\n{\"hi\":\"there\"}".to_string();
    assert_eq!("{\"hi\":\"there\"}".to_string(),resp_body(response).unwrap());
}
#[test]
fn delete_req_creation() {
    let del_req = "DELETE /hello/you HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 130\r\n\r\n\r\n".to_string();
    assert_eq!(
        del_req,
        create_req(
            Method::DELETE,
            "hello/you",
            vec!["Content-Length: 130".to_string()],
            "{dsd}"
        )
    );
}
#[test]
fn get_req_creation() {
    let del_req = "GET /hello/you HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 130\r\n\r\n\r\n".to_string();
    assert_eq!(
        del_req,
        create_req(
            Method::GET,
            "hello/you",
            vec!["Content-Length: 130".to_string()],
            "{dsd}"
        )
    );
}
#[test]
fn ge_t_status(){
    let response = send_request("127.0.0.1","4444",Method::GET, "wd/hub/status", vec![], "").unwrap();
    assert!(response.contains("Server is running"));
} 
