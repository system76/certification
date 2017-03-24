# Run tests using checkbox
set -e

cd provider
./manage.py develop -f
cd ..

PLAN=manual
REPORT="report/$PLAN/"
mkdir -p "$REPORT"
cd "$REPORT"

if [ "$PLAN" == "manual" ]
then
  UI=converged
else
  UI=silent
fi

cat > launcher.conf <<EOF
[launcher]
launcher_version = 1
app_id = 2017.com.system76:launcher
stock_reports = text

[test plan]
unit = 2017.com.system76::$PLAN
forced = yes

[test selection]
forced = yes

[ui]
type = $UI

[exporter:html]
unit = 2013.com.canonical.plainbox::html

[exporter:tar]
unit = 2013.com.canonical.plainbox::tar

[transport:html]
type = file
path = report.html

[transport:tar]
type = file
path = report.tar

[report:html]
exporter = html
transport = html
forced = yes

[report:tar]
exporter = tar
transport = tar
forced = yes
EOF

checkbox-cli launcher.conf
