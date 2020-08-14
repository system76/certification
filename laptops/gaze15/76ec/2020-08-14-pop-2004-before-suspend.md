## Test Platform

| Test complete | OS Version     | BIOS Version       | EC Version         | Before or after suspend |
| ------------- | -------------- | ------------------ | ------------------ | ----------------------- |
| INCOMPLETE      | Pop!\_OS 20.04 | 2020-08-14_c0f1d2c | 2020-08-14_c0f1d2c | Before                  |

- Note: this is the 17" with the GTX 1650

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
- [x] Right click

### Touchpad notes and issues

- After resuming from suspend, the trackpad frequently (at least 50% of the time) is unresponsive.

## Ports (Non Display Related)

- [ ] Left USB Type A
- [x] Right USB Type A
- [x] Right USB Type C
- [x] SD card slot
- [x] Headphone jack
- [x] Microphone jack
- [x] Combo jack headphones
- [x] Combo jack mic
- [x] Internal mic
- [x] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- Both left USB-A ports do not work

## Displays

- [ ] HDMI port
- [ ] HDMI port audio
- [ ] Display config toggle (span/cloned/internal/external)
- [x] Mini display port
- [x] Mini display port audio
- [x] Display config toggle (span/cloned/internal/external)
- [ ] Dual external display + internal
- [ ] Triple external display via DP daisy chain + internal
- [x] Close internal display. Monitors adjust correctly.
- [ ] Reboot w/ lid closed. Decryption dialog shows on external display.

### Displays notes and issues

- "Switch to Nvidia" prompt does not appear when connecting a display (mDP or HDMI) while in integrated or hybrid modes.
- HDMI out was not working at all for me (yes, I was in Nvidia mode :P )
- Cryptsetup dialog did not appear on mDP monitor while the lid was closed. 

## Network and bluetooth

- [x] Wifi connects with expected performance
- [x] Ethernet connects with expected performance
- [x] Bluetooth speaker
- [ ] Bluetooth mic
- [ ] Lock machine

### Network and bluetooth

- My four bluetooth audio devices apparently don't have mics that work with Linux, although they all work on my android phone. Either that or bluetooth is broken on everything ever.

## Suspend

- [x] Close lid - system suspends
- [ ] 150 suspend successful
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- No notes
