# Checksum calculator
This package contains a single function, generate_checksum, which takes a file and returns a promise which resolves to the sha256 checksum of this file.

This has been tested using an Intel Pentium N4200 1.1Ghz and this is faster than the Web Crypto digest by around 1 second per 60 Mb of file.