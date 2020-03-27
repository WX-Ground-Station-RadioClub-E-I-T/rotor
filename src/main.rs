/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */


// import local modules
extern crate rotor;
use rotor::usage;

// import external modules
#[macro_use]
extern crate log;
extern crate fern;
use std::process::exit;
use std::io;
use std::net::TcpStream;
use std::io::prelude::*;

use std::thread;

extern crate time;
extern crate gpredict;
use gpredict::{Predict, Tle, Location};

const BUFFER_SIZE: usize = 8192;

fn main() {
    setup_logger();
    let args = usage::args();

    info!("rotor {} acien@ea4rct.org\n\n", env!("CARGO_PKG_VERSION"));

    thread::spawn(move || { // Pipe stdin to stdout
        io::copy(&mut io::stdin().lock(), &mut io::stdout().lock());
    });

    info!("Rotor");
    info!("\tTLE file        : {}", args.tlefile.as_ref().unwrap());
    info!("\tTLE name        : {}", args.tlename.as_ref().unwrap());
    info!("\tlocation        : {:?}", args.location.as_ref().unwrap());
    info!("\tServer       : {}", args.server.as_ref().unwrap());
    info!("\tPort          : {} \n\n\n", args.port.as_ref().unwrap());

    let l = args.location.unwrap();
    let location: Location = Location{lat_deg: l.lat, lon_deg: l.lon, alt_m: l.alt};
    let tlename = args.tlename.as_ref().unwrap();
    let tlefile = args.tlefile.as_ref().unwrap();
    let server = args.server.as_ref().unwrap();
    let port = args.port.as_ref().unwrap();


    let tle = match Tle::from_file(&tlename, &tlefile) {
        Ok(t) => {t},
        Err(e) => {
            info!("{}", e);
            exit(1);
        }
    };

    let mut predict: Predict = Predict::new(&tle, &location);
    let mut last_time: time::Tm = time::now_utc();

    let mut stream = TcpStream::connect(format!("{}:{}", server, port)).unwrap();

    loop {
        predict.update(None);

        if time::now_utc() - last_time >= time::Duration::seconds(1) {
            last_time = time::now_utc();
            info!("time                : {:}", time::now_utc().to_utc().rfc3339());
            info!("az                  : {:.2}°", predict.sat.az_deg);
            info!("el                  : {:.2}°", predict.sat.el_deg);
            info!("range               : {:.0} km", predict.sat.range_km);
            info!("range rate          : {:.3} km/sec \n\n\n", predict.sat.range_rate_km_sec);

            if predict.sat.el_deg > 0.0 {
                stream.write(format!("P {:.1} {:.1}", predict.sat.az_deg, predict.sat.el_deg).as_bytes()).unwrap();
            }
        }
    }
}

fn setup_logger() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            let t = time::now();
            let ms = t.tm_nsec/1000_000;
            let path = _location.__module_path;
            let line = _location.__line;

            format!("{}.{:3} [{:<6} {:<30} {:>3}]  {}",
                    t.strftime("%Y-%m-%dT%H:%M:%S")
                     .unwrap_or_else(|err| panic!("strftime format error: {}", err)),
                    ms, level, path, line, msg)
        }),
        output: vec![fern::OutputConfig::stderr()],
        level: log::LogLevelFilter::Debug,
        directives: vec!()
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }
}
