# rotor

Command line utility for real time controlling antenna position through rotctld using libgpredict. Pipes audio input to output so it can be use as a pipe when receiving satellites on linux.

Original work [doppler](https://github.com/cubehub/doppler).

## dependencies
#### mac os x
    xcode-select --install
    brew install autoconf
    brew install automake

#### linux
    sudo apt-get install autoconf

#### libgpredict
Follow install instructions from here: https://github.com/cubehub/libgpredict

#### [liquid-dsp](https://github.com/jgaeddert/liquid-dsp)
    git clone git://github.com/jgaeddert/liquid-dsp.git
    cd liquid-dsp
    ./bootstrap.sh
    ./configure
    make
    sudo make install

#### rust
http://www.rust-lang.org/install.html

    curl -sSf https://static.rust-lang.org/rustup.sh | sh

## build
    git clone https://git.radio.clubs.etsit.upm.es/Meteor-automated/rotor
    cd rotor
    cargo build --release

## install
#### mac os x
    cp target/release/rotor /usr/local/bin/

#### linux
    sudo cp target/release/rotor /usr/local/bin/

## usage

```
rotor --tlefile <TLEFILE> --telename <TLENAME> --location <LOCATION> --server <ROTCTLD_ENDPOINT> --port <ROTCTLD_PORT>
```
It pipes audio input to output. Realtime tracking satellite while receiving would be:

```
ss_client iq -r ${SERVER} -q ${PORT} -f ${FREQ} -s ${SAMPLERATE} | rotor --tlefile ${TLE_FILE} --tlename "${SAT}" --location lat=${RX_LAT},lon=${RX_LON},alt=${RX_ALT} --server ${ROTCTLD_SERVER} --port ${ROTCTLD_PORT} > output.iq
```
