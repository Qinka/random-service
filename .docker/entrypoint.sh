# !/bin/bash
set -e

# Execute py src
/usr/local/random_service_py/random_service --config /etc/random-service/config.yaml &

# Execute rs src
exec /usr/local/bin/random-service-rs --config /etc/random-service/config.yaml
