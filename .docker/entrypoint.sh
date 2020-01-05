# !/bin/bash
set -e

# Execute py src
/usr/bin/tini -- /usr/local/random_service_py/random_service --config /etc/random-service/config.yaml

# Execute rs src
exec /usr/bin/tini -- /usr/local/bin/random-service-rs --config /etc/random-service/config.yaml
