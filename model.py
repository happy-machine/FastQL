from fastqlapi import fastql_server

def test(**kwargs):
    return {
        'output': kwargs['input'],
    }

fastql_server.start(
    callback=test, 
    query_name="Test", 
    args={"input": { "type": "String", "description": "A test input field."}}, 
    fields={"output": { "type": "String"}})