#!/bin/bash

rm -rf ~/hypr
rm -rf ~/waybar
ln -sf ~/dotfiles/hypr ~/.config
ln -sf ~/dotfiles/waybar ~/.config
sudo dnf install waybar
sudo dnf install hyprpaper
sudo dnf copr enable atim/starship
sudo dnf install starship
cp ~/dotfiles/starship.toml ~/.config/starship.toml
echo 'eval "$(starship init bash)' >> ~/.bashrc
