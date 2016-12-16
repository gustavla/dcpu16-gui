# Changelog

## Next release
Released: TBD
* Added Generic Keyboard (' and ` still not triggering)
* Moved device implementations to separate files
* Dependency update: dcpu16 0.2.0

## 0.2.0
Released: 2016-12-14
* LEM1802
  * Full support for all interrupts
  * Added border (can be configured through interrupts)
  * Supports blinking flag
  * Shows proper default screen when disconnected
  * Must specify custom address when pre-connecting monitor (e.g. `-m 0x8000`)
  * Ability to pre-configure font address via CLI (e.g. `-f 0x8180`)
* Dependency update: dcpu16 0.1.0

## 0.1.1
Released: 2016-12-06
* Dependency update: dcpu16 0.0.7

## 0.1.0
Released: 2016-12-06
* Updated to recent Piston version
