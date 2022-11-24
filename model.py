import random
import string
from lib.wrapper import fastql_server


def run_model(message):
    # print("running model")
    # simulate variable response time
    return f"{message} : {''.join(random.choice(string.digits) for i in range(10))}"

fastql_server.start(callback=run_model, fields=["field1", "field3"], params=["field1", "field3"])
