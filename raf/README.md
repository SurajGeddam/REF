
## Setup

1. Install RaF

For the development version:

```bash
cargo install --path .
```

<!-- The stable version is not (yet) ready, especially because we depend on the fork that isn't published on crates.io but it's only a git repository. -->

For the production version:

```bash
cargo install telegram-raf
```

2. Create the run path and the environment file

```bash
mkdir $HOME/.raf

echo 'BOT_NAME="<your bot name>"' > $HOME/.raf/raf.env
echo 'TOKEN="<your bot token>"' >> $HOME/.raf/raf.env
```

3. Copy the systemd service file

```bash
sudo cp misc/systemd/raf@.service /lib/systemd/system/
```

4. Start and enable the service

```bash
sudo systemctl start raf@$USER.service
sudo systemctl enable raf@$USER.service
```

The `raf.db` (to backup or inspect) is in `$HOME/.raf/`.

