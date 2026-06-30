Output gifs/initial-lines.gif

Set FontSize 24
Set Width 900
Set Height 600
Set Padding 20
Set Framerate 30
Set Theme "Catppuccin Mocha"

Type "printf 'line %02d\n' {1..20} | target/debug/slomore --initial-lines 8 --lines-per-second 2"
Enter
Sleep 8s
