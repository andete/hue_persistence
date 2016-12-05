# hue_persistence

Hue persistence for Philips Hue Lights 

[![Build Status](https://travis-ci.org/andete/hue_persistence.svg?branch=master)](https://travis-ci.org/andete/hue_persistence)

Run this program as a background service on your local network.
It remembers the configuration of the lamps on the network.
It restores the configuration of a lamp when it is turned on, unlike the default behavior where they switch to normal white.
This programs only checks the status of the lamps every 10 seconds. It also can take the hub up to half a minute to detect a light bulb that has just been turned on.
Als this means that it can take up to 1 minute for a lamp to get configured.

This program is not tested with multiple hubs and will probably not work in any other configuration than single hub.

