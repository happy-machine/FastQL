from lib.wrapper import graphql_wrapper
import time

def runner():
    print("doing some stuff")
    graphql_wrapper.call_response_callback("some data")

graphql_wrapper.set_fn_to_call(runner)
time.sleep(5000)
