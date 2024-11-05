## Test Platform

| Test complete | OS Version    | BIOS Version | Before or after suspend |
| ------------- | ------------- | ------------ | ----------------------- |
| INCOMPLETE    |               |              |                         |

## Checklist
x = pass | blank = fail | na = remove from list

## Ports (Non Display Related)

- [ ] USB 3 Type A ports
- [ ] USB 2 Type A ports
- [ ] USB Type C port(s)
- [ ] Thunderbolt devices plugged in before suspend do not block suspend
- [ ] Thunderbolt devices plugged in during suspend do not block resume
- [ ] Line out jack
- [ ] Line in jack
- [ ] Microphone in jack
- [ ] Internal mic (meerkat)
- [ ] spdif jack
- [ ] Front IO ports:
  - [ ] USB-A
  - [ ] USB-C
  - [ ] Mic in
  - [ ] Line out

### Ports notes and issues

- No notes

## Displays and graphics

- [ ] HDMI port
- [ ] HDMI port audio
- [ ] Display port
- [ ] Display port audio
- [ ] Type C DP
- [ ] Type C DP audio
- [ ] Display config toggle (Super + P)
- [ ] Integrated graphics (AMD / Intel) 
  - [ ] No visible issues playing youtube videos
- [ ] Discrete AMD graphics card
  - [ ] No visible issues playing youtube videos
- [ ] Discrete Nvidia graphics card
  - [ ] No visible issues playing youtube videos
- [ ] Discrete Intel graphics card
  - [ ] No visible issues playing youtube videos
- [ ] Multiple GPUs (max loadout, if possible)

### Displays notes and issues

- No notes

## Network and bluetooth

- [ ] Wifi connects with expected performance
- [ ] Ethernet connects with expected performance
- [ ] Bluetooth speaker
- [ ] Lock machine

### Network and bluetooth notes and issues

- No notes

## Hardware and BIOS settings

- [ ] RAM
    - [ ] System boots and suspends/resumes with both the minimum and maximum supported amounts of RAM/DIMMs
    - [ ] XMP profiles work on maximum speed and capacity supported RAM config (that we will offer)
- [ ] TPM
    - [ ] `/sys/class/tpm/tpm0` directory exists
    - [ ] No TPM errors in dmesg & journalctl logs after 150 suspends
- [ ] KVM `/dev/kvm` file exists

### Hardware and BIOS settings notes and issues

- No notes

## Suspend

- [ ] 150 suspend successful

### Suspend notes and issues

- No notes

