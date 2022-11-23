import time
import random
import string
# from lib.wrapper import st_df_2
import GQLwrapper

model = 0

def start_server():
    GQLwrapper.py_start_server(["field1", "field2"])

def run_model():
    global model
    model = model + 1
    return model