use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    let blob_client = ClientBuilder::emulator().blob_client("cats", file_name);

    blob_client
        .put_block_blob("hello world")
        .content_type("text/plain")
        .await?;

    let mut result: Vec<u8> = vec![];

    // The stream is composed of individual calls to the get blob endpoint
    let mut stream = blob_client.get().into_stream();
    while let Some(value) = stream.next().await {
        let mut body = value?.data;
        // For each response, we stream the body instead of collecting it all
        // into one large allocation.
        while let Some(value) = body.next().await {
            let value = value?;
            result.extend(&value);
        }
    }

    println!("result: {:?}", result);

    Ok(())
}
