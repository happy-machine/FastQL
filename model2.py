import time
import random
import string
import GQLwrapper
from lib.wrapper import graphql_wrapper

model = 0

def start_server():
    global model
    model = 10
    # GQLwrapper.init(["field1", "field2"])

def run_model():
    global model
    model = model + 1
    return model

# def run_model(arg):
#     return arg

graphql_wrapper.setup(callback=run_model, fields=["field1", "field2"])
graphql_wrapper.listen()