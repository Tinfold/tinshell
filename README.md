# tinshell

This is my own custom shell, built in Rust.



Recommended programs:
Ripgrep (rg) (cargo install ripgrep)


TO-DO: 
Add cp.
Add ren
Add scp
Add support for star operator or providing lists of items and shit.
Star operator would create a list or or something. Commands would have to interpret lists.
Tab autocomplete
Realtime syntax highlighting of words like powershell
A text editor

Make sure pipes work -> 
IN ORDER TO GET PIPES TO WORK, each command has to be its OWN BINARY! FUCK!
Then we have to spawn it. So all this shit we've been doing with submodules is retarded. WE should have just been making the commands their own binaries.
Need to set up cargo to understand how to do just that. And convert all the commands into binary format.