# Run tests using checkbox
set -e

cd provider
./manage.py develop -f
cd ..

mkdir -p report
checkbox-cli launcher/automated.conf
