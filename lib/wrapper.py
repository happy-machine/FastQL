import zmq
import GQLwrapper;

class Wrapper:
    def __init__(self):
        self.callback = None
        self.context = zmq.Context()
        print('initialising..')
    def listen(self):
        print('listening...')
        while True:
            socket = self.context.socket(zmq.REP)
            socket.connect("tcp://localhost:5555")
            response = self.context.socket(zmq.REQ)
            response.connect("tcp://localhost:5556")
            while True:
                message = socket.recv_string()
                result = self.run_model(message)
                response.send_string(result)
                break
    def start_server(self, **kwargs):
        self.callback = kwargs['callback']
        GQLwrapper.py_start_server(kwargs['fields'])
        print('started GraphQL server.')
        self.listen()
    def run_model(self, message):
        assert self.callback is not None
        result = self.callback(message)
        return result
        
graphql_wrapper = Wrapper()
