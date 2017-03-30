# Run tests using checkbox
set -e

cd provider
python3 manage.py develop -f
cd ..

if [ -e "provider/units/$1.pxu" ]
then
  PLAN="$1"
else
  echo "$0 [plan]"
  for plan_file in provider/units/*.pxu
  do
    plan="$(basename "$plan_file" .pxu)"
    echo "    $plan"
  done
  exit 1
fi

REPORT="report/$PLAN/"
mkdir -p "$REPORT"
cd "$REPORT"

if [ "$PLAN" == "automated" ]
then
  UI=silent
else
  UI=converged
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

QT_AUTO_SCREEN_SCALE_FACTOR=1 checkbox-cli launcher.conf
