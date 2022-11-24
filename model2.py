import time
import random
import string
from lib.wrapper import graphql_wrapper
import GQLwrapper

model = 0

def start_server():
    GQLwrapper.py_start_server(callback=cb, fields=["field1", "field2"])

def run_model():
    global model
    model = model + 1
    return model


graphql_wrapper.start_server(callback=run_model, fields=["field1", "field2"])