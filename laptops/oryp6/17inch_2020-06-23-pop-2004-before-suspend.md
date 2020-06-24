## Test Platform

| Test complete | OS Version     | BIOS Version       | EC Version         | Before or after suspend |
| ------------- | -------------- | ------------------ | ------------------ | ----------------------- |
| INCOMPLETE    | Pop!\_OS 20.04 | 2020-06-24_5c24b6f | 2020-06-24_5c24b6f | Before                  |

## Checklist
x = pass | blank = fail | na = remove from list

## Hot Keys

Note: display toggle hotkey is in the displays section below.

- [x] Touchpad Lock
- [x] Turn off display
- [x] Mute sound
- [x] Volume down
- [x] Volume up
- [x] Brightness Down
- [x] Brightnes Up
- [x] Camera on/off
- [x] Airplane Mode
- [x] Suspend
- [x] Play/Pause
- [x] Keyboard backlight on/off
- [x] Keyboard backlight brightness up
- [x] Keyboard backlight brightness down
- [x] Keyboard backlight toggle colors

### Hot key notes and issues

- No notes

## Touchpad

- [x] Touchpad two finger scrolling 
- [x] Tap to click
- [x] Left click
- [x] Right click (physical button and two-finger tap)

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [x] Left USB Type A
- [x] Right USB Type A (back)
- [x] Right USB Type A (front)
- [x] Back USB Type C
- [x] Micro-SD card slot
- [x] Headphone jack
- [x] Microphone jack
- [x] Combo jack headphones
- [x] Combo jack mic
- [ ] Internal mic
- [x] Internal speakers
- [x] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- Internal mic and internal speakers don't work on Open Firmware. Don't have a MicroSD card to test with.

## Displays

- [x] HDMI port
- [x] HDMI port audio
- [x] Mini display port
- [x] Mini display port audio
- [x] Type C DP
- [x] Type C DP audio
- [x] Dual external display + internal
- [x] Close internal display. Monitors adjust correctly.
- [ ] Reboot w/ lid closed. Decryption dialog shows on external display.
- [x] Triple external display via DP daisy chain + internal

### Displays notes and issues

- Rebooting currently does not work, so not able to test rebooting w/ lid closed.

## Network and bluetooth

- [x] Wifi connects with expected performance
- [x] Ethernet connects with expected performance
- [x] Bluetooth speaker
- [x] Lock machine

### Network and bluetooth

- Don't have a Bluetooth speaker to test with. Bluetooth mouse does work.

## Suspend

- [x] Close lid - system suspends
- [x] 150 suspend successful (in NVIDIA mode)
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- Resume from suspend works while plugged into AC adapter on fresh boot. When on battery, system powers off just after resuming, and does not power back on until AC adapter is plugged in again.
