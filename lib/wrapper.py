import zmq
import fastql;
import json;
class Wrapper:
    def __init__(self):
        self.callback = None
        self.args = {}
        self.fields = {}
        print('initialising..')
        print('started GraphQL server.')
        self.context = zmq.Context()
    def listen(self):
        fastql.init(self.args, self.fields)
        while True:
            socket = self.context.socket(zmq.REP)
            socket.connect("tcp://localhost:5555")
            while True:
                message = socket.recv_string()
                parsed = json.loads(message)
                result = self.run_model(**parsed)
                socket.send_string(json.dumps(result))
    def run_model(self, **kwargs):
        assert self.callback is not None
        result = self.callback(**kwargs)
        return result
    def start(self, **kwargs):
        assert self.fields is not []
        self.args = kwargs['args']
        self.fields = kwargs['fields']
        self.callback = kwargs['callback']
        self.listen()

fastql_server = Wrapper()