# !/bin/bash
set -e

# Execute py src
tini -- /usr/local/random_service_py/random_service

# Execute rs src
exec tini -- /usr/local/bin/random-service-rs