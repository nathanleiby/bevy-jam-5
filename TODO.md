# TODO

## Active

- [ ] smoothly animate the rotation (not hard timesteps) - play/pause/reset

## Paused

## Backlog

- [ ] success when syzygy occurs!
- [ ] tweak sun size
- [ ] handcrafted puzzles
  - [ ] 3 orbits, 2 bodies
    - [ ] desired sound = 2 against 4
  - [ ] 4 orbits, 2 bodies
    - [ ] desired sound = 2 against 3
  - [ ] (optional) 5 elliptical orbits, 3 bodies
    - [ ] desired sound = 4 against 3 (against 1 per measure)
  - [ ] 2 orbits, 2 bodies, .. 2 sun options
    - [ ] desired sound = 2 against 4
  - [ ] 3 orbits, 2 bodies, 3 sun options
    - [ ] desired sound = 2 against 5
- [ ] bug: flickering of overlapping meshes. draw order?
- [ ] play sound for each orbit
  - [ ] maybe use https://github.com/NiklasEi/bevy_kira_audio for better clock scheduling?
  - [ ] ensure web build works with audio
- [ ] add "notches" (debug only) to the orbits that make sound. e.g. every pi/2 or every pi/3
- [ ] add entity spawn fn, for Bevy ease of use (spawn_x)
- [ ] refactor out idea of System/Puzzle

## Done

- [x] Get it drawing again!!
- [x] simplify the math a lot to ease puzzle design .. orbital period $T$ is $T=2\pi {\sqrt {a^{3} \over {\mu }}}$
      For simplicity, the wrong period value could be an irrational number (or most fractions), so it will never sync up an an integer LCM.
- [x] draw a body orbiting another body
- [x] make orbit speed relate to real math
- [x] allow choosing position (aka: assinging body to specific radius from central mass)
  - [x] press a key ("o") to random assign which body has which radius (0, r1, r2)
  - [x] press a key ("r") to reset timestep to 0
- [x] draw the orbital rings (light color circle/ellipse at various distances)
- [x] for visual clarity, scale the size of circles according to their mass

```

```
