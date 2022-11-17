import GQLwrapper

class PythonApi:

    def __init__(self):
        self.callback = None

    def set_callback(self, callback):
        print("This is PythonApi::set_callback")
        self.callback = callback

    def call_callback(self):
        print("This is PythonApi::call_callback")
        assert self.callback is not None
        self.callback(self)

    def some_operation(self):
        print("This is PythonApi::some_operation")

def python_function(python_api, callback):
    print("This is python_function")
    python_api.callback = callback


def main():
    print("This is main")
    python_api = PythonApi()
    print("Calling rust_register_callback")
    GQLwrapper.rust_register_callback(python_api)
    print("Returned from rust_register_callback; back in main")
    print("Calling callback")
    python_api.call_callback()


if __name__ == '__main__':
    main()