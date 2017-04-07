# Run tests using checkbox
set -e

# Check if checkbox-converged QML file is available
QML="$PWD/checkbox-converged/checkbox-converged.qml"
if [ ! -f "$QML" ]
then
    echo "$0: checkbox-converged not found" >&2
    echo "    Please try running deps.sh" >&2
    exit 1
fi

# Check if provider is available
if [ -f "provider/units/$1.pxu" ]
then
  PLAN="$1"
else
  echo "$0 <plan> [model]" >&2
  for plan_file in provider/units/*.pxu
  do
    plan="$(basename "$plan_file" .pxu)"
    echo "    $plan" >&2
  done
  exit 1
fi

# Update provider information
cd provider
python3 manage.py develop -f
cd ..

# Create report directory
REPORT="report/$PLAN/"
mkdir -p "$REPORT"
cd "$REPORT"

# Create launcher
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
type = converged

[exporter:html]
unit = 2013.com.canonical.plainbox::html

[exporter:json]
unit = 2013.com.canonical.plainbox::json

[exporter:tar]
unit = 2013.com.canonical.plainbox::tar

[transport:html]
type = file
path = report.html

[transport:json]
type = file
path = report.json

[transport:tar]
type = file
path = report.tar

[report:html]
exporter = html
transport = html
forced = yes

[report:json]
exporter = json
transport = json
forced = yes

[report:tar]
exporter = tar
transport = tar
forced = yes
EOF

# Run checkbox-converged with the launcher
qmlscene "$QML" --launcher=launcher.conf || true

if [ -n "$2" ]
then
  SERVER="http://10.17.75.229:8000"
  MODEL="$2"
  TEST="$(date "+%F_%T")_$PLAN"
  echo "Uploading report.json to $SERVER/$MODEL/$TEST"
  curl -F "model=$MODEL" -F "test=$TEST" -F "file=@report.json" "$SERVER/upload"
fi
