# Install dependencies, to be moved into debian/control
set -e
FIND='deb http:\/\/ppa.launchpad.net\/hardware-certification\/public\/ubuntu cosmic main'
REPLACE='deb http:\/\/ppa.launchpad.net\/hardware-certification\/public\/ubuntu bionic main'

sudo add-apt-repository -y ppa:hardware-certification/public
if  [ -f /etc/apt/sources.list.d/hardware-certification-ubuntu-public-cosmic.list ]
then
    sudo sed -i -e 's/'"$FIND"'/'"$REPLACE"'/g' /etc/apt/sources.list.d/hardware-certification-ubuntu-public-cosmic.list
fi

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
    wmctrl \
    gnome-sound-recorder \
    audacity \
    cheese
# Added both gnome-sound-recorder and audacity because of bug found on leow9
# while using Audacity
# cheese only needed on laptops
    
sudo snap install core
