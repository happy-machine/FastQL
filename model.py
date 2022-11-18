from lib.wrapper import graphql_wrapper
import time

def runner():
    print("Python: doing some work")
    graphql_wrapper.call_response_callback()

graphql_wrapper.set_fn_to_call(runner)
time.sleep(5000)
