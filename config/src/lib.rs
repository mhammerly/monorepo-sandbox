use std::env;

pub fn container_repo() -> Option<String> {
    env::var("CONTAINER_REPO").ok()
}

pub fn gcp_project_id() -> Option<String> {
    println!("{:?}", env::var("GCP_PROJECT_ID"));
    env::var("GCP_PROJECT_ID").ok()
}

pub fn gcp_topic_id() -> Option<String> {
    env::var("GCP_TOPIC_ID").ok()
}

pub fn gcp_schema_id() -> Option<String> {
    env::var("GCP_SCHEMA_ID").ok()
}

pub fn gcp_subscription_id() -> Option<String> {
    env::var("GCP_SUBSCRIPTION_ID").ok()
}

pub fn using_pubsub_emulator() -> bool {
    env::var("PUBSUB_EMULATOR_HOST").is_ok()
}
