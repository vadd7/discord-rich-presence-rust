## Discord Rich Presence
a tool with a gui to customize your discord rich presence.

this isnt finished yet.
i need to do:
1. 	preview.
2. 	a normal settings page.

# ⚠️
1. 	This is the spaghettiest code ever

2.	i havent tested it on any other platform, only windows 11.
	not like anyone is going to use this anyway so who cares.

# How to set up:
1.  go to the [discord developer portal](https://discord.com/developers/applications/), and create new application.
    name the application, and the name of the application will be the first text of the rich presence.
    copy the application id.

2.  open gui.exe, and go to the settings, and set the discord application id setting to the id of the discord application you created.

3.  configure however you'd like

# Images:
1.  go to the [discord developer portal](https://discord.com/developers/applications/)
2.  go to the rich presence tab.
3.  under rich presence assets, add the images you want.
4.  when configuring, set the image keys to the names you set for the image.

# Dependencies:
sysinfo
discord-rich-presence
egui (egui-extras, eframe, image)
reqwest
serde-json
chorno (i wish i could remove this, used to adjust for timezones.)

# Building:
just run 
```bash
cargo run --bin gui
```
if you want to run the rich presence itself, put discord_presence_activity instead.

or 
```bash
cargo build
```
to just build the project.

use "--release" flag to build the project for release