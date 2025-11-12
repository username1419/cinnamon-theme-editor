An application which helps users create and edit themes for the Cinnamon Desktop Environment, written in Rust with the help of libadwaita and GTK4.

---
## Features
something

---
## Installation
something

---
## Building

### Dependencies:
- libadwaita-1 >= 1.4
- gtk-4 >= 4.14(?)
- glib
- cargo
- git

Install using the `build.sh` script provided:
```sh
git clone https://github.com/username1419/cinnamon-desktop-editor # clone the repository
cd cinnamon-desktop-editor

./scripts/build.sh --release # build the project
```
The script does not do anything other than compiling gresources and executing `cargo build`, you can view its code [here](file address here).

---
## Contribution
Any contributions would be greatly appreciated, just fork the repo, make your changes, and open a pull request for the project. I will review them when I have the time. Bug reports, ideas for improvements, questions, and the like are welcome on the [issues](https://github.com/username1419/cinnamon-desktop-editor/issues) tab!

---
## Known Issues
- the css parser this project uses was implemented badly, and will perform not greatly with larger css files. i will go back and rewrite it when i feel like it
   - also as a result of this stupid decision of mine the parsing process also removes all comments from the theme css file and if the css file contains an error, the parser will not be able to point out where it is
- missing documentation for many functions & classes

---
## License
MIT

---
man
