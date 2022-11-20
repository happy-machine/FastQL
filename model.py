import time
import random
import string
from lib.wrapper import graphql_wrapper

def run_model(message):
    print("running model")
    time.sleep(random.randint(0,9) * 0.005)
    # simulate variable response time
    return f"{message} : {''.join(random.choice(string.digits) for i in range(10))}"

graphql_wrapper.set_model(run_model)
graphql_wrapper.listen()