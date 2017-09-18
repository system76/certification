# Install dependencies, to be moved into debian/control
set -e

sudo add-apt-repository -y ppa:hardware-certification/public
sudo apt update -y
sudo apt install -y checkbox-ng checkbox-converged \
                    plainbox-provider-certification-client \
                    net-tools snapd
sudo snap install core
