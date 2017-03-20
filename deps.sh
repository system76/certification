# Install dependencies, to be moved into debian/control
set -e
sudo add-apt-repository ppa:hardware-certification/public
sudo apt update
sudo apt install checkbox-ng plainbox-provider-checkbox fswebcam fwts iperf