from lib.wrapper import fastql_server;
from schema import args, fields;

# def run_model(**kwargs):
#     return {
#         'images': f'https://stabilityfastqldev.s3.amazonaws.com/11_22/{kwargs["prompt"]}.jpg',
#         'tokens': 'An array of stuff',
#     }
def run_model(**kwargs):
    return {
        'images': ['test', 'testst', 'esetst'],
        'tokens': ['test', 'testst', 'esetst'],
    }
fastql_server.start(callback=run_model, args=args, fields=fields)
