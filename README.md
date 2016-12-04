# hue_storage
Hue persistence for Philips Hue Lights 

Run this program as a background service on your local network.
It remembers the configuration of the lamps on the network.
It restores the configuration of a lamp when it is turned on, unlike the default behavior where they switch to normal white.
Please allow up to 10 seconds for the program to restore the lamp as this program polls the hue hub only every 10 seconds.

Not tested with multiple hubs and will probably not work.
