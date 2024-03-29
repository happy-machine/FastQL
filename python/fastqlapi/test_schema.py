test_args = {
    'prompt': {
        'type': 'String', 
        'description': 'A text prompt for the image you want generated.'
    },
    'artifact': {
        'type': 'String'
    },
    'artifactType': {
        'type': 'String'
    },
    'model': {
        'type': 'String'
    }
}

test_fields = {
    'images': {
        'type': 'String',
        'description': 'A collection of images generated by the model'
    },
    'tokens': { 
        'type': '[String]',
        'description': 'A collection of tokens generated by the tokeniser'
    }
}