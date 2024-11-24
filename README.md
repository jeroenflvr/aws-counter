# Quick rust aws sdk test

## Setup aws
**Limited to using an aws profile, for now.**

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
