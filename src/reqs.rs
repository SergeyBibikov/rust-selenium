use std::io::{Read, Write};
use std::net::TcpStream;
//use std::string::String;
use std::error::Error;

pub enum Method {
    GET,
    POST,
    DELETE,
}
pub fn send_request(method: Method, path: &str, headers: Vec<String>, body: &str)->Result<String,Box<dyn Error>> {
    let request = create_req(method, path, headers, body);
    let mut connection = TcpStream::connect("127.0.0.1:4444")?;
    connection.write(request.as_bytes())?;
    connection.flush()?;
    Ok(read_response(&connection)?)
    
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
    for h in headers {
        request.push_str(&h);
        request.push_str("\r\n");
    }
    request.push_str("\r\n\r\n");
    match method {
        Method::POST => request.push_str(body),
        _ => (),
    }
    request
}

fn read_response(mut stream: &TcpStream) -> Result<String,Box<dyn Error>> {
    let bytes_num = stream.peek(&mut vec![0;16384]).unwrap();
    let mut buff = vec![0;bytes_num];
    stream.read(&mut buff)?;
    let response = String::from_utf8(buff)?;

    Ok(response)
}

#[test]
fn delete_method() {
    let del_req = "DELETE /hello/you HTTP/1.1\r
Host: 127.0.0.1\r
Content-Length: 130\r
\r
\r
"
    .to_string();
    //println!("{}",create_req(Method::DELETE,"hello/you",vec!["Content-Length: 130".to_string()],"{dsd}"));
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
fn get_method() {
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