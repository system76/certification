# Run tests using checkbox
set -e

# Check if checkbox-converged QML file is available
QML="$PWD/checkbox-converged/checkbox-converged.qml"
if [ ! -f "$QML" ]
then
    git submodule update --init --remote
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

# Create test directory
TEST="$(date "+%F_%T")_$PLAN"
rm -rf "tests/$TEST/"
mkdir -p "tests/$TEST/"
cd "tests/$TEST/"

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
type = interactive

[exporter:html]
unit = com.canonical.plainbox::html

[exporter:json]
unit = 2017.com.system76::json_patched

[exporter:tar]
unit = com.canonical.plainbox::tar

[transport:html]
type = file
path = report.html

[transport:json]
type = file
path = report.json

[transport:tar]
type = file
path = report.tar.xz

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
sudo checkbox-cli launcher launcher.conf || true

jsonlint-php report.json

if [ -n "$2" ]
then
  SERVER="http://10.17.75.78:8000"
  MODEL="$2"
  echo "Uploading report.json to $SERVER/$MODEL/$TEST"
  curl -F "model=$MODEL" -F "test=$TEST" -F "file=@report.json" "$SERVER/upload"
fi
