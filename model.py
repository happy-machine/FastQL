from lib.wrapper import graphql_wrapper
import time

def model_runner():
    print("inferring")

graphql_wrapper.set_fn_to_call(model_runner)
time.sleep(5000)
