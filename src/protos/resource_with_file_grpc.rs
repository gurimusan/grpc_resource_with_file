// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_REGISTER_SERVICE_REGISTER: ::grpcio::Method<super::resource_with_file::RegisterRequest, super::resource_with_file::RegisterResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/resource_with_file.RegisterService/register",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RegisterServiceClient {
    client: ::grpcio::Client,
}

impl RegisterServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RegisterServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn register_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::resource_with_file::RegisterRequest>, ::grpcio::ClientCStreamReceiver<super::resource_with_file::RegisterResponse>)> {
        self.client.client_streaming(&METHOD_REGISTER_SERVICE_REGISTER, opt)
    }

    pub fn register(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::resource_with_file::RegisterRequest>, ::grpcio::ClientCStreamReceiver<super::resource_with_file::RegisterResponse>)> {
        self.register_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RegisterService {
    fn register(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::resource_with_file::RegisterRequest>, sink: ::grpcio::ClientStreamingSink<super::resource_with_file::RegisterResponse>);
}

pub fn create_register_service<S: RegisterService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_REGISTER_SERVICE_REGISTER, move |ctx, req, resp| {
        instance.register(ctx, req, resp)
    });
    builder.build()
}
