## meer10-b (core ultra 3)

## Test Platform

| Test complete | OS Version    | BIOS Version | Before or after suspend |
| ------------- | ------------- | ------------ | ----------------------- |
| COMPLETE      | ubuntu 24.04  | CRRPLR30.0008.2025.0225.1349 | both    |

## Checklist
x = pass | blank = fail | na = remove from list

## Ports (Non Display Related)

- [x] USB 3 Type A ports
- [x] USB Type C port(s)
- [x] Thunderbolt devices plugged in before suspend do not block suspend
- [x] Thunderbolt devices plugged in during suspend do not block resume
- [x] Front IO ports:
  - [x] USB-A
  - [x] USB-C

### Ports notes and issues

- No headphone/mic input/output jack

## Displays and graphics

- [x] HDMI port
- [x] HDMI port audio
- [x] Type C DP
- [x] Type C DP audio
- [x] Display config toggle (Super + P)
- [x] Integrated graphics (AMD / Intel) 
  - [x] No visible issues playing youtube videos

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
- [x] TPM
    - [x] `/sys/class/tpm/tpm0` directory exists
    - [x] No TPM errors in dmesg & journalctl logs after 150 suspends
- [x] KVM `/dev/kvm` file exists

### Hardware and BIOS settings notes and issues

- No notes

## Suspend

- [x] 150 suspend successful

### Suspend notes and issues

- No notes

