import time
import random
import string
import GQLwrapper
from lib.wrapper import fastql

model = 0

def run_model(message):
    print("running model")
    time.sleep(random.randint(0,9) * 0.005)
    # simulate variable response time
    return f"{message} : {''.join(random.choice(string.digits) for i in range(10))}"

fastql.start(callback=run_model, fields=["field1", "field2"])
