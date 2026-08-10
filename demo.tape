# Only tested on linux!

# Previous Work
#   Hide
#   Type "cargo build --release --features vhs; echo BUILD_FINISHED"
#   Enter
#   Wait+Screen@15000ms /BUILD_FINISHED/
#   Show

Output assets/demo.gif

# Setup game window
Set Theme "tokyonight"
Set FontFamily "Agave Nerd Font Mono"
Set FontSize 24
Set Width 1200
Set Height 750
Set Padding 20

# Open the game
Type "tetrus"
Sleep 2s

Hide
Ctrl+C
Type "./target/release/tetrus"
Enter
Sleep 3s
Show

# Enter a game mode
Sleep 2s
Enter
Sleep 2s

# Gameplay
Left 2
Sleep 200ms
Space
Sleep 500ms

Right 3
Sleep 200ms
Space
Sleep 500ms

Right 3
Left 2
Space
Sleep 500ms

Right 3
Space
Sleep 500ms

Left 5
Space
Sleep 500ms

# End of the Record
Hide
Escape
Sleep 500ms
Type "clear"
Enter
Sleep 500ms
Type "tetrus"
Sleep 500ms
Show

Sleep 2s
