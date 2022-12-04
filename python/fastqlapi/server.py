import os;
import zmq;
import fastqlapi;
import json;

class Wrapper:
    def __init__(self):
        print('Initialising FastQL...')
        self.callback = None
        self.args = {}
        self.fields = {}
        self.context = zmq.Context()
    def listen(self):
        fastqlapi.init(self.args, self.fields)
        print(f"Started GraphQL server on http//{os.getenv('GRAPHQL_HOST', default='localhost')}:{os.getenv('GRAPHQL_PORT', default='8000')}.")
        while True:
            socket = self.context.socket(zmq.REP)
            socket.connect(f"tcp://{os.getenv('ZEROMQ_HOST', default='localhost')}:{os.getenv('ZEROMQ_PORT', default='5555')}")
            while True:
                message = socket.recv_string()
                parsed = json.loads(message)
                out = {}
                for k,v in parsed.items():
                    try:
                      out[k] = json.loads(v)
                    except:
                        out[k] = v
                result = json.dumps(self.run_model(**out))
                socket.send_string(result)
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