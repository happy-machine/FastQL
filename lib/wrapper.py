import zmq
import GQLwrapper;

class Wrapper:
    def __init__(self):
        self.callback = None
        print('initialising..')
        GQLwrapper.init()
        print('started GraphQL server.')
        self.context = zmq.Context()
    def listen(self):
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
    def run_model(self, message):
        assert self.callback is not None
        result = self.callback(message)
        return result
    def set_model(self, model):
        self.callback = model

graphql_wrapper = Wrapper()
