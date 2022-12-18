from fastqlapi import fastql_server
import torch

def test(**kwargs):
    print (kwargs['input'])
    print (kwargs['urls'])
    return {
        'output': "test response",
        'seed': torch.random.initial_seed()
    }

fastql_server.start(
    callback=test, 
    query_name="Model2", 
    args={"input": { "type": "String", "description": "my input"}, "urls": { "type": "[URL]", "description": "an image to upload"}}, 
    fields={"output": { "type": "String"}, "seed": { "type": "Int" }})
