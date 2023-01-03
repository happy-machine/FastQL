from fastqlapi import fastql_server
def infer(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

fastql_server.start(callback=infer, query_name="Model", args={"input": { "type": "String", "description": "this is my input field"}}, fields={"output": { "type": "String"}})