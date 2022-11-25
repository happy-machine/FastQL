import zmq
import fastql;

class Wrapper:
    def __init__(self):
        self.callback = None
        self.fields = []
        print('initialising..')

        print('started GraphQL server.')
        self.context = zmq.Context()
    def listen(self):
        print('listening')
        fastql.init(self.fields)
        while True:
            socket = self.context.socket(zmq.PULL)
            socket.connect("tcp://localhost:5555")
            response = self.context.socket(zmq.PUSH)
            response.connect("tcp://localhost:5556")
            while True:
                message = socket.recv_string()
                result = self.run_model(message)
                response.send_string(result)
                # break
    def run_model(self, message):
        assert self.callback is not None
        result = self.callback(message)
        return result
    def start(self, **kwargs):
        assert self.fields is not []
        self.fields = kwargs['fields']
        self.callback = kwargs['callback']
        self.listen()

fastql_server = Wrapper()