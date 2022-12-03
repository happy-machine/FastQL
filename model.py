from fastqlapi import wrapper, test_schema
def test(**kwargs):
    print (kwargs['input'])
    return {
        'output': "test response",
    }

wrapper.fastql_server.start(callback=test, args={"input": { "type": "String"}}, fields={"output": { "type": "String"}})