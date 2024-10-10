# Test Platform

| Test complete | OS Version     | BIOS Version | Before or after suspend |
| ------------- | -------------- | ------------ | ----------------------- |
| INCOMPLETE    | Pop!\_OS 22.04 | F2           | Before                  |

## Checklist

x = pass | blank = fail | na = remove from list

## Ports (Non Display Related)

- [x] USB 3 Type A ports
- [x] USB Type C port(s)
- [x] Line out jack
- [x] Microphone in jack
- [x] Front IO ports:
  - [x] USB-A
  - [x] USB-C
  - [x] Mic in
  - [x] Line out

### Ports notes and issues

- No notes

## Displays and graphics

- [x] Type C DP
- [x] Type C DP audio
- [x] Display config toggle (Super + P)
- [x] Discrete GPU DisplayPort Passthrough
  - [x] No visible issues playing youtube videos
- [x] Discrete AMD graphics card
  - [x] No visible issues playing youtube videos
- [x] Discrete Nvidia graphics card
  - [x] No visible issues playing youtube videos
- [x] Discrete Intel graphics card
  - [x] No visible issues playing youtube videos
- [x] Multiple GPUs (max loadout, if possible)

### Displays notes and issues

- No notes

## Network and bluetooth

- [x] Wifi connects with expected performance
- [x] Ethernet connects with expected performance
- [x] Bluetooth speaker
- [x] Lock machine

### Network and bluetooth notes and issues

- No notes

## Hardware and BIOS settings

- [x] RAM
    - [x] System boots and suspends/resumes with both the minimum and maximum supported amounts of RAM/DIMMs
    - [x] XMP profiles work on maximum speed and capacity supported RAM config (that we will offer)
- [x] TPM
    - [x] `/sys/class/tpm/tpm0` directory exists
    - [x] No TPM errors in dmesg & journalctl logs after 150 suspends
- [x] KVM `/dev/kvm` file exists

### Hardware and BIOS settings notes and issues

- No notes

## Suspend

- [x] 150 suspend successful

### Suspend notes and issues

- System includes Qualcomm WiFi 7 adapter which has suspend issues with linux kernels older than 6.10
- With linux 6.10.9 kernel installed while I have 3 4090s installed, with nvidia-driver-555, the system does suspend and resume according to journal, but display does not work after suspend.
- With linux 6.11 kernel installed and 2 AMD Radeon Pro GPUs (W7900 & W7800) suspend works correctly.
- With linux 6.10.9 kernel installed and 2 AMD Radeon Pro GPUs (W7900 & W7800) it seems like suspend may work correctly. Does not seem as though issue with 3 Nvidia GPUs was the kernel.
- With linux 6.10.9 kernel installed and 3 Nvidia 4090 GPUs installed, with nvidia-driver-560, system does suspend and resume according to the journal, but display does not work after suspend. 
- suspend completely working after building linux 6.10+/6.11+ then installing nvidia drivers from nvidia sources
