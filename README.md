# mc-port-scanning-thing
Simple port scanner for minecraft written in rust.

Current commands:
mc_port_scanner address
    - Scans the provided IP address

mc_port_scanner -h
    - Returns a help message

mc_port_scanner -r start_ip end_ip
    - Scan over a range of ip addresses starting from start_ip and ending at end_ip

mc_port_scanner -F ...
    - Enable forceful conversion of client bound bytes to string
    - Note this uses unsafe rust type conversion!!
