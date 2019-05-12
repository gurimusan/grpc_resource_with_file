#[macro_use] extern crate log;
extern crate failure;
extern crate chrono;
extern crate protobuf;
extern crate futures;
extern crate grpcio;

mod error;
mod protos;

use std::io::Read;
use std::sync::Arc;
use std::io;
use std::thread;

use futures::sync::oneshot;
use futures::{Future, Stream};
use grpcio::{Environment, ServerBuilder, RpcContext, RequestStream,
    ClientStreamingSink, Result as GrpcResult, Error as GrpcError};

use protos::resource_with_file::{RegisterRequest, RegisterResponse};
use protos::resource_with_file_grpc::{self, RegisterService};

#[derive(Clone)]
struct Server;


struct Register {
    pub count: u8,
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub file_mime: Option<String>,
    pub file_content: Vec<u8>,
}

impl Default for Register {
    fn default() -> Self {
        Self {
            count: 0 as u8,
            title: None,
            file_name: None,
            file_mime: None,
            file_content: vec!(),
        }
    }
}

impl RegisterService for Server {
    fn register(
        &mut self,
        ctx: RpcContext,
        stream: RequestStream<RegisterRequest>,
        sink: ClientStreamingSink<RegisterResponse>,
    ) {
        let r: Register = Default::default();

        let f = stream
            .fold(r, |mut r, req| {
                debug!("{:?}", req);
                if req.has_resource() {
                    r.title = Some(req.get_resource().get_title().to_string());
                }
                if req.has_attachment() {
                    let att = req.get_attachment();
                    if att.has_header() {
                        r.file_name = Some(att.get_header().get_name().to_string());
                        r.file_mime = Some(att.get_header().get_mime().to_string());
                    }
                    if att.has_chunk() {
                        r.file_content.extend(att.get_chunk().get_data().iter().cloned());
                    }
                }
                r.count = r.count + 1;
                Ok(r) as GrpcResult<_>
            })
            .and_then(|r| {
                println!("call stream count={}", r.count);
                println!("title={:?}", r.title);
                println!("file_name={:?}", r.file_name);
                println!("file_mime={:?}", r.file_mime);
                println!("file_content length={}", r.file_content.len());
                let mut resp = RegisterResponse::new();
                resp.set_result(true);
                sink.success(resp)
            })
            .map_err(|e| match e {
                GrpcError::RemoteStopped => {}
                e => error!("failed to send streaming input: {:?}", e),
            });
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));

    let service = resource_with_file_grpc::create_register_service(Server);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 8000)
        .build()
        .unwrap();
    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
