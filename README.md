# A simple DNS resolver written in Rust.
This small project is made exclusively out of my curiosity and desire to learn rust as well as some networking basics.

## Usage
For now, it works on macOS. You can run it with the following command:
```bash
sudo cargo build 

target/debug/resolver -i v4 -d bbc.com
```
Where `-i` is a flag that specifies the IP version and `-d` is a flag that specifies the domain name to resolve.

Output should look like this: 
```bash
using 0.0.0.0:5768 
asking 1.1.1.1:53 for bbc.com...
answer:
IPv4: 151.101.0.81
IPv4: 151.101.64.81
IPv4: 151.101.128.81
IPv4: 151.101.192.81
``` 

Desirable DNS Server and UDP port are set in `.env` file:
```bash
DNS_SERVER=1.1.1.1
UDP_SOCKET_PORT=5768
```
Default DNS Server is `8.8.8.8` by Google.
