# Tetris Guideline Standards

- [x] Matrix 10 columns 22 rows high.
- [x] Blocks must follow specific colors.
- [x] The I and O pieces must spawn in the exact center columns.
- [x] The J, L, S, T, and Z pieces must spawn in the left-middle columns.
- [x] All pieces must spawn horizontally.
- [x] SRS and wall kicks.
- [x] Rotation controls must make use of 90° counterclockwise and 90° clockwise.
- [x] 7-Bag Randomizer.
- [x] Hold Box, allowing only one swap per piece drop.
- [x] Ghost Block.
- [x] Lock down must use Extended Placement (Infinity).
- [x] Level progression must be tied to clearing lines.
- [x] Game Over occurs via Block Out or Lock Out .
- [x] The display upcoming pieces.
- [x] Reward T-Spins (Minis, Singles, Doubles, and Triples).
- [x] Back-to-Back Bonus.
- [x] Combo System.
- [ ] DAS and ARR.

# Essentials

### Mechanics

- [x] Implement "next block/s" mechanic.
- [x] Implement "ghost block" mechanic.
- [x] Implement "hold block" mechanic.
- [x] Implement "combo" mechanic.
- [ ] Detect All-spins (L-spin, J-spin, Z-spin, etc.)

### Game Modes

- [x] Endless
- [ ] 40 Lines
- [ ] Blitz

### Architecture

- [x] Refactor Board, too much attributes (use enums, newtypes and typestates. To make that impossible logical states impossible).

### Tets


- [ ] Implement a way to create pre-defined blocks arrangements.
- [ ] Create pre-defined blocks arrangements to test specific scenarios (improve kicks, all spins, etc).

### Github

- [x] Publish release binaries for linux and windows.
- [x] Implement a vhs script to record demos.

### Others

- [x] Implement an initial menu.
- [x] Implement a "game over" screen & "try again" option.
- [x] Implement a debug-only constructor method for BlocksManager that accepts a seed as parameter.
- [x] Implement visual display of "combo name" messages.
- [x] Implement minimal keyboard instructions in the menu.
- [x] Implement 180 deg rotation when pressing "a".
- [x] Implement "delay frames" (Lock Delay).
- [x] Improve x axis translation feel (DAS/ARR).
- [x] Implement better score system (T-Spin, Back-to-Back, Perfect Clear).
- [ ] Implement optional (or default) background color.
- [ ] Implement local stored score system.
- [ ] Implement display top 5 best scores in game over modal.

### Effects

- [ ] explosion effect when die fx::explode

# Fix

- [x] b2b is not accumulating beyond x1 (tested across multiple consecutive quads)
