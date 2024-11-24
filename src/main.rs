use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use aws_config::BehaviorVersion;
use aws_sdk_s3::types::Object as AwsObject;
use aws_sdk_s3::Client;
use clap::Parser;
use error::S3ExampleError;
use helpers::helpers::bytes_to_human_readable_string;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub mod error;
mod helpers;

lazy_static! {
    static ref DEBUG: Mutex<bool> = Mutex::new(false);
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: String,
    #[arg(short, long)]
    bucket: String,
    #[arg(short, long)]
    debug: bool,
}

pub struct BucketObjects {
    prefixes: Vec<String>,
    objects: Vec<AwsObject>,
}

impl BucketObjects {
    fn summary(self) {
        let t = self
            .objects
            .iter()
            .fold(0, |acc, obj| acc + obj.size.unwrap());

        println!("\nTotal Objects: {}", self.objects.len());
        println!(
            "Total Size: {} ({} bytes)\n",
            bytes_to_human_readable_string(t),
            t
        );
    }
}

pub struct BucketRequest {
    client: Client,
    bucket: String,
    items: BucketObjects,
    request_count: u64,
}

impl BucketRequest {
    fn new(client: Client, bucket: String) -> Self {
        let prefixes = Vec::new();
        let objects = Vec::new();
        let items = BucketObjects { prefixes, objects };
        Self {
            client,
            bucket,
            items,
            request_count: 0,
        }
    }

    pub async fn list_objects(&mut self, prefix: &str) -> Result<(), S3ExampleError> {
        let mut response = self
            .client
            .list_objects_v2()
            .bucket(self.bucket.to_owned())
            .delimiter("/")
            .prefix(prefix)
            .max_keys(10) // In this example, go 10 at a time.
            .into_paginator()
            .send();

        self.request_count += 1;

        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        if *DEBUG.lock().unwrap() {
                            println!(" - {}", object.key().unwrap_or("Unknown"));
                        };
                        self.items.objects.push(object.to_owned());
                    }
                    for prefix in output.common_prefixes() {
                        if let Some(p) = prefix.prefix() {
                            if *DEBUG.lock().unwrap() {
                                println!(" |- {}", p);
                            };
                            _ = Box::pin(self.list_objects(p)).await;
                            self.items.prefixes.push(p.to_string());
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}")
                }
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // TODO: setup logging facility
    *DEBUG.lock().unwrap() = args.debug;

    if *DEBUG.lock().unwrap() {
        println!("Debug mode is on");
    }

    let bucket = args.bucket;
    let profile = args.profile;

    println!("bucket: {}", bucket);
    println!("profile: {}\n", profile);

    let region = DefaultRegionChain::builder()
        .profile_name(&profile)
        .build()
        .region()
        .await;

    let creds = DefaultCredentialsChain::builder()
        .profile_name(&profile)
        .region(region.clone())
        .build()
        .await;

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .credentials_provider(creds)
        .region(region)
        .load()
        .await;

    let client = Client::new(&config);

    let mut bucket_request = BucketRequest::new(client, bucket);

    match bucket_request.list_objects("").await {
        Ok(_) => {
            bucket_request.items.summary();
        }
        Err(e) => {
            eprintln!("Something went wrong: {}", e);
        }
    }

    if *DEBUG.lock().unwrap() {
        println!("Total #requests: {}\n", bucket_request.request_count);
    }

}
