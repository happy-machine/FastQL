from fastqlapi import fastql_server
def test(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

fastql_server.start(callback=test, args={"input": { "type": "String"}}, fields={"output": { "type": "String"}})