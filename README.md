<img src="fastql-logo.png" width="100" height="100">

## FastQL Inference Server

Spin up a blazing fast rust GraphQL API and query around your ML model in one line of python code.

**NB. This is currently prototype only, not suitable for production. Can only create flat / non nested schema. Make sure you set RUST_ENV to production if you are using it on a remote machine**

#### How to:

`pip install fastqlapi`

Visit localhost:8000/graphiql for the graphql playground UI or make a request to localhost:8000

example:

```
from fastqlapi import fastql_server
def test(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

fastql_server.start(callback=test, query_name="Model", args={"input": { "type": "String", "description": "this is my input field"}}, fields={"output": { "type": "String"}})
```

to try with an example schema:

```
from fastqlapi import fastql_server, test_args, test_fields

def test(**kwargs):
    print (kwargs['prompt'])
    return {
        "tokens": ["example", "tokens"],
    }

fastql_server.start(callback=test, args=testargs, fields=testfields)
```

<br/>

- FastQL implements all the basic GraphQL types and array types, including required types but not currently
  required subtypes (an element of a list).

- Using types URL, URL!, [URL] or [URL!] in python code will cause a valid URL as a returned value under that type to be downloaded.

- Under the hood FastQL uses the actix rust web server which is currently no.5 fastest web framework according to https://www.techempower.com/benchmarks/#section=data-r21&test=composite. By comparison, python's FastAPI is at no.93. We've observed about a 2x speed up across the example schema here vs a FastAPI/Ariadne python GraphQL server with the same schema.

### Environment variables

**GRAPHQL_HOST**
Default localhost

**GRAPHQL_PORT**
Default 8000

**RUST_LOG**
Rust log level | default 'debug'

**RUST_BACKTRACE**
Add rust backtrace to log | default 1

**RUST_QUIET**
No rust logs | default false

**TRACING**
Turn on Apollo tracing | default false

**DOWNLOAD_PATH**
Path to download files given as a value for URL types | default ./
