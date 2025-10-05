#![feature(iter_array_chunks)]
use anyhow::Context;
use msquic_async::msquic::*;
use msquic_async::*;
use rodio::Source;
use std::future::poll_fn;
use tokio::{io::AsyncWriteExt, *};
use tracing::{error, info};
use tracing_subscriber::*;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    /*
     *    1. credential
     *    2. registration , configuration
     *    3. listening...
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


    // load some music....
    // then get some basic info
    // file -> sound chunk.






    // let bytes: Vec<u8> = buffer
    // .iter()
    // .flat_map(|&num| num.to_be_bytes().to_vec())
    // .collect();



    while let Ok(conn) = listener.accept().await {
        tokio::spawn(async move{

            info!("a new connection from {:?}", conn.get_remote_addr().unwrap());
            let mut music_chan = reading();
            let mut r =  conn.open_outbound_stream(StreamType::Unidirectional,false).await.unwrap();

            while let Some(buf) = music_chan.recv().await{
                r.write_all(buf.as_slice()).await.unwrap();

            }
            r.flush().await.unwrap();
            // r.write_all(b"something").await?;
            poll_fn(|cx| r.poll_finish_write(cx)).await.unwrap();
        });
    }
    // listener.stop()
    Ok(())
    // reading().await?;
    // Ok(())
}

fn reading() -> tokio::sync::mpsc::Receiver<Vec<u8>> {
    let file = std::io::BufReader::new(
        std::fs::File::open("../../list_song/04 - In My Blood.flac").unwrap(),
    );
    // std::io::pipe()
    let source = rodio::Decoder::new(file).unwrap();
    println!("{:?}",source.channels());
    // source.into_iter()
    // let sample: Vec<i16> = source.by_ref().collect();
    let mut iter=Iterator::array_chunks::<1000>(source);

    let (tx, rx) = tokio::sync::mpsc::channel(250);
    tokio::spawn(async move{
        while let Some(buf) = iter.next() {
            let buf: Vec<u8> = buf
            .iter()
            .flat_map(|&num| num.to_be_bytes().to_vec())
            .collect();
            tx.send(buf).await.unwrap();
            // println!("{:?}", buf1.len());
        }
        if let Some(buf) = iter.into_remainder(){
            let buf = buf.flat_map(|num| num.to_be_bytes().to_vec()).collect();
            tx.send(buf).await.unwrap();
        }
        drop(tx);
    });
    return rx;


    // println!("{:?}", sample.len());

     // = sample.chunks(200);



    //load real-time and asynchronously:
    /*

     load(i):
        1. load file, source
        2. s


     */

    // while (!iter.next().is_none()){
    //     let bytes: Vec<u8> = iter
    //     .flat_map(|&num| num.to_be_bytes().to_vec())
    //     .collect();
    // }

    // Ok(())
}
