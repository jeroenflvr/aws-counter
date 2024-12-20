# Quick rust aws sdk test

## Introduction
**Limited to using an aws profile, for now.**

Counts objects and total object size in a given bucket. Using delimiter, each request is limited to a single prefix, recursive and async.

## Usage

```shell
$ aws-counter --profile <my-profile> --bucket <my-bucket> --debug
Debug mode is on
bucket: <my-bucket>  
profile:  <my-profile>

 |- prefix_01/
 |- prefix_01/sub/
 - ... object_1
 - ... object_2
 - ... object_3
 - ... object_4

Total Objects: 4
Total Size: 38.32 MB (40178340 bytes)
  
Total #requests: 4
  
$ 
```


## Build
```shell
$ cargo build --release
Compiling aws-counter v0.1.0 (/projects/aws-counter)
Finished `release` profile [optimized] target(s) in 58.85s
$ 
```

## Setup aws

### config
~/.aws/config
```
[profile <profile-name>]
region = <your-region> 
output = json
services = <profile-name>-services

[services <profile-name>-services]
s3 =
  endpoint = https://s3.<region>.amazonaws.com

```

### credentials
~/.aws/credentials

```
[<profile-name>]
aws_access_key_id = <your access key> 
aws_secret_access_key = <your secret access key> 
```

## tip

use [awscli](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) for quick testing/provisioning/etc..

## TODO
- [ ] arc mutex on update from coroutine
- [ ] limit concurrency (semaphore)
- [ ] setup logging facility