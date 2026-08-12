# Only tested on linux!

Output assets/demo.gif

# Setup game window
Set Theme "tokyonight"
Set FontFamily "Agave Nerd Font Mono"
Set FontSize 24
Set Width 1200
Set Height 750
Set Padding 20

# Previous Work
Hide
Type "cargo build --release --features vhs"
Enter
Wait@20s />$/
Type "clear"
Enter
Sleep 500ms
Show

# Open the game
Type "tetrus"
Sleep 2s

Hide
Ctrl+C
Type "./target/release/tetrus"
Enter
Sleep 1.5s
Show

# Enter main game mode
Sleep 2s
Enter
Sleep 1.5s

# -- Gameplay -- 
Set TypingSpeed 40ms

# Setup Ground
Type "c"
Sleep 50ms
Right 2
Space
Sleep 50ms

Left 2
Space
Sleep 50ms

Type "c"
Sleep 50ms
Left 2
Space
Sleep 50ms

Right 4
Space
Sleep 50ms

Type "z"
Left 4
Space
Sleep 50ms

Right 2
Space
Sleep 50ms

Type "a"
Right 2
Space
Sleep 50ms

Type "a"
Left 2
Space
Sleep 50ms

Left 2
Space
Sleep 50ms

Type "cz"
Right 2
Space
Sleep 50ms

Right 4
Space
Sleep 50ms

Type "x"
Left 2
Space
Sleep 50ms

Type "z"
Left 3
Space
Sleep 50ms

Right 4
Space
Sleep 50ms

Type "z"
Left 3
Space
Sleep 50ms

Right 4
Space
Sleep 50ms

Type "z"
Right 3
Space
Sleep 50ms

Type "cx"
Right 4
Space
Sleep 50ms

Type "x"
Right 3
Space
Sleep 50ms

Type "a"
Left 3
Space
Sleep 50ms

Type "x"
Left 2
Space
Sleep 50ms

Type "x"
Right 2
Space
Sleep 50ms

# First T-spin
Type "x"
Down 20
Type "x"
Sleep 50ms
Space
Sleep 100ms

# Second T-spin, combo 1
Type "cx"
Down 20
Type "x"
Sleep 50ms
Space
Sleep 500ms

# combo 2
Type "x"
Sleep 1s
Space
Sleep 500ms

# combo 3

Type "c"
Sleep 1s
Space
Sleep 1s

# combo 4

Type "cx"
Sleep 1s
Space
Sleep 1s

# Die on purpose
Sleep 3s
Space 10
Sleep 1s

# Quit
Down
Sleep 500ms
Enter

# -- Gameplay -- 

# End of the Record
Hide
Sleep 500ms
Type "clear"
Enter
Sleep 500ms
Type "tetrus"
Sleep 500ms
Show

Sleep 5s
