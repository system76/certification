## Test Platform

| Test complete | OS Version     | BIOS Version        | EC Version          | Before or after suspend |
| ------------- | -------------- | ------------------- | ------------------- | ----------------------- |
| INCOMPLETE    | Pop!\_OS 20.10 | 2020-11-23\_a5ed282 | 2020-11-23\_a5ed282 | Before                  |

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
- [x] Keyboard backlight brightness up/down

### Hot key notes and issues

- No notes

## Power button

- [ ] External power button is disabled when the lid is closed, the machine is powered off, and not connected to a charger (barrel or USB-C)
- [x] External power button is disabled when the lid is closed, the machine is suspended, and not connected to a charger (barrel or USB-C)
- [ ] External power button turns on the machine when the lid is closed and the barrel charger is connected
- [ ] External power button turns on the machine when the lid is closed and a USB-C charger is connected
- [x] External power button wakes the machine from suspend when the lid is closed and the barrel charger is connected
- [x] External power button wakes the machine from suspend when the lid is closed and a USB-C charger is connected

## Power button notes and issues

- When no chargers are connected and the lid is closed: the power button does not resume from suspend, but it does turn the machine on. 
- When the lid is closed and a USB-C charger is connected, the power button does not power on the machine.
- When the lid is closed and a barrel charger is connected, the power button does not power on the machine if the barrel charger was connected when the machine was shut down. If I unplug then re-plug the barrel charger, the power button will turn on the machine.

## Touchpad

- [x] Touchpad two finger scrolling 
- [x] Tap to click
- [x] Left click
- [x] Right click
- [x] Middle click (three finger click/tap)
- [x] Palm rejection works

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [x] Left USB Type A
- [x] Right USB Type A
- [ ] Left USB Type C
- [x] Right USB Type C
- [x] SD card slot
- [x] Combo jack headphones
- [ ] Combo jack mic
- [x] Internal mic
- [x] Internal speakers
- [x] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- Left USB-C works with 65W USB-C charger, but storage devices are not detected.
- Plugging headphones in does not show the headphones/headset prompt and a combo headset mic doesn't show up as an input source.

## Displays

- [x] HDMI port
- [x] HDMI port audio
- [ ] Type C DP
- [ ] Type C DP audio
- [ ] Dual external display + internal
- [x] Close internal display. Monitors adjust correctly.
- [x] Reboot w/ lid closed. Decryption dialog shows on external display.
- [ ] Triple external display via DP daisy chain + internal

### Displays notes and issues

- USB-C with DP (left side) is not working.

## Network and bluetooth

- [x] Wifi connects with expected performance
- [x] Ethernet connects with expected performance
- [x] Bluetooth speaker
- [x] Lock machine

### Network and bluetooth

- No notes

## Suspend

- [x] Close lid - system suspends
- [ ] 150 suspend successful
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- No notes
