use std::io::{Read, Write};
use std::net::TcpStream;
use std::error::Error;

pub (crate) enum Method {
    GET,
    POST,
    DELETE,
}
pub(crate) fn send_request(method: Method, path: &str, headers: Vec<String>, body: &str)->Result<String,Box<dyn Error>> {
    let request = create_req(method, path, headers, body);
    let mut connection = TcpStream::connect("127.0.0.1:4444")?;
    connection.write(request.as_bytes())?;
    connection.flush()?;
    Ok(read_response(&connection)?)
    
}
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

fn read_response(mut stream: &TcpStream) -> Result<String,Box<dyn Error>> {
    //let bytes_num = stream.peek(&mut vec![0;32768]).unwrap();
    let bytes_num = stream.peek(&mut vec![0;2097152]).unwrap();
    let mut buff = vec![0;bytes_num];
    stream.read(&mut buff)?;
    let response = String::from_utf8(buff)?;

    Ok(response)
}
fn read_response_screensh(mut stream: TcpStream) -> Result<Vec<u8>,Box<dyn Error>> {
    use std::thread;
    use std::sync::mpsc;
    let mut result_buf = vec![];
    let mut temp_buf = vec![];
    let (sender,receiver) = mpsc::channel();
    std::thread::spawn(move || {
        loop{
            let bytes_num = stream.peek(&mut vec![0;4096]).unwrap();
            let mut buff = vec![0;bytes_num];
            stream.read(&mut buff).unwrap();
            sender.send(buff);
        }
    });
    thread::sleep(std::time::Duration::from_millis(100));
    let mut it = receiver.try_iter();
    while let Some(n) = it.next(){
        temp_buf.push(n);
    }
    /*for a in receiver.try_recv(){
        temp_buf.push(a);
    }*/
    for v in temp_buf{
        for b in v{
            result_buf.push(b);
        }
    }
    //let response = String::from_utf8(buff)?;
    for i in 0..result_buf.len(){
        if result_buf[i]==b"\r"[0]&&result_buf[i+1]==b"\n"[0]&&result_buf[i+2]==b"\r"[0]&&result_buf[i+3]==b"\n"[0]{
            println!("The position is - {}",i);
        }
    };
    Ok(result_buf)
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
