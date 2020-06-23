## Test Platform

| Test complete | OS Version    | BIOS Version | EC Version | Before or after suspend |
| ------------- | ------------- | ------------ | ---------- | ----------------------- |
| INCOMPLETE | Pop!\_OS 20.04 | 2020-06-19_bf83c07 | 2020-06-19_bf83c07 | Before      |

## Checklist
x = pass | blank = fail | na = remove from list

## Hot Keys

Note: display toggle hotkey is in the displays section below.

- [x] Touchpad Lock
- [X] Turn off display
- [X] Mute sound
- [X] Volume down
- [X] Volume up
- [X] Brightness Down
- [X] Brightnes Up
- [X] Camera on/off
- [X] Airplane Mode
- [X] Suspend
- [X] Play/Pause
- [X] Keyboard backlight on/off
- [X] Keyboard backlight brightness up
- [X] Keyboard backlight brightness down
- [X] Keyboard backlight toggle colors

### Hot key notes and issues

- No notes

## Touchpad

- [x] Touchpad two finger scrolling 
- [X] Tap to click
- [X] Left click
- [X] Right click (physical button and two-finger tap)

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [X] Left USB Type A
- [X] Right USB Type A (back)
- [X] Right USB Type A (front)
- [X] Back USB Type C
- [ ] Micro-SD card slot
- [X] Headphone jack
- [X] Microphone jack
- [X] Combo jack headphones
- [X] Combo jack mic
- [ ] Internal mic
- [ ] Internal speakers
- [ ] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- Internal mic and internal speakers don't work on Open Firmware. Don't have a MicroSD card to test with.

## Displays

- [x] HDMI port
- [X] HDMI port audio
- [X] Mini display port
- [X] Mini display port audio
- [X] Type C DP
- [X] Type C DP audio
- [X] Dual external display + internal
- [X] Close internal display. Monitors adjust correctly.
- [ ] Reboot w/ lid closed. Decryption dialog shows on external display.
- [X] Triple external display via DP daisy chain + internal

### Displays notes and issues

- Rebooting currently does not work, so not able to test rebooting w/ lid closed.

## Network and bluetooth

- [X] Wifi connects with expected performance
- [X] Ethernet connects with expected performance
- [ ] Bluetooth speaker
- [X] Lock machine

### Network and bluetooth

- Don't have a Bluetooth speaker to test with. Bluetooth mouse does work.

## Suspend

- [ ] Close lid - system suspends
- [X] 150 suspend successful (in NVIDIA mode)
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- Lid close/open: works while plugged into AC adapter on fresh boot. When on battery, system powers off just after resuming, and does not power back on until AC adapter is plugged in again.

- Depleted battery: system powered itself off entirely. journalctl shows that it tried to enter Suspend+Hibernate.

- `sudo systemctl suspend` sometimes shows the same issue where the system powers off when attempting to resume (from a fresh boot on battery power, it takes two tries to see that issue occur.)
