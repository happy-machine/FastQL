# FastQL API

Spin up a blazing fast rust GraphQL server in one line of python code.

**NB. This is currently prototype only**

#### How to:

`pip install fastqlapi`

example:

```
from fastqlapi import fastql_server
def test(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

fastql_server.start(callback=test, args={"input": { "type": "String", "description": "this is my input field"}}, fields={"output": { "type": "String"}})
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
FastQL implements all the basic GraphQL types and array types, including required types but not currently
required subtypes (an element of a list).

Under the hood FastQL uses the actix rust web server which is currently no.7 fastest web framework according to https://www.techempower.com/benchmarks/#section=data-r21. By comparison, python's FastAPI is no.279. I've observed about a 2x speed up across the example schema here vs a FastAPI/Ariadne python GraphQL server with the same schema.

### Environment variables

- RUST_LOG='debug'
  Rust log level
- RUST_BACKTRACE=1
  Add rust backtrace to log
- RUST_QUIET=''
  No rust logs
- GRAPHQL_HOST='localhost'
- GRAPHQL_PORT='8020'
