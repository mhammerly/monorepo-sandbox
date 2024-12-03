use std::io::Write;
//use prost::Message;
use google_cloud_gax::grpc::Status;
use google_cloud_googleapis::pubsub::v1::schema::Type;
use google_cloud_googleapis::pubsub::v1::SchemaSettings;
use google_cloud_pubsub::client::{Client, ClientConfig};
use google_cloud_pubsub::subscription::Subscription;
use google_cloud_pubsub::subscription::SubscriptionConfig;
use google_cloud_pubsub::topic::TopicConfig;
use std::env;
use tokio_util::sync::CancellationToken;

use prost::Message;

use archive::async_archive;
use storage_proto::com::mhammerly::storage::storage_service_client::StorageServiceClient;
use storage_proto::com::mhammerly::storage::SaveRequest;

async fn setup() -> Result<Subscription, Status> {
    let client_config = ClientConfig {
        project_id: rust_config::gcp_project_id(),
        ..Default::default()
    };
    let client = Client::new(client_config).await.unwrap(); // TODO handle error

    let topic = client.topic(&rust_config::gcp_topic_id().unwrap());
    if !topic.exists(None /* RetrySetting */).await? {
        let schema_settings: Option<SchemaSettings> = if !rust_config::using_pubsub_emulator() {
            println!("Creating a schema");
            Some(SchemaSettings {
                schema: rust_config::gcp_schema_id().unwrap(),
                encoding: Type::ProtocolBuffer.into(),
                first_revision_id: "0".to_string(), // TODO figure out what these versions are
                last_revision_id: "0".to_string(),
            })
        } else {
            println!("Not creating a schema");
            None
        };
        topic
            .create(
                Some(TopicConfig {
                    schema_settings,
                    ..Default::default()
                }),
                None, /* RetrySetting */
            )
            .await?;
    } else {
        println!("Topic already exists");
    }

    let subscription = client.subscription(&rust_config::gcp_subscription_id().unwrap());
    if subscription.exists(None).await? {
        println!("Deleting subscription...");
        subscription.delete(None).await?;
    }
    if !subscription.exists(None /* RetrySetting */).await? {
        println!(
            "Creating subscription on topic {:?}",
            topic.fully_qualified_name()
        );
        subscription
            .create(
                topic.fully_qualified_name(),
                SubscriptionConfig::default(),
                None, /* RetrySetting */
            )
            .await?;
    } else {
        println!("Subscription already exists");
    }

    std::io::stdout().flush().unwrap();
    Ok(subscription)
}

async fn runloop(subscription: Subscription) -> Result<(), Status> {
    let cancel = CancellationToken::new();
    std::io::stdout().flush().unwrap();
    subscription
        .receive(
            |mut message, cancel| async move {
                let ingest_request = ingest_proto::IngestRequest::decode(
                    &mut std::io::Cursor::new(&message.message.data),
                )
                .unwrap();
                println!("Request to archive: {:?}", ingest_request);

                let content = async_archive(&ingest_request.url).await;
                println!("Content length {:?}", content.len());

                let mut client = StorageServiceClient::connect("http://storage:50051")
                    .await
                    .unwrap();
                let request = tonic::Request::new(SaveRequest {
                    url: ingest_request.url,
                    content: content,
                });
                let response = client.save_archive(request).await;
                println!("gRPC response {:?}", response);

                let _ = message.ack().await;
            },
            cancel.clone(),
            None,
        )
        .await?;
    println!("Loop over");
    std::io::stdout().flush().unwrap();

    subscription.delete(None).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Status> {
    println!("setting up subscription");
    let subscription = setup().await?;

    println!("entering runloop");
    runloop(subscription).await
}
