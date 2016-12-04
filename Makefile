all:
	cargo build --release

# assumes hue_storage.services is already copied in /lib/systemd/system/

install:
	-systemctl stop hue_storage
	cp ./target/release/hue_storage /usr/local/sbin
	systemctl enable hue_storage
	systemctl deamon-reload
	systenctl start hue_storage
	systemctl status hue_storage
