<img src="fastql-logo.png" width="100" height="100">

# FastQL Inference Server

Spin up a blazing fast Rust GraphQL API and query around your ML model in one line of python code.

We include an example which can deploy and serve stable diffusion / runwayml / midjourney or any of the diffusers models in seconds.

We've observed about a **2x speed up** across the example schema vs a FastAPI/Ariadne python GraphQL server with identical schema. This is because although the model code is in python, the API is actually a Rust process running ActiX.

We believe that API code that is used to publish your model should be seperated from the model itself.

**Please note**

- Can only create flat / non nested schema.
- Does not include an auth mechanism, you can implement auth using an API Gateway or auth proxy.
- Make sure you set RUST_ENV to production if you are using it on a remote machine.
- This is currently a prototype and should not be used in production (currently CORS is hardwired).

<br/>

## Installation

```bash
pip install fastqlapi
```

## Usage

Visit localhost:8000/graphiql for the graphql playground UI or make a request to localhost:8000

example:

```python
from fastqlapi import fastql_server
def infer(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

fastql_server.start(callback=infer, query_name="Model", args={"input": { "type": "String", "description": "this is my input field"}}, fields={"output": { "type": "String"}})
```

**or** try with the example schema:

```python
from fastqlapi import fastql_server, test_args, test_fields

def infer(**kwargs):
    print (kwargs['prompt'])
    return {
        "tokens": ["example", "tokens"],
    }

fastql_server.start(callback=infer, args=testargs, fields=testfields)
```

<br/>

## How to spin up our example 'Any diffusers API' using Docker insecurely on AWS EC2

- Sign up at https://huggingface.co/ to get your free ACCESS_TOKEN
- Launch an EC2 GPU powered instance (for example p3.2xlarge)
- Search for NVIDIA GPU-Optimized PyTorch AMI in the Amazon Machine Image search bar
- Select this AMI and configure storage to 64GIB
- You will need to set a PEM key-pair, download and chmod the key on your local machine using chmod 400 {{key pair name}}
- Launch your instance
- In the security tab for your instance select the auto-created or chosen security group and add an inbound rule setting custom TCP and port 8020 (graphql api) to 0.0.0.0/0 (or more appropriate scoped access) and another setting port 8080 (image server) to 0.0.0.0/0
- Setup aws configure with the command aws configure and enter access key details after creating an access key in the IAM panel
- Run the following commands to set your public IP:

```bash
EC2_INSTANCE_ID="`wget -q -O - http://169.254.169.254/latest/meta-data/instance-id || die \"wget instance-id has failed: $?\"`"
export EC2_PUBLIC_IP=$(aws ec2 describe-instances --instance-ids $EC2_INSTANCE_ID --query 'Reservations[*].Instances[*].PublicIpAddress' --output text)
```

- Connect to your instance and use our example docker image on blahblah/blah or run your published docker image similar to:

```bash
docker run -p 8020:8020 -p 8080:8080 \
    -e EC2_PUBLIC_IP=$EC2_PUBLIC_IP \
    -e ACCESS_TOKEN="my_access_token" \
    -e MODEL_ID="stabilityai/stable-diffusion-2" \
    --gpus all repo/yourimage:latest
```

- You can `echo $EC2_PUBLIC_IP` to get your public IP or find it in your EC2 console instance details
- Connect to {{EC2_PUBLIC IP}}:8020/graphql to visit your new GraphQL API and make sure that the URL next to the history button contains {{EC2_PUBLIC IP}}:8020
- An example query:

```graphql
{
  Model1(prompt: "a handsome man holding a baby goat") {
    images
  }
}
```

You can change MODEL_ID to one of the following to try a different diffusers model for example:

```python
"CompVis/stable-diffusion-v1-4"
"stabilityai/stable-diffusion-2"
"runwayml/stable-diffusion-v1-5"
```

The example API surfaces prompt, number_of_images, guidance_scale, number_inference_steps and seed and returns an array of images or a seed.
<br/>

## Further info

- FastQL implements all the basic GraphQL types and array types, including required types but not currently
  required subtypes (an element of a list).

- Using types URL, URL!, [URL] or [URL!] in python code will translate to GraphQL String equivalents, but will cause a valid URLreturned under that type to be downloaded.

- Under the hood FastQL uses the actix rust web server which is currently no.5 fastest web framework according to https://www.techempower.com/benchmarks/#section=data-r21&test=composite. By comparison, python's FastAPI is at no.93. We've observed about a 2x speed up across the example schema here vs a FastAPI/Ariadne python GraphQL server with the same schema.
  <br/>

### Environment variables

**GRAPHQL_HOST**
Default localhost

**GRAPHQL_PORT**
Default 8000

**DOWNLOAD_PATH**
Path to download files given as a value for URL types | default ./

**MODEL_ID**
The diffusers model ID for the huggingface diffusers model you want to launch

**ACCESS_TOKEN**
Your huggingface diffusers access token

**RUST_LOG**
Rust log level | default 'debug'

**RUST_BACKTRACE**
Add rust backtrace to log | default 1

**RUST_QUIET**
No rust logs | default false

**TRACING**
Turn on Apollo tracing | default false

<br/>

## Thank you

The folks at https://huggingface.co/
The team at https://actix.rs/
Sunli @ https://github.com/async-graphql/async-graphql
The team at https://github.com/PyO3/maturin
