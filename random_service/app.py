from flask import Flask, redirect, request, make_response, abort, render_template
from logging.config import dictConfig
from werkzeug.exceptions import BadRequest
import re
import json

dictConfig({
    'version': 1,
    'formatters': {'default': {
        'format': '[%(asctime)s] %(levelname)s in %(module)s: %(message)s',
    }},
    'handlers': {'wsgi': {
        'class': 'logging.StreamHandler',
        'stream': 'ext://flask.logging.wsgi_errors_stream',
        'formatter': 'default'
    }},
    'root': {
        'level': 'INFO',
        'handlers': ['wsgi']
    }
})

app = Flask(__name__.split('.')[0])

@app.errorhandler(BadRequest)
def handle_bad_request(e):
    return str(e), 400

@app.route("/")
def index():
    return redirect('https://github.com/Qinka/random-service')

ipv4_pattern = r'(([0-9]{1,3}.)+[0-9]{1,3})+'
ipv6_pattern = r'(([a-f0-9]{1,4}:?:)+[a-f0-9]{1,4})+'

@app.route('/ipaddr', methods=['POST'])
def post_ipaddr():
    global ipv4_pattern
    context = get_file_or_param('context')
    typ = request.args.get('ip','v4')
    rt = ''
    pattern = ipv4_pattern
    if typ == 'v6':
        pattern = ipv6_pattern
    spt = request.args.get('split', '\n')
    ips = map(lambda x: re.findall(pattern, x), context.split(spt))
    return unwords([x[0][0] for x in ips if len(x) > 0])

@app.route('/domain', methods=['POST'])
def post_domain_record():
    record_json = get_record_json()
    domain = request.args['domain']
    rt = [x for x in record_json['domains'] if x.get('punycode',None) == domain]
    field = request.args.get('field', None)
    if field == None:
        return str(rt)
    else:
        return unlines([x.get(field, '') for x in rt])

@app.route('/record', methods=['POST'])
def post_record_del():
    record_json  = get_record_json()
    record_name  = request.args['name']
    record_field = request.args.get('field', 'id')
    record_line  = request.args.get('line', None)
    record_type  = request.args.get('type', None)
    record_value = request.args.get('value', None)
    def match(x):
        if x['name'] != record_name:
            return False
        if record_line != None and x['line'] != record_line:
            return False
        if record_type != None and x['type'] != record_type:
            return False
        if record_value != None and x['value'] != record_value:
            return False
        return True
    return unlines([x[record_field] for x in record_json['records'] if match(x)])

@app.route('/install')
def get_install():
    return render_template('install.sh', request = request)
@app.route('/install/default')
def get_install_default():
    return render_template('default.sh', root = request.url_root)
@app.route('/install/domain')
def get_install_domain():
    return render_template('domain.sh', domain = request.args['domain'])
@app.route('/install/sub-domain')
def get_install_subdomain():
    return render_template('subdomain.sh', domain_file = request.args['domain_file'], sub_domain = request.args['sub_domain'])
@app.route('/install/update.list')
def get_install_update():
    ip        = request.args.get('ip',        None)
    config    = request.args.get('config',    None)
    interface = request.args.get('interface', None)
    if ip == None or config == None or interface == None:
        return render_template('update.sh')
    else:
        return 'ddopsnd_core.sh ' + interface + ' ' + ip + ' ' + config


def get_file_or_param(name, decode='utf-8'):
    rt = None
    if request.files.get(name, None) != None:
        rt = request.files[name].read()
    elif request.form.get(name, None) != None:
        rt = request.form[name]
    elif request.args.get(name, None) != None:
        rt = request.args[name]
    else:
            abort(400, 'Need ' + name)
    if type(rt) == bytes:
        rt = rt.decode(decode)
    return rt


def get_record_json():
    record = get_file_or_param('record')
    return json.loads(record)

def unwords(vs):
    rt = ''
    for v in vs:
        if type(v) != str:
            rt += str(v) + ' '
        else:
            rt += v + ' '
    return rt

def unlines(vs):
    rt = ''
    for v in vs:
        if type(v) != str:
            rt += str(v) + '\n'
        else:
            rt += v + '\n'
    return rt