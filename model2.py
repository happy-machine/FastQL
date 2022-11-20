# from lib.wrapper import graphql_wrapper
import GQLwrapper;
import os
import time
import zmq
import sys



test = 0
def run_model():
    global test
    test = test + 1
    # time.sleep(3300)


def runner():
    global test
    print("doing some stuff")
    print("state", test)

    # graphql_wrapper.call_response_callback("some data")
print('name: ', __name__)
if __name__ == "__main__":

    context = zmq.Context()
    GQLwrapper.init()
    run_model()
    print("main state: ", test)
    socket = context.socket(zmq.REQ)
    socket.connect("tcp://localhost:1234")
    # message = socket.recv()
    socket.recv()
    #print ("Received reply ", message)
# else:
#     runner()



# graphql_wrapper.set_fn_to_call(runner)
