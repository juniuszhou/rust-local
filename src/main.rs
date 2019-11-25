use async_std::task;
use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;


async fn listen(){
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = incoming.next().await {
        let stream = stream.unwrap();
        let (reader, writer) = &mut (&stream, &stream);
        // io::copy(reader, writer).await;
        writer.write_all(b"Hello from Jun").await;
    }
}

async fn exec(handler: task::JoinHandle<()>) {
    handler.await
}

fn main() {
    let handle = task::spawn(listen());
    println!("listen now");
    task::block_on(exec(handle));

}
