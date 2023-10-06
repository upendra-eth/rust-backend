use std::net::SocketAddr;

use jsonrpsee::client_transport::ws::{Url, WsTransportClientBuilder};
use jsonrpsee::core::client::{Client, ClientBuilder, ClientT};
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let uri = Url::parse(&format!("ws://{}", addr))?;

	let (tx, rx) = WsTransportClientBuilder::default().build(uri).await?;
	let client: Client = ClientBuilder::default().build_with_tokio(tx, rx);
	let response: String = client.request("say_hello", rpc_params![]).await?;
	tracing::info!("response: {:?}", response);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo")?;
	let addr = server.local_addr()?;

	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}