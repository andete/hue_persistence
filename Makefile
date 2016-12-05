all:
	cargo build --release

# assumes hue_persistence.services is already copied in /lib/systemd/system/

install:
	-systemctl stop hue_persistence
	cp ./target/release/hue_persistence /usr/local/sbin
	systemctl enable hue_persistence
	systemctl deamon-reload
	systenctl start hue_persistence
	systemctl status hue_persistence
