// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use strum::IntoEnumIterator;
use sui_macros::sim_test;
use sui_single_node_benchmark::command::{Component, WorkloadKind};
use sui_single_node_benchmark::run_benchmark;
use sui_single_node_benchmark::workload::Workload;

#[sim_test]
async fn benchmark_non_move_transactions_smoke_test() {
    for component in Component::iter() {
        run_benchmark(
            Workload::new(
                10,
                WorkloadKind::PTB {
                    num_transfers: 2,
                    use_native_transfer: true,
                    num_dynamic_fields: 0,
                    computation: 0,
                },
            ),
            component,
            1000,
            false,
        )
        .await;
    }
}

#[sim_test]
async fn benchmark_move_transactions_smoke_test() {
    // This test makes sure that the benchmark runs.
    for component in Component::iter() {
        run_benchmark(
            Workload::new(
                10,
                WorkloadKind::PTB {
                    num_transfers: 2,
                    use_native_transfer: false,
                    num_dynamic_fields: 1,
                    computation: 1,
                },
            ),
            component,
            1000,
            false,
        )
        .await;
    }
}

#[sim_test]
async fn benchmark_publish_from_source() {
    // This test makes sure that the benchmark runs.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend([
        "tests",
        "data",
        "package_publish_from_source",
        "manifest.json",
    ]);
    for component in Component::iter() {
        run_benchmark(
            Workload::new(
                10,
                WorkloadKind::Publish {
                    manifest_file: path.clone(),
                },
            ),
            component,
            1000,
            false,
        )
        .await;
    }
}

#[sim_test]
async fn benchmark_publish_from_bytecode() {
    // This test makes sure that the benchmark runs.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend([
        "tests",
        "data",
        "package_publish_from_bytecode",
        "manifest.json",
    ]);
    for component in Component::iter() {
        run_benchmark(
            Workload::new(
                10,
                WorkloadKind::Publish {
                    manifest_file: path.clone(),
                },
            ),
            component,
            1000,
            false,
        )
        .await;
    }
}
