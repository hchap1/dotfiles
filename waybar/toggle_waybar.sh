#!/bin/bash

if pgrep -x "waybar" > /dev/null; then
	killall -q waybar
else
	waybar &
fi
