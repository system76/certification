## Test Platform

| Test complete | OS Version     | BIOS Version | EC Version |
| ------------- | -------------- | ------------ | ---------- |
| INCOMPLETE    | Pop!\_OS 22.04 | XYZ          | 123        |

## Checklist
x = pass | blank = fail | na = remove from list

## Hot Keys

Note: display toggle hotkey is in the displays section below.

- [ ] Touchpad Lock
- [ ] Turn off display
- [ ] Mute sound
- [ ] Volume down
- [ ] Volume up
- [ ] Brightness Down
- [ ] Brightnes Up
- [ ] Camera on/off
- [ ] Airplane Mode
- [ ] Suspend
- [ ] Play/Pause
- [ ] Keyboard backlight on/off
- [ ] Keyboard backlight brightness up
- [ ] Keyboard backlight brightness down
- [ ] Keyboard backlight toggle colors

### Hot key notes and issues

- No notes

## Power button

- [ ] External power button is disabled when the lid is closed, the machine is powered off, and not connected to a charger (barrel or USB-C)
- [ ] External power button is disabled when the lid is closed, the machine is suspended, and not connected to a charger (barrel or USB-C)
- [ ] External power button turns on the machine when the lid is closed and the barrel charger is connected
- [ ] External power button turns on the machine when the lid is closed and a USB-C charger is connected
- [ ] External power button wakes the machine from suspend when the lid is closed and the barrel charger is connected
- [ ] External power button wakes the machine from suspend when the lid is closed and a USB-C charger is connected

## Power button notes and issues

- No notes

## Touchpad

- [ ] Touchpad two finger scrolling
- [ ] Tap to click
- [ ] Left click
- [ ] Right click
- [ ] Middle click (three finger tap/click for clickpads, click both buttons simultaneously for non-clickpad)
- [ ] Four finger swipe up or down changes workspaces

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [ ] Left USB Type A
- [ ] Right USB Type A
- [ ] Back USB Type A
- [ ] Left USB Type C
- [ ] Right USB Type C
- [ ] Back USB Type C
- [ ] Thunderbolt devices plugged in before suspend do not block suspend
- [ ] Thunderbolt devices plugged in during suspend do not block resume
- [ ] SD card slot
- [ ] Kensington lock slot
- [ ] Headphone jack
- [ ] Microphone jack
- [ ] Combo jack headphones
- [ ] Combo jack mic
- [ ] Internal mic
- [ ] Internal speakers
- [ ] spdif jack
- [ ] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- No notes

## Displays

- [ ] HDMI port
- [ ] HDMI port audio
- [ ] Mini display port
- [ ] Mini display port audio
- [ ] Type C DP
- [ ] Type C DP audio
- [ ] Dual external display + internal
- [ ] Close internal display. Monitors adjust correctly.
- [ ] Reboot w/ lid closed. Decryption dialog shows on external display.
- [ ] Triple external display via DP daisy chain + internal
- [ ] Laptop boots and logs in to desktop with HDMI monitor connected
- [ ] Laptop boots and logs in to desktop with DP/USB-C monitor connected

### Displays notes and issues

- No notes

## Network and bluetooth

- [ ] Wifi connects with expected performance
- [ ] Ethernet connects with expected performance
- [ ] Bluetooth speaker
- [ ] Lock machine

### Network and bluetooth

- No notes

## Suspend & Power

- [ ] Close lid - system suspends
- [ ] 150 suspend successful
- [ ] Perform 20 manual suspends-- keyboard/touchpad are functional on every resume.
- [ ] System stays suspended for at least 15 minutes on battery power, then resumes as expected afterwards
- [ ] Power on while unplugged & run stress command.
- [ ] Power on while unplugged & build the Linux kernel.
- [ ] Laptop charges to full as expected (drain it to ~25% before charging)
- [ ] Laptop reaches at least a C8 power state after idling for ~10 minutes (in `powertop`, in the "Idle stats" tab, the `Pkg(HW)` column should have a C8 percentage >0%)

### Suspend notes and issues

- No notes

## Hardware compatibility

- [ ] RAM
    - [ ] System boots and suspends/resumes with both the minimum and maximum supported amounts of RAM/DIMMs
    - [ ] Speed of highest frequency RAM supported is correctly reported in `dmidecode -t 17`
- [ ] Storage
    - [ ] System boots and suspends/resumes with all possible SATA drives populated
    - [ ] System boots and suspends/resumes with each of the following drive models (test each one in every possible slot):
        - [ ] Samsung 970 Evo Plus (PCIe Gen 3)
        - [ ] Samsung 980 Pro (PCIe Gen 4)
        - [ ] Western Digital Blue SN550 (PCIe Gen 3)
        - [ ] Any M.2 SATA drive
    - [ ] Read and write speeds are within expectations with each of the following drive models (test each one in every possible slot):
        - [ ] Samsung 970 Evo Plus (PCIe Gen 3)
        - [ ] Samsung 980 Pro (PCIe Gen 4)
        - [ ] Western Digital Blue SN550 (PCIe Gen 3)
        - [ ] Any M.2 SATA drive
- [ ] TPM
    - [ ] `/sys/class/tpm/tpm0` directory exists
    - [ ] No TPM errors in dmesg & journalctl logs after 150 suspends
- [ ] KVM `/dev/kvm` file exists

### Hardware compatibility notes and issues

- No notes
