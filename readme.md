<h3 style="font-size: 2rem;font-weight:600;" align="center">☆:｡✱*. niri-sense ｡*✱.:｡</h3>
<p align="center">
integrate your toys with niri!
</br>
</br>
<img alt="GitHub Release" src="https://img.shields.io/github/v/release/itzreesa/niri-sense?style=flat-square&color=faa">
<img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/itzreesa/niri-sense?style=flat-square&color=fdf">
<img alt="GitHub License" src="https://img.shields.io/github/license/itzreesa/niri-sense?style=flat-square&color=aaf">
</br>
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/itzreesa/niri-sense?style=flat-square&labelColor=4B8BBE&color=FFE873">
<img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/t/itzreesa/niri-sense?style=flat-square&color=bfb">
<img alt="Static Badge" src="https://img.shields.io/badge/made%20with-%3A3-d26?style=flat-square">
</br>
</p>


### Quick Start
Install `niri` and rust... then:  
`cargo install --git https://github.com/itzreesa/niri-sense`  

Or download the latest binary from [the releases tab](https://github.com/itzreesa/niri-sense/releases)  
and put it somewhere in your `PATH`, like `~/.local/bin`

##### preview
[preview.webm](https://github.com/user-attachments/assets/9f5c33e7-d0d3-436b-a05d-a19e148bcdcc)

### Usage
Run `niri-sense` inside a _niri session_ to launch it!  
You'll need a _buttplug.io_ compatible server, like [Intiface(r) Central](https://intiface.com/)  

- Start the _server_.
- From `sense`'s **REPL**, use `connect` to connect to your _server_.
- Run a **scan** using `scan start`, after finding your **device** use `scan stop` to stop the scanning.
- That's it! Try moving around your workspace, you should now _feel_ the app running!

Optionally, you can use `list` and `select` to select between one or all **devices** found.


### Trivia
I thought of this project while playing _deadlock_, then I jokingly DM'd **oomf** about this idea, because I remembered the lovelock mod that I've seen was popular on twitter. 

Oh, and thanks **oomf** for testing, I don't actually own any hardware. My testing was on just the simulated one.

### REPL Commands
- `connect` and `disconnect` - handle the connection between `sense` and your **server**
- `scan start` and `scan stop` - toggle scanning
- `list` - provides a nice list of **devices**, their **features** and **batteries**, if found.
- `select` - will prompt you with a device index selection, where `-1` means all devices!
- `test` - will run a short **test** of vibrations
- `stop` - will stop all **server** _events_ that were currently running
- `pause` - toggles the _event_ processing.
- `reload` - will reload your configuration file.
- `exit` - will exit at the next **niri event**.
  
### Configuration
The **configuration** is in a `toml` format.
Here's a few useful fields:
- `address` and `port` control where you will **connect** with the `connect` command
- all the **fields** from the `[lengths]` _table_, will correspond to named event lengths. (time in milliseconds)
- all the **fields** from the `[strengths]` _table_, will correspond to named event strengths. (values in %, from 0 to 100)
- `repeated_event_prevention` from `[events]`, if turned on, will try to prevent multiple outputs for some events.

#### the `[events]` table.
**Events** here are formatted like that: `event_name = ["state", "strength", "length"]`  
Where state can be either `"on"` or `"off"`, and the **lengths** and **strengths** are defined above.  
###### note: you cannot add more strength or length variables for now.

#### explanation for a few events
- `workspace_changed` - triggers when your **workspace** is changed, e.g. when adding or removing
- `window_opened_or_changed` - triggers either when a **new window** is opened, or when an **existing window's** information changes, like on resizing.
- `window_layout_changed` - triggers on your **window layout** changes
- `niri_config_reloaded` - triggers on **niri's config** reload, off by default

### Issues
Report issues in the [issues](https://github.com/itzreesa/niri-sense/issues) tab

#### troubleshooting tips
- make sure you have flipped the switch in your server...
- make sure you have _bluetooth_ enabled,
- make sure you have entered the right address and port in your config
- make sure nothing is blocking the ports
- try manually settings `NIRI_SOCKET` if you can't connect to niri's ipc.
- if a new version is released, try deleting your config if it fails to load.
- try seeing if your devices are recognized in your server panel.
- try switching between _bluetooth le_ and _non-le_

### License
`niri-sense`'s source code is license under the GPLv3 license

###### previously the project was a fork of niri, day later i noticed i can just listen to niri's ipc :sob: https://github.com/itzreesa/niri-sense-legacy
###### the original caption was "make it make more sense", I'm so funny right?
