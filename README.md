<img src="assets/fastql-logo.png" width="60" height="80">

# FastQL Inference Server

Spin up a blazing fast Rust GraphQL query API around your ML model in one line of Python code.

Included are examples which can deploy and serve stable diffusion / runwayml / midjourney or any of the [Hugging face](https://huggingface.co/) diffusers models in seconds.

We've observed about a **2x speed up** across the example schema vs a FastAPI/Ariadne Python GraphQL server with identical schema. This is because although the model code is in Python, the API is actually a Rust process running ActiX.

**Please note**

- **Does not include an auth mechanism, you can implement auth using an API Gateway or auth proxy**.
- Can only create flat / non nested schema, currently only a single query, support for subscription and mutation coming soon.
- Make sure you set RUST_ENV to production if you are using it on a remote machine.
- This is currently a prototype and should not be used in production.

<br/>

## Why would I need FastQL?

- FastQL encourages seperation of your API and model, allowing you to focus on your model code, making PRs with your model code completely detached from the API layer.
- All the benefits of GraphQL... Fetch data with single API call, no overfetching, autogenerated API documentation, validation and type checking out-of-the-box, API evolution without versioning, subscriptions (websockets) as a first class citizen.
- FastQL is really fast, much faster than a Python GraphQL implementation, the example schema has a latency of < 20ms when tested locally.
- FastlQL is [Apollo federation](https://www.apollographql.com/docs/federation/#:~:text=Apollo%20Federation%20is%20a%20powerful,subgraph) compliant, which means that you can aggregate many models into a single curated API using schema stitching. This means you can maintain both fixed and dynamic schema's for params to your models, so it is easy to test new features and benefit from federation versioning.
- FastQL gives you GraphQL playground for testing and demonstrating your prototype models.
- Because ML models are typically sequential in terms of request response pattern, requests to the model are **always** blocking,response time is the important bottleneck FastQL is focused on reducing.

## Installation

```bash
pip install fastqlapi
```

## Usage

You can call **any** callback with each GraphQL query recieved by the server, the callback must return a dict with values for each field you wish to return. The returned type must match the type defined for the field in the fields dict given to the start method (see below).

For example:

```python
from fastqlapi import fastql_server
def infer(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

fastql_server.start(callback=infer, query_name="Model", args={"input": { "type": "String", "description": "this is my input field"}}, fields={"output": { "type": "String"}})
```

Will spin up the below GraphQL Playground ready for requests on localhost:8000/graphiql or you can make a GraphQL request to localhost:8000 like:

```graphql
{
  Model(input="to send"){
    output
  }
}
```

<img src="assets/playground-screenshot.png" width="1000" height="600">

<br/>
<br/>

### Spin up an example hugging face diffusers GraphQL API using Docker on AWS EC2

**NB. these examples are just toy examples not suitable for production**

- Launch an EC2 GPU powered instance (for example p3.2xlarge)
- Search for NVIDIA GPU-Optimized PyTorch AMI in the Amazon Machine Image search bar.
- Select this AMI and configure storage to 64GIB.
- You will need to set a PEM key-pair, download and chmod the key on your local machine using chmod 400 {{key pair name}}.
- Launch your instance.
- Get your current IPV4 IP (for example from [whatsmyip](whatsmyip.org))
- In the security tab for your instance select the auto-created or chosen security group and add an inbound rule setting custom TCP and port 8000 (graphql api) with source to {{Your IPV4 IP}} (or 0.0.0.0/0 to make it public) and another identical rule for port 8080 (the image server)
- Sign up at https://huggingface.co/ to get your free ACCESS_TOKEN.
- Setup aws configure with the command aws configure and enter access key details after creating an access key in the IAM panel.
- Run the following commands to set your public IP:

```bash
EC2_INSTANCE_ID="`wget -q -O - http://169.254.169.254/latest/meta-data/instance-id || die \"wget instance-id has failed: $?\"`"
export PUBLIC_IP=$(aws ec2 describe-instances --instance-ids $EC2_INSTANCE_ID --query 'Reservations[*].Instances[*].PublicIpAddress' --output text)
```

- `echo $PUBLIC_IP` and make a note of this public ip you will use to connect to the API
- Connect to your instance and use the example docker image on danfreshbc/fastqlapi-diffusers or run your published docker image similar to:

```bash
docker run -p 8000:8000 -p 8080:8080 \
    -e PUBLIC_IP=$PUBLIC_IP \
    -e MODEL_ID="stabilityai/stable-diffusion-2" \
    --gpus all danfreshbc/fastqlapi-diffusers:latest
```

Once Docker has finished installing...

- Connect to {{PUBLIC IP}}:8000/graphql to visit your new GraphQL API and make sure that the URL next to the history button contains {{PUBLIC IP}}:8000
- Try the following example query in the GraphQL Playground console:

```graphql
{
  Model1(prompt: "a handsome man holding a baby goat") {
    images
  }
}
```

- In the response on the right you will get back a URL you can visit to see the generated image.

You can change MODEL_ID to one of the following to try a different diffusers model for example:

```python
"CompVis/stable-diffusion-v1-4"
"stabilityai/stable-diffusion-2"
"runwayml/stable-diffusion-v1-5"
```

The example API surfaces prompt, number_of_images, guidance_scale, number_inference_steps and seed and returns an array of images or a seed.

The example uses a ruby webserver to serve up the content of an images directory which it will deposit generated images into on port 8080.

### A diffusers example implementing fine tuning using dreambooth

**NB. Mutation support coming soon.**

This example allows you to download a folder of images from google drive or individual gdrive image ids. It is suggested to upload 10-12 images to a Gdrive folder and make sure you set sharing to 'Anyone with the link' and copy the link. The toy API will then finetune the model on these images so you can run inference against them. This example will overwrite the model each time you upload new files.

- Setup your instance as above but with a machine with **at least** 16GB ram, the smallest on AWS would be m5.2xlarge. (see [Huggingface dreambooth](https://huggingface.co/docs/diffusers/training/dreambooth)).
- Use the following docker command:

```bash
docker run -p 8000:8000 -p 8080:8080 \
    -e PUBLIC_IP=$PUBLIC_IP \
    -e MODEL_ID="stabilityai/stable-diffusion-2" \
    --gpus all danfreshbc/fastqlapi-diffusers:latest bash start_finetune.sh
```

To start finetuning use a query like below:

```graphql
{
  Model(
    fine_tune_photo_description: "Photo of happymachine"
    gdrive_folder_of_images_link: "https://drive.google.com/drive/folders/17g_m3eaBA6SJQP-xppGkAHXYoKCLObZg"
  ) {
    images
  }
}
```

This query will take around five minutes while the model is tuned and prepared for inference.

To infer:

```graphql
{
  Model(prompt: "happymachine standing in front of an F1 McLaren") {
    images
  }
}
```

<br/>

## Further info

- FastQL implements all the basic GraphQL types and array types, including required types but not currently
  required subtypes (an element of a list).

- Under the hood FastQL uses the ActiX Rust web server which is currently no.5 fastest web framework according to [Tech Empower](https://www.techempower.com/benchmarks/#section=data-r21&test=composite). By comparison, Python's [FastAPI](https://fastapi.tiangolo.com/) is at no.93. We've observed about a 2x speed up across the example schema here vs a FastAPI/Ariadne Python GraphQL server with the same schema.

<br/>

## Environment variables

**GRAPHQL_HOST**
Default localhost

**GRAPHQL_PORT**
Default 8000

**MODEL_ID**
The diffusers model ID for the huggingface diffusers model you want to launch

**ACCESS_TOKEN**
Your huggingface diffusers access token

**ENABLE_GRAPHIQL**
Will enable GraphiQL | default to true unless RUST_ENV is set to production (in which case setting ENABLE_GRAPHIQL=true will still overide)

**ENABLE_CORS**
default false

**CORS_PERMISSIVE**
Should be used in Development/Test only, allows all headers and origins, credentials supported, maximum age 1 hour does not send wildcard | default false

**ALLOW_AUTHORIZATION_HEADER**
default false

**ALLOW_ORIGIN_HEADER**
default false

**ALLOW_CONTENT_TYPE_HEADER**
default false

**MAX_AGE_HEADER**
default 3600

**RUST_LOG**
Rust log level | default 'debug'

**RUST_BACKTRACE**
Add Rust backtrace to log | default 1

**RUST_QUIET**
No Rust logs | default false

**TRACING**
Turn on Apollo tracing | default false

<br/>

## License

The code in this repo is released under the MIT license.

## Thank you

The folks at [Hugging Face](https://huggingface.co/) \
The team at [Actix](https://actix.rs/) \
Sunli @ [Async-graphql.rs](https://github.com/async-graphql/async-graphql) \
The team at [Maturin](https://github.com/PyO3/maturin)
