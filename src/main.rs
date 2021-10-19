use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let sample_request = r#"
{
  "command": "RSHOT",
  "objectType": "eventJSON",
  "siteCode": "ds12", 
  "holeNumber": "hole3",  
  "archiveFilename": "Archive_211001_140321", 
  "archivePath": "./path/to/archive/file/"
}
"#;
    let wresult = stream.write(sample_request.as_bytes());

    match wresult {
        Err(e) => {
            println!("error writing: {}", e);
        }
        _ => {}
    }
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line)?;
        if len > 0 {
            println!("{}", line.trim());
        } else {
            println!("len zero");
            break;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("localhost:8080").unwrap();
    handle_client(stream)?;
    Ok(())
}
