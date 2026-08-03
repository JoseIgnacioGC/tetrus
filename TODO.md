# Tetris Guideline Standards

- [x] Matrix 10 columns 22 rows high.
- [x] Blocks must follow specific colors.
- [x] The I and O pieces must spawn in the exact center columns.
- [x] The J, L, S, T, and Z pieces must spawn in the left-middle columns.
- [x] All pieces must spawn horizontally.
- [x] Super Rotation System (SRS) for rotation and wall kicks.
- [x] Rotation controls must map left to 90° counterclockwise and right button to 90° clockwise.
- [x] 7-Bag Randomizer.
- [x] Hold Box, allowing only one swap per piece drop.
- [x] Ghost Block.
- [x] Lock down must use Extended Placement (Infinity), allowing up to 15 movements or rotations on a surface before locking.
- [x] Level progression must be tied to clearing lines or performing T-Spins, not just time survived.
- [x] Game Over (Top out) occurs via Block Out (spawning on an existing block) or Lock Out (locking completely above the visible playfield).
- [x] The game should display upcoming pieces in a Next Piece Queue (strongly recommended up to 6 pieces).
- [x] The engine should recognize and reward T-Spins (Minis, Singles, Doubles, and Triples).
- [x] Chaining difficult line clears (T-Spins or Tetrises) should grant a Back-to-Back Bonus.
- [x] The game should feature a Combo System that rewards clearing lines on consecutive piece drops.
- [ ] Delayed Auto Shift (DAS) and Auto Repeat Rate (ARR) must be tuned to standard comfortable thresholds.

---

# Essentials

### Mechanics

- [x] Implement "next block/s" mechanic.
- [x] Implement "ghost block" mechanic.
- [x] Implement "hold block" mechanic.
- [x] Implement "combo" mechanic.

### Game Modes

- [x] Endless
- [ ] 40 Lines
- [ ] Blitz

### Architecture

- [ ] Refactor Board, too much attributes (use enums, newtypes and typestates. To make that impossible logical states impossible).

### Controls

- [ ] Implement reload current game (game mode) after hold "r" for x seconds (display a message/modal that warn and inform about that you need to hold "r" while you press "r").
- [ ] Improve "esc" quit behavior, you should be able to quit in any moment if you hold "esc" for x seconds (display a message/modal while holding "esc" to show progress).

### Others

- [x] Implement an initial menu.
- [x] Implement a "game over" screen & "try again" option.

# Maybe

- [x] Implement visual display of "combo name" messages.
- [x] Implement minimal keyboard instructions in the menu.
- [x] Implement 180 deg rotation when pressing "a".
- [x] Implement "delay frames" (Lock Delay).
- [x] Improve x axis translation feel (DAS/ARR).
- [x] Implement better score system (T-Spin, Back-to-Back, Perfect Clear).
- [ ] Implement optional (or default) background color.
- [ ] Implement local stored score system.
- [ ] Implement display top 5 best scores in game over modal.
