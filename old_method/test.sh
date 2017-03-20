#install packages
sudo apt install -y mesa-utils phoronix-test-suite xbacklight glmark2 xsel stress-ng fwts powerstat powertop memtester smartmontools exfat-utils speedtest-cli
phoronix-test-suite install unigine xonotic tesseract gputest unigine-valley opengl-demos opengl-workstation opencl

sudo add-apt-repository ppa:varlesh-l/indicator-kdeconnect
sudo apt update
sudo apt install indicator-kdeconnect kdeconnect #Share clipboard

#check Ubuntu version
lsb_release -d | xsel -ib

#copy lspci to clipboard
lspci | xsel -ib

# copy lsusb to clipboard
lsusb | xsel -ib

#copy dmidecode to clipboard
sudo dmidecode | xsel -ib

# Check BIOS DMI information
sudo dmidecode | grep -P "(r: Sy|t Na|sion:)"

# Check hostname
hostname

# Check location
cat /etc/timezone

#check touchpad lock

# Play a video
amixer -D pulse sset Master 30%
pacmd list-cards
pactl set-card-profile 0 output:hdmi-stereo-extra1
pactl set-card-profile 0 output:analog-stereo
totem /usr/share/example-content/Ubuntu_Free_Culture_Showcase/Nathan\ Haines\ -\ Ubuntu\ Through\ The\ Years.ogg &

# Check brightness
# nVidia dropped support for /sys/class/backlight
watch -n .2 xbacklight
cat /sys/class/backlight/acpi_video0/brightness

# Check webcam
if [ -c /dev/video0 ]; then echo "webcam on"; else echo "webcam off"; fi
watch -n 1 lsusb

# Check airplane mode
watch -n 1 rfkill list # both
hciconfg # bluetooth
nmcli d # wifi

# check resolution and refresh rate
xrandr

# Check 3d acceleration
sudo add-apt-repository ppa:graphics-drivers/ppa

phoronix-test-suite benchmark unigine-valley
phoronix-test-suite default-benchmark openarena xonotic tesseract gputest unigine-valley
phoronix-test-suite default-benchmark opengl-demos opengl-workstation opencl
glxgears -info
glmark2 --fullscreen # glmark2 will crash nVidia hardware

# detect battery
upower -e
upower -i /org/freedesktop/UPower/devices/battery_BAT0

# check suspend
sudo pm-suspend

# watch current CPU frequency
watch -n .5 sudo cat /sys/bus/cpu/devices/cpu*/cpufreq/scaling_cur_freq

# test scaling of CPU.
stress-ng --cpu 8 # 8 for quad core, 4 for duo
stress-ng --cpu 4 --io 2 --vm 1 --vm-bytes 1G --timeout 60s --metrics-brief
stress-ng --cpu 4 --io 4 --vm 2 --vm-bytes 2G --metrics-brief

# test microphone
arecord -d 10 -f cd -t wav test.wav
aplay test.wav

#check NIC working
speedtest

#test settings on touchpad
gsettings set org.gnome.desktop.peripherals.touchpad tap-to-click true
gsettings set org.gnome.desktop.peripherals.touchpad natural-scroll false
gsettings set org.gnome.desktop.peripherals.touchpad scroll-method 'two-finger-scrolling'
gsettings set org.gnome.desktop.peripherals.touchpad scroll-method 'edge-scrolling'

#check horizontal scrolling
firefox http://www.californiabeaches.com/wp-content/uploads/2014/09/Big_Sur_June_20081.jpg

# change brightness and lock timeout to 10 seconds
gsettings set org.gnome.desktop.session idle-delay 10
gsettings set org.gnome.desktop.session idle-delay 300

# check cpu freqency driver
watch -n .5 cat /sys/bus/cpu/devices/cpu*/cpufreq/scaling_driver

# check cpu frequency governor
watch -n .5 cat /sys/bus/cpu/devices/cpu*/cpufreq/scaling_governor

# check wifi and nic hardware
ifconfig

# check suspend for instability
sudo fwts s3 --s3-multiple=150 --s3-min-delay=18 --s3-max-delay=20

#check power usage
powerstat -Ra 5 12

#run a memtest in user space
free -m
sudo swapoff -a
memtester 2500M 1
sudo swapon -a

#check smart status of the drive
sudo smartctl -a /dev/sda

#install CUDA 8
sudo dpkg -i ./cuda-repo-ubuntu1604-8-0-rc_8.0.27-1_amd64.deb
sudo apt update
sudo apt install cuda-8-0
sudo nano /usr/local/cuda-8.0/include/host_config.h  # line 115

#keyboard ghosting
www.microsoft.com/appliedsciences/KeyboardGhostingDemo.mspx

#nVidia info
nvidia-smi -q
nvidia-smi -L

#nVidia driver
sudo add-apt-repository ppa:graphics-drivers/ppa
sudo apt update
sudo apt install nvidia-367

#Checkbox software, full test suite
plainbox-provider-checkbox
qmlscene --settings=/usr/share/checkbox-converged/settings.json $@ /usr/share/checkbox-converged/checkbox-touch.qml

#Nvidia CUDA 8
http://developer.download.nvidia.com/compute/cuda/8.0/secure/rc1/local_installers/cuda-repo-ubuntu1604-8-0-rc_8.0.27-1_amd64.deb?autho=1472063115_0bfc244837f383fceb8309b92c74f6c6&file=cuda-repo-ubuntu1604-8-0-rc_8.0.27-1_amd64.deb

#Set HD password on self-encrypting drives
sudo hdparm --user-master u --security-set-pass 123456 /dev/sdb

For airplane mode:
sudo nano /etc/system76-daemon.json
{"model": "kudp1"}
sudo systemctl restart system76-driver
system76-driver --model kudp1

If OEM-firstboot isn't showing on desktop:
sudo apt install ubiquity-frontend-gtk oem-config-gtk grub-efi
sudo oem-config-prepare
Watch for acpi events:
acpi_listen
