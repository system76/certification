## Test Platform

| Test complete | OS Version     | BIOS Version       | EC Version         | Before or after suspend |
| ------------- | -------------- | ------------------ | ------------------ | ----------------------- |
| COMPLETE      | Pop!\_OS 20.10 | 2021-01-19_d6d37c0 | 2021-01-19_d6d37c0 | Both                    |

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
- [X] Keyboard backlight brightness

### Hot key notes and issues

- No notes

## Power button

- [X] External power button is disabled when the lid is closed, the machine is powered off, and not connected to a charger (barrel or USB-C)
- [X] External power button is disabled when the lid is closed, the machine is suspended, and not connected to a charger (barrel or USB-C)
- [X] External power button turns on the machine when the lid is closed and the barrel charger is connected
- [X] External power button turns on the machine when the lid is closed and a USB-C charger is connected
- [X] External power button wakes the machine from suspend when the lid is closed and the barrel charger is connected
- [X] External power button wakes the machine from suspend when the lid is closed and a USB-C charger is connected

## Power button notes and issues

- After one of the test suspend/resumes, the system froze at the lock screen with the fans spinning.

## Touchpad

- [X] Touchpad two finger scrolling 
- [X] Tap to click
- [X] Left click
- [X] Right click
- [X] Middle click (three finger tap/click for clickpads)

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [X] Left USB Type A
- [X] Right USB Type A
- [X] Left USB Type-C (Thunderbolt)
- [X] Right USB Type C
- [ ] SD card slot
- [X] Headphone jack
- [X] Combo jack headphones
- [X] Combo jack mic
- [X] Internal mic
- [X] Internal speakers
- [X] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- Still having trouble with an SD card after connecting/disconnecting USB and Thunderbolt devices. Not a regression.

## Displays

- [X] HDMI port
- [X] HDMI port audio
- [X] Type C DP
- [X] Type C DP audio
- [X] Dual external display + internal
- [X] Close internal display. Monitors adjust correctly.
- [X] Reboot w/ lid closed. Decryption dialog shows on external display.
- [ ] Triple external display via DP daisy chain + internal

### Displays notes and issues

- DisplayPort dasiy-chain is mirrored only via USB-C (not specific to galp5 and not a regression.)

## Network and bluetooth

- [X] Wifi connects with expected performance
- [X] Ethernet connects with expected performance
- [X] Bluetooth mouse
- [X] Lock machine

### Network and bluetooth notes and issues

- No notes

## Suspend

- [X] Close lid - system suspends
- [X] 150 suspend successful
- [ ] System prompts and eventually suspends on depleted battery


### Suspend notes and issues

- System powered off instead of suspending on depleted battery (not specific to galp5, not a regression.)
