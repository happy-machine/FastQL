from fastqlapi import fastql_server
import torch
def infer(**kwargs):
    print (kwargs['input'])
    return {
        'output':torch.random.seed(),
    }

fastql_server.start(callback=infer, query_name="Model", args={"input": { "type": "String", "description": "this is my input field"}}, fields={"output": { "type": "Int"}})