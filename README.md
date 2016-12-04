# hue_storage
Hue persistence for Philips Hue Lights 

Run this program as a background service on your local network.
It remembers the configuration of the lamps on the network.
It restores the configuration of a lamp when it is turned on, unlike the default behavior where they switch to normal white.
This programs only checks the status of the lamps every 10 seconds.
This means that it can take up to 10 seconds for a lamp to get configured.

This program is not tested with multiple hubs and will probably not work in any other configuration the single hub.
