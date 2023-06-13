# ethernet
A no_std parser for IEEE 802.3 ethernet headers.
## Performance
On my 12th Gen Intel i5-1240p Framework the following execution speeds were achieved.
-- | ns/iter | 1/s
read_ethernet_header | 8.24 | 121*10^6
write_ethernet_header | 7.53 | 132*10^6
