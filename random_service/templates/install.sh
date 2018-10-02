#.sh

mkdir -p /etc/ddopsndd
curl {{request.url_root}}static/dns_core.sh > /bin/ddopsndd_cor.sh
curl {{request.url_root}}install/default > /etc/ddopsndd/default
echo "Input token"
read token
echo $token > /etc/ddopsndd/token