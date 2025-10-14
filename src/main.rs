use std::fmt::format;
use std::io::{Read, Write};
use std::net::TcpStream;
use native_tls::TlsConnector;

fn main() -> std::io::Result<()> {




    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Fucked up reading");



    let mut stream = TcpStream::connect("10.32.136.154:80")?;

    let mut bytes = [0;128];

    let mut request = "GET /people/dem HTTP/1.1\nHost: RustRequest\nAccept: pngidk";


    stream.write(request.as_bytes())?;
    stream.read(&mut bytes)?;

    let mut page = String::from_utf8_lossy(&bytes);

    println!("{}", page.lines().skip(3).collect::<Vec<&str>>().join("\n"));

    Ok(())
} // the stream

