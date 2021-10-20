use std::io::prelude::*;
use std::net::TcpStream;

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

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("localhost:8080").unwrap();
    handle_client(stream)?;
    Ok(())
}
