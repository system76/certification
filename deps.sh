# Install dependencies, to be moved into debian/control
set -e

sudo add-apt-repository -y ppa:hardware-certification/public
sudo apt update -y
sudo apt install -y \
    checkbox-ng \
    plainbox-provider-checkbox \
    fswebcam \
    fwts \
    iperf \
    jsonlint \
    net-tools \
    obexftp \
    snapd \
    wmctrl
sudo snap install core
