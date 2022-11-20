from lib.wrapper import graphql_wrapper
import time

def model_runner():
    print("inferring")

def runner():
    print("being called")
    graphql_wrapper.test_func()

graphql_wrapper.set_fn_to_call(model_runner)
time.sleep(5000)