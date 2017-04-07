# Install dependencies, to be moved into debian/control
set -e

sudo add-apt-repository -y -u ppa:hardware-certification/public
sudo apt install -y checkbox-ng checkbox-converged \
                    plainbox-provider-certification-client \
                    net-tools

git submodule update --init --remote
