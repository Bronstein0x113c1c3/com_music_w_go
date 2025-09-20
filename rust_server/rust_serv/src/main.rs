
use std::{future::poll_fn, net::SocketAddr};

use anyhow::Context;
use msquic_async::msquic::*;
use msquic_async::*;
use tokio::{io::AsyncWriteExt, *};
use tracing::{error, info};
use tracing_subscriber::*;
pub mod music_reading;

#[tokio::main]
async fn main()->anyhow::Result<()>{

    tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
    .with_writer(std::io::stderr)
    .with_max_level(tracing::Level::INFO)
    .init();




    /*
     1. credential
     2. registration , configuration
     3. listening...
     */

    let cred_config = msquic_async::msquic::CredentialConfig::new()
    .set_credential(msquic_async::msquic::Credential::CertificateFile(CertificateFile::new("server_key.pem".to_string(), "server_cert.pem".to_string())));

    let registration = msquic::Registration::new(&msquic::RegistrationConfig::default())?;

    let alpn = [msquic::BufferRef::from("sample")];

    let configuration = msquic::Configuration::open(
        &registration,
        &alpn,
        Some(
            &msquic::Settings::new()
            .set_IdleTimeoutMs(10000)
            .set_PeerBidiStreamCount(100)
            .set_PeerUnidiStreamCount(100)
            .set_DatagramReceiveEnabled()
            .set_StreamMultiReceiveEnabled(),
        ),
    )?;
    configuration.load_credential(&cred_config)?;

    let listener = msquic_async::Listener::new(&registration, configuration)?;

    // let addr = Addr::as_socket()
    // listener = Listener::start(alpn,Some(":8080".parse()))?;
    listener.start(&alpn, Some("0.0.0.0:8080".parse()?))?;
    while let Ok(conn) = listener.accept().await {
           // info!("a new connection from {:?}", conn.get_remote_addr()?);
           // let mut r =  conn.open_outbound_stream(StreamType::Unidirectional,false).await?;
           // r.write_all(b"something").await?;
           // poll_fn(|cx| r.poll_finish_write(cx)).await?;







    }
    Ok(())
}

