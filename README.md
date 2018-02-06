# miosc

Miosc is a simple microtonal protocol over OSC.

## Description

- `/m/reference freq`: set the tone generator reference pitch to `freq`. [freq] = Hz.
- `/m/note_on id pitch vel`: creates a tone `id` with the pitch of `pitch` 12et semitones from the reference pitch and velocity `vel`. The velocity value is in [0, 1.0] range.
- `/m/note_off`: stops the tone `id`
- `/m/pitch id pitch time`: changes the pitch of the tone `id` from the current value to the pitch of `pitch` semitones from the reference. The pitch changes lineary in `time` seconds. 
- `/m/velocity id vel time`: changes the velocity of the tone `id` from the current value to `vel`. The velocity changes lineary in `time` seconds.
