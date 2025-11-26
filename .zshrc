export ZSH="$HOME/.oh-my-zsh"
ZSH_THEME=""
source $ZSH/oh-my-zsh.sh
export EDITOR=nvim
eval "$(zoxide init zsh)"
ZSH_THEME="robbyrussell"
plugins=(
	zsh-autosuggestions
	fast-syntax-highlighting
	rust
	git
	gpg-agent
)

# Autosuggestion TAB
bindkey '^I' autosuggest-accept

source $ZSH/oh-my-zsh.sh
export STARSHIP_CONFIG="$HOME/dotfiles/starship.toml"
eval "$(starship init zsh)"
export XDG_CONFIG_HOME="$HOME/.config"
export PATH="$HOME/.cargo/bin:$PATH"
export VDPAU_DRIVER=nvidia
