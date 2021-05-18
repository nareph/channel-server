# channel-server Project
web server to communicate with iota tangle.
It's just a small project to use the features of IOTA Streams (https://github.com/iotaledger/streams) 
which is developed in Rust; that's why this project is also developed in Rust which allowed me to learn this language; 
so this project is my first in Rust.  

## Services
The web server offers four services:
 1. **Create channel** 
    (path = /iotaOrigin/create and method = POST).
    the service return the channel parameter (state) and first message(payload)
    
 2. **Write message on Channel** 
    (path = /iotaOrigin/update, method = PUT, Data = state of channel + message(payload))
     the service return new **state** of channel (needed to write the future message)

 3. **Get last message on channel**  
    (path = /iotaOrigin/balance, method = GET, Data = last state of channel)   

4. **List all message on channel**  
    (path = /iotaOrigin/history, method = GET, Data = first state of channel)   

**Note**: the first state of the channel is a state returned when you created the channel. and the last state is a state returned when you wrote the last message.
    
## Usage
Edit the assets/channel-config.init file and set the address and port of your server.
You can also change the node that represents the iota node link and the password that is used to encrypt/decrypt the channel state.

You can now use the Rust command to run the project in Debug/Release mode.

```bash
cargo run --release # run this in your Cargo project directory (channel)
```

## Generate a Linux App Installer 
```bash
cargo install cargo-deb
cargo deb # run this in your Cargo project directory (channel)
```
You have a target/debian/channel-server_0.1.0_amd64.deb created for you. 
You can install this in any debian based linux system and run your server from the command line.

#### Install Linux App 
you can simply double click and install through the GUI or 
```bash
sudo dpkg -i channel-server_0.1.0_amd64.deb 
```

#### Check your installation 
```bash
cat /usr/local/etc/channel-config.ini 
ls -la /usr/local/bin/channel-server
```

#### Start, Check and Restart server 
```bash
sudo systemctl start channel-server # to start the server
sudo systemctl status channel-server # check status
sudo systemctl restart channel-server # you can change the config file, maybe the port and restart server
```