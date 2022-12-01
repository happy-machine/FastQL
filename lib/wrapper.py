import os;
import zmq;
import fastql;
import json;
class Wrapper:
    def __init__(self):
        print('initialising..')
        self.callback = None
        self.args = {}
        self.fields = {}
        self.context = zmq.Context()
    def listen(self):
        fastql.init(self.args, self.fields)
        print('started GraphQL server.')
        while True:
            socket = self.context.socket(zmq.REP)
            socket.connect(f"tcp://{os.environ['ZEROMQ_HOST']}:{os.environ['ZEROMQ_PORT']}")
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