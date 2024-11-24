// use std::env;
use aws_config::BehaviorVersion;
use aws_config::default_provider::credentials::DefaultCredentialsChain;
use aws_config::default_provider::region::DefaultRegionChain;
use error::S3ExampleError;
use aws_sdk_s3::types::Object as AwsObject;
use aws_sdk_s3::Client;
use clap::{Parser};
use helpers::helpers::bytes_to_human_readable_string;

pub mod error;
mod helpers;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: String,
    #[arg(short, long)]
    bucket: String
}

pub struct BucketObjects {
    prefixes: Vec<String>,
    objects: Vec<AwsObject>
}

impl BucketObjects {
    fn summary(self) {

        let t = self.objects.iter().fold(0, |acc, obj|
                acc + obj.size.unwrap());

        println!("Total Objects: {}", self.objects.len());
        println!("Total Size: {} ({} bytes)", bytes_to_human_readable_string(t), t);
    }
}

pub struct BucketRequest {
    client: Client,
    bucket: String,
    items: BucketObjects,
}

impl BucketRequest {
    fn new(client: Client, bucket: String) -> Self {
        let prefixes = Vec::new();
        let objects = Vec::new();
        let items = BucketObjects{prefixes, objects};
        Self {
            client,
            bucket,
            items,
        }
    }

    pub async fn list_objects(&mut self, prefix: &str) -> Result<(), S3ExampleError> {
        let mut response = self.client
            .list_objects_v2()
            .bucket(self.bucket.to_owned())
            .delimiter("/")
            .prefix(prefix)
            .max_keys(10) // In this example, go 10 at a time.
            .into_paginator()
            .send();

        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {

                    for object in output.contents() {
                        println!(" - {}", object.key().unwrap_or("Unknown"));
                        self.items.objects.push(object.to_owned());
                    }
                    for prefix in output.common_prefixes(){
                        if let Some(p) = prefix.prefix(){
                            println!(" |- {}", p);
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

    let bucket = args.bucket;
    let profile = args.profile;

    println!("bucket: {}", bucket);
    println!("profile: {}", profile);


    let region = DefaultRegionChain::builder().profile_name(&profile).build().region().await;

    let creds = DefaultCredentialsChain::builder()
        .profile_name(&profile)
        .region(region.clone())
        .build()
        .await;

    let config = aws_config::defaults(BehaviorVersion::v2024_03_28()).credentials_provider(creds).region(region).load().await;

    let client = Client::new(&config);

    let mut bucket_request = BucketRequest::new(client, bucket);

    _ = bucket_request.list_objects("").await;

    bucket_request.items.summary();

    // _ = list_objects(&client, bucket).await;




}