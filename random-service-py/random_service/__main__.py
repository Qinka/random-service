from   random_service import *
import argparse

from yaml import load, dump
try:
    from yaml import CLoader as Loader, CDumper as Dumper
except ImportError:
    from yaml import Loader, Dumper


if __name__ == '__main__':
    ap = argparse.ArgumentParser()
    ap.add_argument('--config', dest = 'config', default = 'config.yaml', type = str, help = 'Configuration file')
    args = ap.parse_args()
    with open(args.config) as file:
        config = load(file.read(), Loader=Loader)
    if 'random-service-py' not in config:
        raise Exception("invailed config file, need field of random-service-py")
    else:
        raw = config
        config = config['random-service-py']
    app.config['raw'] = raw
    app.config['MONGO_URI'] = raw['auth-mg']['uri']
    app.run(host='0.0.0.0', port=config['port'])
