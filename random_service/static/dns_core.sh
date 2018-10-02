#!/bin/sh

# Error
set -e

echo $#
if [ $# -lt 3 ]; then
    echo "USAGE: $0 interface 4/6 config.file"
    echo "e.g.: $0 eth0 4 /etc/ddopsnd/cfg"
    echo "      $0 eth0 6 /etc/ddopsnd/cfg"
    exit 1;
elif [ $# -lt 4 ]; then
    K=2
else
    K=$4
fi

# functions
help()
{
    echo "USAGE: $0 interface 4/6 config.file"
    echo "e.g.: $0 eth0 4 /etc/ddopsnd/cfg"
    echo "      $0 eth0 6 /etc/ddopsnd/cfg"

}

function info(){
    if [ -n "$DEBUG" ]; then
        echo $@
    fi
}


USR_AGENT="ddnspod/0.1 me@qinka.pro"
info USR_AGENT $USRAGENT

#args
# $1 for ifconfig's interface
INTERFACE=$1
info INTERFACE $INTERFACE
# $2 for ipv4 or ipv6
if   [ "$2" = "4" ]; then
    IP=4
    IPv4="true"
    info IPv4 $IPv4
elif [ "$2" = "6" ]; then
    IP=6
    IPV6="true"
    info IPv6 $IPv6
else
    help
    exit 2;
fi
info IP $IP
# $3 for config file
CONFIG=$3
info CONFIG $CONFIG
source $CONFIG


##  check
### check login token
if [ -z "$LOGIN_TOKEN" ]; then
    echo "need login token"
    exit 3;
fi
info LOGIN_TOKEN $LOGIN_TOKEN
### check format
if [ -z "$FORMAT" ]; then
    FORMAT="json"
fi
FORMAT="json" ## force
info FORMAT $FORMAT
### check lang
if [ -z "$REQ_LANG" ]; then
    REQLANG="cn"
fi
info REQLANG $REQLANG
### check domain
if [ -z "$DOMAIN" ]; then
    echo "need domain"
    exit 4;
fi
info DOMAIN $DOMAIN
### check sub domain
if [ -z "$SUB_DOMAIN" ]; then
    echo "need sub domain"
    exit 5;
fi
info SUB_DOMAIN $SUB_DOMAIN
### check record line
if [ -z "$RECORD_LINE" ]; then
    RECORD_LINE="默认"
fi
info RECORD_LINE $RECORD_LINE
### check ddopsndd url
if [ -z "$DDOPSNDD" ]; then
    DDOPSNDD="localhost:3000"
fi
info DDOPSNDD $DDOPSNDD
### check ttl
if [ -z "$TTL"  ]; then
    TTL=600
fi
info TTL $TTL
##  generate
### generate value
VALUE_IFC=`ifconfig $INTERFACE | grep inet`
info VALUE_IFC $VALUE_IFC
VALUE=`curl -X POST $DDOPSNDD/ipaddr\?ip=v$IP -F "context=$VALUE_IFC" | awk '{print $"'$K'"}'`
info VALUE $VALUE
### generate record type
if [ "$IP" = "4" ];then
    RECORD_TYPE="A"
else
    RECORD_TYPE="AAAA"
fi
info RECORD_TYPE $RECORD_TYPE
### Get records
RECORD_CURL=`curl -X POST https://dnsapi.cn/Record.List -F "login_token=$LOGIN_TOKEN" -F "format=$FORMAT" -F "lang=$REQLANG" -F "domain=$DOMAIN"`
info RECORD_CURL $RECORD_CURL
### Delete records
DEL_RECORED_PARAM=`curl -X POST $DDOPSNDD/record\?field=id\&name=$SUB_DOMAIN\&type=$RECORD_TYPE\&split=inet -F "record=$RECORD_CURL"`
info DEL_RECORED_PARAM $DEL_RECORED_PARAM
if [ -n "$DEL_RECORED_PARAM" ]; then
    DEL_RT=`echo $DEL_RECORED_PARAM | tr " " "\n"| awk "{print \"curl -X POST https://dnsapi.cn/Record.Remove -F login_token=$LOGIN_TOKEN -F format=$FORMAT -F lang=$REQLANG -F record_id=\""'$1'"\" -F domain=$DOMAIN \"}" `
    info DEL_RT $DEL_RT
else
    echo No old record
fi
RT=`curl -X POST https://dnsapi.cn/Record.Create -F "login_token=$LOGIN_TOKEN" \
                                                 -F "format=$FORMAT" \
                                                 -F "domain=$DOMAIN" \
                                                 -F "sub_domain=$SUB_DOMAIN" \
                                                 -F "value=$VALUE" \
                                                 -F "record_type=$RECORD_TYPE" \
                                                 -F "record_line=$RECORD_LINE" \
                                                 -F "ttl=$TTL"`
echo RT