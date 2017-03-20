# Run tests using checkbox
set -e

cd provider
./manage.py develop -f
cd ..

checkbox-cli launcher/automated.conf
