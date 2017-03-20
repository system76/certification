# Run tests using checkbox
set -e

cd provider
./manage.py develop -f
cd ..

JOB=automated
DATE="$(date "+%Y-%m-%d_%H-%M-%S")"
REPORT="report/$JOB/"
mkdir -p "$REPORT"
cd "$REPORT"

cat << EOF > launcher.conf
[launcher]
launcher_version = 1
app_id = 2017.com.system76:launcher
stock_reports = text

[test plan]
unit = 2017.com.system76::$JOB
forced = yes

[test selection]
forced = yes

[ui]
type = silent

[exporter:html]
unit = 2013.com.canonical.plainbox::html

[exporter:json]
unit = 2013.com.canonical.plainbox::json

[exporter:text]
unit = 2013.com.canonical.plainbox::text

[transport:html]
type = file
path = report.html

[transport:json]
type = file
path = report.json

[transport:text]
type = file
path = report.txt

[report:html]
exporter = html
transport = html
forced = yes

[report:json]
exporter = json
transport = json
forced = yes

[report:text]
exporter = text
transport = text
forced = yes
EOF

checkbox-cli launcher.conf
