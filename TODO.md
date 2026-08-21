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

### Tests

- [x] Implement a way to create pre-defined blocks arrangements.
- [x] Create pre-defined blocks arrangements to test specific scenarios (improve kicks, all spins, etc).
- [x] When you enter a test board you should have the piece that you need to do what the preset is intended to do. (e.g. you should start with the T piece when entering the T-Spin Double Setup).
- [ ] Rename "debug boards" to "learn moves" remove the debug flags.
- [ ] Learn boards should have 0 gravity, or the user should be able to change gravity (option between "Learn Boards\n\n\gravity 1 (if you press <-/-> the counter increase and decrease, <- do not go back in this option, this is not the default option, that's the first board in the list\n\n \<first board\> ...)
- [ ] Learn boards shouldn't have "top scores" or any score system, at all. (extract score logic into a widget, then inyect it just when needed. Improve the gameover state machine to support any game style and debug boards)
- [ ] Learn boards "again?" option should return to the pre-build board, not a normal board.

### Github

- [x] Publish release binaries for linux and windows.
- [x] Implement a vhs script to record demos.

### UI/UX

- [ ] Display a borders around the entire tui if the terminal is smaller than the min width ( based on COLUMNS \* 2 + gaps) or min height (based on ROWS \* 2 + the title + gaps), Display a warning message.

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
- [x] Add an option to return to menu in game over.
- [ ] Block "space" key for some time when you die to avoid unintentional fast restart.
- [ ] Implement optional (or default) background color.
- [x] Implement local stored score system.
- [x] Implement display top 5 best scores in game over modal.

### Effects

- [ ] explosion effect when die fx::explode

# Fix

- [x] b2b is not accumulating beyond x1 (tested across multiple consecutive quads)
