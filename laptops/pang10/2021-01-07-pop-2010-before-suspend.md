## Test Platform

| Test complete | OS Version     | BIOS Version | EC Version | Before or after suspend |
| ------------- | -------------- | ------------ | ---------- | ----------------------- |
| INCOMPLETE    | Pop!\_OS 20.10 | 1.07.05TMID  | 1.07.04    | Before                  |

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
- [ ] Airplane Mode
- [ ] Suspend
- [x] Play/Pause
- [x] Keyboard backlight on/off
- [x] Keyboard backlight brightness up
- [x] Keyboard backlight brightness down
- [x] Keyboard backlight toggle colors

### Hot key notes and issues

- Airplane mode hotkey turns on airplane mode, but does not turn it off.
- Suspend hotkey suspends the machine but does not wake it up. Only the power button currently wakes the machine.

## Touchpad

- [x] Touchpad two finger scrolling 
- [x] Tap to click
- [x] Left click
- [x] Right click
- [x] Middle click (3 finger tap and click)
- [x] Palm rejection

### Touchpad notes and issues

- No notes

## Ports (Non Display Related)

- [x] Left USB Type A ports (USB 2.0)
- [x] Right USB Type A
- [x] Right USB Type C
- [x] MicroSD card slot
- [x] Combo jack headphones
- [x] Combo jack mic
- [x] Internal mic
- [x] Internal speakers
- [x] Plugging in headphone or combo jack mutes internal speakers

### Ports notes and issues

- MicroSD card goes deep into the case and it's a bit difficult to get a hold of it to remove from the slot

## Displays

- [x] HDMI port
- [x] HDMI port audio
- [x] Dual external display + internal
- [x] Close internal display. Monitors adjust correctly.
- [ ] Reboot w/ lid closed. Decryption dialog shows on external display.

### Displays notes and issues

- Decryption dialog does not show on external display unless I hit Esc twice. When rebooting with external display connected, the encryption prompt seems frozen. Sometimes I see just the gray background with nothing on it, and sometimes I see the dialog, but the cursor does not blink and placeholder symbols do not appear when I type. Typing the password and hitting enter still unlocks the drive and takes me to GDM. I believe this freeze is what prevents the dialog from showing on the external display. Since this seems like a Plymouth bug, I filed it here: https://github.com/pop-os/plymouth-theme/issues/18 .
- When rebooting the machine with the lid closed and an external display connected, the machine suspends after I sign in at GDM.

## Network and bluetooth

- [x] Wifi connects with expected performance
- [x] Ethernet connects with expected performance
- [x] Bluetooth speaker
- [x] Lock machine

### Network and bluetooth

- No notes

## Suspend

- [x] Close lid - system suspends
- [ ] Open lid - system resumes
- [ ] 150 suspend successful
- [ ] System prompts and eventually suspends on depleted battery

### Suspend notes and issues

- Lid close suspends the machine, but lid open does not wake it up. Only the power button currently wakes the machine.

## Other notes

- When I boot the machine on AC power and stress it with `s-tui`, the CPU hovers around 4100MHz. When I unplug it, it drops and bounces around at ~2800MHz. If I then plug the AC adapter back in, the frequency stays at the speeds I was seeing on battery.
