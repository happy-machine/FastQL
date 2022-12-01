from lib.wrapper import fastql_server;
from schema import args, fields;

def run_model(**kwargs):
    return {
        'images': f'https://stabilityfastqldev.s3.amazonaws.com/11_22/{kwargs["prompt"]}.jpg',
        'tokens': [0.2333, 0.4444, 0.2444],
    }
    
fastql_server.start(callback=run_model, args=args, fields=fields)
