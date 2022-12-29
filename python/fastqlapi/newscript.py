if self.args[k]['type'] in ["URL", "URL!"]:
    self.download(v)
elif self.args[k]['type'] in ["[URL]", "[URL!]"]:         
    for url in json.loads(v):
        self.download(url)