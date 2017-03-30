# Install dependencies, to be moved into debian/control
set -e

sudo add-apt-repository -u ppa:hardware-certification/public
sudo apt install checkbox-ng checkbox-converged plainbox-provider-certification-client

git submodule update --init --remote
