## Test Platform

| Test complete | OS Version     | BIOS Version | EC Version | Before or after suspend |
| ------------- | -------------- | ------------ | ---------- | ----------------------- |
| INCOMPLETE    | Pop!\_OS 20.04 | 1.07.03      | 1.07.02    | Both                    |

## Checklist
x = pass | blank = fail | na = remove from list

## Hot Keys

Note: display toggle hotkey is in the displays section below.

- [X] Touchpad Lock
- [ ] Turn off display
- [ ] Mute sound
- [ ] Volume down
- [ ] Volume up
- [ ] Brightness Down
- [ ] Brightnes Up
- [X] Camera on/off
- [ ] Airplane Mode
- [X] Suspend
- [X] Play/Pause
- [ ] Keyboard backlight brightness

### Hot key notes and issues

- Fn-F2 (which is supposed to turn off the display) mutes and un-mutes.
- Fn-F3 (which is supposed to mute/un-mute) turns the volume down.
- Fn-F4 (which is supposed to cycle the keyboard backlight brightness) turns the volume up.
- Fn-F5 (which is supposed to turn the volume down) turns the display brightness down.
- Fn-F6 (which is supposed to turn the volume up) turns the display brightness up.
- Fn-F8 (which is supposed to turn the display brightness down) turns the display off.
- Fn-F9 (which is supposed to turn the display brightness up) toggles airplane mode.
- Fn-F11 (which is supposed to toggle airplane mode) cycles the keyboard backlight brightness.

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

- [X] Touchpad two finger scrolling 
- [X] Tap to click
- [X] Left click
- [X] Right click
- [X] Middle click (three finger tap/click for clickpads, click both buttons simultaneously for non-clickpad)

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [X] Left USB Type A
- [X] Right USB Type A
- [X] Left USB Type-C (Thunderbolt)
- [X] Right USB Type C
- [X] SD card slot
- [X] Headphone jack
- [X] Combo jack headphones
- [X] Combo jack mic
- [X] Internal mic
- [X] Internal speakers
- [X] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- Left Thunderbolt port doesn't work.

## Displays

- [X] HDMI port
- [X] HDMI port audio
- [ ] Type C DP
- [ ] Type C DP audio
- [ ] Dual external display + internal
- [X] Close internal display. Monitors adjust correctly.
- [X] Reboot w/ lid closed. Decryption dialog shows on external display.
- [ ] Triple external display via DP daisy chain + internal

### Displays notes and issues

- Left Thunderbolt port doesn't work (including DisplayPort via that port.)
- HDMI audio works, but GNOME doesn't play the volume up/down sound when adjusting via the keyboard.
- Decryption prompt showed up on external display, but GDM didn't start while it was plugged in.

## Network and bluetooth

- [X] Wifi connects with expected performance
- [X] Ethernet connects with expected performance
- [X] Bluetooth mouse
- [X] Lock machine

### Network and bluetooth

- No notes

## Suspend

- [X] Close lid - system suspends
- [ ] 150 suspend successful
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- Have not yet run the long-running suspend tests.

## Other notes

- Had one hang at power-off (as soon as I entered the command.)

