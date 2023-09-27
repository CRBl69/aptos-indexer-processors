// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{utils::custom_configs::CustomProcessorConfigs, worker::Worker};
use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use server_framework::RunnableConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndexerGrpcProcessorConfig {
    pub processor_name: String,
    pub postgres_connection_string: String,
    // TODO: add tls support.
    pub indexer_grpc_data_service_address: String,
    // Indexer GRPC http2 ping interval in seconds; default to 30.
    // tonic ref: https://docs.rs/tonic/latest/tonic/transport/channel/struct.Endpoint.html#method.http2_keep_alive_interval
    pub indexer_grpc_http2_ping_interval_in_secs: Option<u64>,
    // Indexer GRPC http2 ping timeout in seconds; default to 10.
    pub indexer_grpc_http2_ping_timeout_in_secs: Option<u64>,
    pub auth_token: String,
    pub starting_version: Option<u64>,
    pub ending_version: Option<u64>,
    pub number_concurrent_processing_tasks: Option<usize>,
    pub custom_processor_configs: Option<CustomProcessorConfigs>,
    // TODO: Move these vars into individual config structs for different processors.
    pub nft_points_contract: Option<String>,
    pub pubsub_topic_name: Option<String>,
    pub google_application_credentials: Option<String>,
}

#[async_trait::async_trait]
impl RunnableConfig for IndexerGrpcProcessorConfig {
    async fn run(&self) -> Result<()> {
        let mut worker = Worker::new(
            self.processor_name.clone(),
            self.postgres_connection_string.clone(),
            self.indexer_grpc_data_service_address.clone(),
            std::time::Duration::from_secs(
                self.indexer_grpc_http2_ping_interval_in_secs.unwrap_or(30),
            ),
            std::time::Duration::from_secs(
                self.indexer_grpc_http2_ping_timeout_in_secs.unwrap_or(10),
            ),
            self.auth_token.clone(),
            self.starting_version,
            self.ending_version,
            self.number_concurrent_processing_tasks,
            self.custom_processor_configs.clone(),
            self.nft_points_contract.clone(),
            self.pubsub_topic_name.clone(),
            self.google_application_credentials.clone(),
        )
        .await;
        worker.run().await;
        Ok(())
    }

    fn get_server_name(&self) -> String {
        "idxproc".to_string()
    }
}