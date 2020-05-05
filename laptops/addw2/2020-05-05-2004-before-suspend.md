## Test Platform

| Test complete | OS Version    | BIOS Version | EC Version | Before or after suspend |
| ------------- | ------------- | ------------ | ---------- | ----------------------- |
| COMPLETE      | Pop!\_OS 20.04 | 1.07.01     | 1.07.01    | Before                  |

## Checklist
x = pass | blank = fail | na = remove from list

## Hot Keys

Note: display toggle hotkey is in the displays section below.

- [X] Touchpad Lock
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

- [X] Touchpad two finger scrolling
- [X] Tap to click
- [X] Left click
- [X] Right click

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [X] Left USB Type A
- [X] Right USB Type A
- [X] Right USB Type C
- [X] SD card slot
- [X] Headphone jack
- [X] Microphone jack
- [X] Combo jack headphones
- [X] Combo jack mic
- [X] Internal mic
- [X] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- When trying to boot from external device:
  - M.2 SSD enclosure does not show up as bootable when plugged into the right USB 3 type A port.

## Displays

- [ ] HDMI port
- [ ] HDMI port audio
- [ ] Display config toggle (span/cloned/internal/external)
- [ ] Mini display port
- [ ] Mini display port audio
- [ ] Display config toggle (span/cloned/internal/external)
- [ ] Type C DP
- [ ] Type C DP audio
- [ ] Display config toggle (span/cloned/internal/external)
- [ ] Dual external display + internal
- [ ] Dual external display + internal
- [ ] Triple external display via DP daisy chain + internal
- [ ] Close internal display. Monitors adjust correctly.
- [ ] Reboot w/ lid closed. Decryption dialog shows on external display.

### Displays notes and issues

- No notes

## Network and bluetooth

- [X] Wifi connects with expected performance
- [ ] Ethernet connects with expected performance
- [ ] Bluetooth speaker
- [ ] Bluetooth mic
- [X] Lock machine

### Network and bluetooth

- No notes

## Suspend

- [X] Close lid - system suspends
- [ ] 150 suspend successful
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- On second system suspend, system fails to suspend
- Could only test after suspend functionality after one suspend
- UPDATE: this was fixed by https://github.com/system76/firmware/pull/210 (ME downgrade). Now full suspend tests can commence.
