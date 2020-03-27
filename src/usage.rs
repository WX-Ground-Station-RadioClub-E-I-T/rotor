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

use clap::{App, AppSettings, Arg};

use std::process::exit;

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
}

pub struct CommandArgs {
    pub tlefile: Option<String>,
    pub tlename: Option<String>,
    pub location: Option<Location>,
    pub server: Option<String>,
    pub port: Option<String>,
}

fn parse_location(location: &str) -> Result<Location, String> {
    if location.contains("lat") && location.contains("lon") && location.contains("alt"){
        let split = location.split(",");

        let mut lat: Option<f64> = None;
        let mut lon: Option<f64> = None;
        let mut alt: Option<f64> = None;

        for s in split {
            if s.contains("lat") && s.contains("=") {
                lat = s.split("=").nth(1).unwrap().parse::<f64>().ok();
            }
            else if s.contains("lon") && s.contains("=") {
                lon = s.split("=").nth(1).unwrap().parse::<f64>().ok();
            }
            else if s.contains("alt") && s.contains("=") {
                alt = s.split("=").nth(1).unwrap().parse::<f64>().ok();
            }
        }

        if lat.is_some() && lon.is_some() && alt.is_some() {
            Ok(Location{lat: lat.unwrap(), lon: lon.unwrap(), alt: alt.unwrap()})
        }
        else {
            Err(format!("{} isn't a valid value for --location\n\t[use as: lat=58.64560,lon=23.15163,alt=8]", location).to_string())
        }
    }
    else {
        Err("--location should be defined as: lat=58.64560,lon=23.15163,alt=8".to_string())
    }
}

pub fn args() -> CommandArgs {
    let matches = App::new("rotor")
                .author("Fran Acien - RadioClub E.I.T <acien@ea4rct.org>")
                .version(env!("CARGO_PKG_VERSION"))
                .about("Move antenna rotor with rotctld using TLE data. Pipe input to output. The idea is to use it as a Linux Pipe when receiving.")
                .setting(AppSettings::AllowLeadingHyphen) // allow negative values like --offset -5000
                .arg(Arg::with_name("TLEFILE")
                   .long("tlefile")
                   .help("TLE file: eg. http://www.celestrak.com/NORAD/elements/cubesat.txt")
                   .required(true)
                   .takes_value(true))
                .arg(Arg::with_name("TLENAME")
                   .long("tlename")
                   .help("TLE name in TLE file: eg. ESTCUBE 1")
                   .required(true)
                   .takes_value(true))
                .arg(Arg::with_name("LOCATION")
                   .long("location")
                   .help("Observer location (lat=<deg>,lon=<deg>,alt=<m>): eg. lat=58.64560,lon=23.15163,alt=8")
                   .required(true)
                   .use_delimiter(false)
                   .takes_value(true))
                .arg(Arg::with_name("ROTCTLD_ENDPOINT")
                   .long("server")
                   .help("Server name to connect: eg. 138.30.4.1")
                   .required(true)
                   .takes_value(true))
                .arg(Arg::with_name("ROTCTLD_PORT")
                   .long("port")
                   .help("Server port to connect: eg. 4555")
                   .required(true)
                   .takes_value(true))
                .get_matches();


    let mut args = CommandArgs {
                    tlefile : None,
                    tlename : None,
                    location: None,
                    server: None,
                    port: None,
                };

    args.tlefile = Some(matches.value_of("TLEFILE").unwrap().to_string());
    args.tlename = Some(matches.value_of("TLENAME").unwrap().to_string());
    let location = parse_location(&matches.value_of("LOCATION").unwrap().to_string());
    match location {
        Ok(loc) => { args.location = Some(loc);},
        Err(e) => {
            error!("{}.", e);
            exit(1);
        }
    }

    args.server = Some(matches.value_of("ROTCTLD_ENDPOINT").unwrap().to_string());
    args.port = Some(matches.value_of("ROTCTLD_PORT").unwrap().to_string());

    args
}
