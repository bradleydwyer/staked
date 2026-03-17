use crate::registry::{AvailableSignal, Registry, browse_url, check_url};
use crate::types::*;
use reqwest::Client;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

fn build_client() -> Client {
    Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("staked/1.0.0")
        .build()
        .expect("Failed to build HTTP client")
}

async fn check_registry(
    client: &Client,
    registry: &'static Registry,
    name: &str,
    semaphore: &Semaphore,
) -> PackageResult {
    let _permit = semaphore.acquire().await.unwrap();
    let start = Instant::now();
    let url = check_url(registry, name);

    let mut request = client.get(&url);
    for &(key, value) in registry.headers {
        request = request.header(key, value);
    }

    match request.send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.ok();

            let available = dispatch_signal(registry.signal, status, body.as_deref());

            PackageResult {
                registry_id: registry.id.to_string(),
                registry_name: registry.name.to_string(),
                available,
                browse_url: Some(browse_url(registry, name)),
                elapsed_ms: start.elapsed().as_millis() as u64,
                error: None,
            }
        }
        Err(e) => PackageResult {
            registry_id: registry.id.to_string(),
            registry_name: registry.name.to_string(),
            available: Availability::Unknown,
            browse_url: Some(browse_url(registry, name)),
            elapsed_ms: start.elapsed().as_millis() as u64,
            error: Some(e.to_string()),
        },
    }
}

fn dispatch_signal(
    signal: AvailableSignal,
    status: reqwest::StatusCode,
    body: Option<&str>,
) -> Availability {
    match signal {
        AvailableSignal::Http404 => {
            if status.as_u16() == 404 || status.as_u16() == 410 {
                Availability::Available
            } else if status.is_success() {
                Availability::Taken
            } else {
                Availability::Unknown
            }
        }
        AvailableSignal::EmptyResults => {
            if !status.is_success() {
                return if status.as_u16() == 404 {
                    Availability::Available
                } else {
                    Availability::Unknown
                };
            }
            match body {
                Some(text) => {
                    let trimmed = text.trim();
                    if trimmed.is_empty() || trimmed == "[]" {
                        return Availability::Available;
                    }
                    match serde_json::from_str::<serde_json::Value>(trimmed) {
                        Ok(serde_json::Value::Array(arr)) if arr.is_empty() => {
                            Availability::Available
                        }
                        Ok(_) => Availability::Taken,
                        Err(_) => Availability::Unknown,
                    }
                }
                None => Availability::Unknown,
            }
        }
        AvailableSignal::NullOrError => {
            if status.as_u16() == 404 {
                return Availability::Available;
            }
            if !status.is_success() {
                return Availability::Unknown;
            }
            match body {
                Some(text) => {
                    let trimmed = text.trim();
                    if trimmed == "null" || trimmed == "false" {
                        return Availability::Available;
                    }
                    match serde_json::from_str::<serde_json::Value>(trimmed) {
                        Ok(serde_json::Value::Null) => Availability::Available,
                        Ok(serde_json::Value::Object(ref obj)) if obj.contains_key("error") => {
                            Availability::Available
                        }
                        Ok(_) => Availability::Taken,
                        Err(_) => Availability::Unknown,
                    }
                }
                None => Availability::Unknown,
            }
        }
        AvailableSignal::XmlNoEntry => {
            if !status.is_success() {
                return Availability::Unknown;
            }
            match body {
                Some(text) => {
                    if text.contains("<entry") {
                        Availability::Taken
                    } else {
                        Availability::Available
                    }
                }
                None => Availability::Unknown,
            }
        }
    }
}

async fn check_package_inner(
    name: &str,
    registries: &[&'static Registry],
    client: &Client,
    semaphore: &Arc<Semaphore>,
) -> CheckResult {
    let start = Instant::now();
    let name = name.trim().to_string();

    let mut handles = Vec::new();
    for &registry in registries {
        let client = client.clone();
        let sem = Arc::clone(semaphore);
        let name = name.clone();
        handles.push(tokio::spawn(async move {
            check_registry(&client, registry, &name, &sem).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }

    let summary = Summary {
        available: results
            .iter()
            .filter(|r| r.available == Availability::Available)
            .count(),
        taken: results
            .iter()
            .filter(|r| r.available == Availability::Taken)
            .count(),
        unknown: results
            .iter()
            .filter(|r| r.available == Availability::Unknown)
            .count(),
        total: results.len(),
    };

    CheckResult {
        name,
        summary,
        results,
        elapsed_ms: start.elapsed().as_millis() as u64,
    }
}

pub async fn check_package(name: &str, registries: &[&'static Registry]) -> CheckResult {
    let client = build_client();
    let semaphore = Arc::new(Semaphore::new(20));
    check_package_inner(name, registries, &client, &semaphore).await
}

pub async fn check_packages(
    names: &[String],
    registries: &[&'static Registry],
) -> Vec<CheckResult> {
    let client = build_client();
    let semaphore = Arc::new(Semaphore::new(20));

    let mut handles = Vec::new();
    for name in names {
        let name = name.clone();
        let registries = registries.to_vec();
        let client = client.clone();
        let sem = Arc::clone(&semaphore);
        handles.push(tokio::spawn(async move {
            check_package_inner(&name, &registries, &client, &sem).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    results
}
