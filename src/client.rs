extern crate failure;
extern crate chrono;
extern crate mime_guess;
extern crate protobuf;
extern crate futures;
extern crate grpcio;
extern crate clap;

mod protos;

use std::sync::Arc;
use std::path::Path;
use std::io::{BufReader, Read};
use std::fs;

use futures::{Future, Sink};

use grpcio::{ChannelBuilder, EnvBuilder, Error as GrpcError, WriteFlags};

use protos::resource_with_file::{RegisterRequest, Resource, Attachment, FileHeader, Chunk};
use protos::resource_with_file_grpc::RegisterServiceClient;

fn main() {
    let args = clap::App::new("client")
        .version("0.1")
        .about("CLI for client")
        .arg(clap::Arg::with_name("file")
             .short("f")
             .long("file")
             .value_name("FILE")
             .required(true)
             .help("Sets a upload file")
             .takes_value(true))
        .get_matches();;

    let file_path = Path::new(args.value_of("file").unwrap());
    let file_mime = mime_guess::guess_mime_type(file_path);

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("127.0.0.1:8000");
    let client = RegisterServiceClient::new(ch);

    let mut requests = Vec::new();

    // stream resource data
    {
        let mut resource = Resource::new();
        resource.set_title("test title".to_string());
        let mut req = RegisterRequest::new();
        req.set_resource(resource);
        requests.push((req, WriteFlags::default()));
    }

    // stream file header
    {
        let mut attachment = Attachment::new();
        let mut file_header = FileHeader::new();
        file_header.set_name(file_path
                             .file_name().unwrap()
                             .to_str().unwrap()
                             .to_string());
        file_header.set_mime(file_mime.to_string());
        attachment.set_header(file_header);
        let mut req = RegisterRequest::new();
        req.set_attachment(attachment);
        requests.push((req, WriteFlags::default()));
    }

    // stream file data
    let mut f = BufReader::new(fs::File::open(file_path).unwrap());
    let mut buf = [0u8; 32*1024];
    while let Ok(n) = f.read(&mut buf) {
        if n == 0 {
            break;
        }
        let mut attachment = Attachment::new();
        let mut chunk = Chunk::new();
        chunk.set_data(buf.to_vec());
        attachment.set_chunk(chunk);
        let mut req = RegisterRequest::new();
        req.set_attachment(attachment);
        requests.push((req, WriteFlags::default()));
    }

    let (sender, receiver) = client.register().unwrap();

    sender.send_all(futures::stream::iter_ok::<_, GrpcError>(requests))
        .wait()
        .unwrap();

    let resp = receiver.wait().unwrap();

    assert_eq!(true, resp.get_result());
    println!("pass");
}
