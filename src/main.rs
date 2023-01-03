//
//
//
//

// extern crate serialize;
// use serialize::json;
use clap::Parser;
use color_print::cprintln;
use remotefs::RemoteFs;
use remotefs_ssh::SftpFs;
use remotefs_ssh::SshOpts;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::process;
use std::time;
use std::time::SystemTime;
use std::{fs, io};

// use serde::{Deserialize, Serialize};

type Tdateiliste = HashMap<String, i64>;

#[derive(Parser)]
struct Args {
    #[arg(short, long, env, required = false, default_value = "localhost")]
    server: String,
    #[arg(short, long, env, required = false, default_value = "user")]
    user: String,
    #[arg(short, long, env, required = false, default_value = "password")]
    password: String,
    #[arg(short, long, env, required = false, default_value = "control.txt")]
    configfile: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    pretty_env_logger::init();
    //    println!(
    //        "Host: {}, User: {}, Password: {}",
    //        args.server, args.user, args.password
    //    );

    cprintln!("\n<w!>ugsftp \n\n Dateien abgleich via sFTP zwecks Backup.<w!>\n");

    let mut locdir = String::from("/.");
    let mut remdir = String::from("/.");
    let mut fehlerlog = String::from("");
    //    let mut ddateiliste: Tdateiliste = HashMap::new();
    let mut qdateiliste: Tdateiliste = HashMap::new();
    let mut jobliste: Tdateiliste = HashMap::new();
    let mut fehlercnt: i32 = 0;
    let mut okcnt: i32 = 0;

    //    let controlfile = File::open("/etc/ugftp/control.txt").expect("");

    let mut configfilepath = String::from("/etc/ugftp/");
    configfilepath.push_str(&args.configfile);
    let hintfilename = configfilepath.clone();

    match File::open(Path::new(&configfilepath)) {
        Ok(controlfile) => {
            let file_reader = BufReader::new(controlfile);

            for (i, l) in file_reader.lines().enumerate() {
                let zeile = l?;
                //                println!("{} : {} ", i, zeile);
                let param: Vec<&str> = zeile.split("=").collect();
                if param[0] == "locdir" {
                    locdir = param[1].to_string();
                }
                if param[0] == "remdir" {
                    remdir = param[1].to_string();
                }
                if param[0] == "rmhost" {
                    args.server = param[1].to_string();
                }
                if param[0] == "kaewor" {
                    args.password = param[1].to_string();
                }
                if param[0] == "person" {
                    args.user = param[1].to_string();
                }
                if param[0] == "fehler" {
                    fehlerlog = param[1].to_string();
                }

                //                if param[0] != "" {
                //                    println!("param {}", param[0]);
                //                    println!("wert {}", param[1]);
                //                }
            }
        }
        Err(e) => {
            eprintln!(
                "controlfile {} konnte nicht geöffnet werden:\n  {}",
                hintfilename, e
            );
            process::exit(0x100);
        }
    }
    let mut tdest: String = args.server.clone();
    tdest.push_str(&remdir.as_str());

    cprintln!(
        "<w>von (source): {:?} nach (destination) {:?}</w>",
        locdir,
        tdest
    );
    //    if (args.server == '') { args.server }

    let mut client: SftpFs = SshOpts::new(args.server)
        .port(22)
        .username(args.user)
        .password(args.password)
        //        .config_file(Path::new("/home/cvisintin/.ssh/config"))
        .into();

    // connect
    assert!(client.connect().is_ok());
    // get working directory
    //    println!("Wrkdir: {}", client.pwd().ok().unwrap().display());
    // change working directory remdir.clone()
    let mut ziel = String::from(remdir.clone());
    assert!(client.change_dir(Path::new(&ziel)).is_ok());
    match client.change_dir(Path::new(&ziel)) {
        Ok(res) => {
            log::trace!("remote path: {:?}", res.to_str())
        }
        Err(err) => {
            log::error!("error change dir {:?}", err);
            fehlercnt = fehlercnt + 1;
        }
    }
    // disconnect
    let ddateiliste: HashMap<_, _> = match client.list_dir(Path::new("./")) {
        Ok(list) => {
            let filepath: Vec<_> = list.iter().map(|file| file.path()).collect();
            let vl = filepath.len();
            cprintln!("<w>Anzahl Files: {}</w>", vl);
            filepath
                .iter()
                .filter_map(|file_path| {
                    // .for_each(|file_path| {
                    match client.stat(file_path) {
                        Ok(statdata) => {
                            let _time: time::SystemTime = statdata.metadata.accessed.unwrap();
                            let _time_duration = _time.duration_since(SystemTime::UNIX_EPOCH);
                            let mut tv_sec = 0;
                            log::trace!("Datei alter {:?}", _time_duration);
                            match _time.duration_since(SystemTime::UNIX_EPOCH) {
                                Ok(n) => {
                                    tv_sec = n.as_secs() as i64;
                                }
                                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
                            }
                            let filename = match file_path.file_name() {
                                Some(name) => name.to_string_lossy(),
                                None => return None,
                            };
                            Some((filename.to_string(), tv_sec as i64))
                            //    ddateiliste.insert(filename.to_string(), tv_sec as i64);
                            //                        let tv_nsec: usize = now_splitted[1].parse().unwrap(); //129747070
                        }
                        Err(err) => {
                            log::error!("{}", err);
                            return None;
                        }
                    }
                })
                .collect()
        }
        Err(err) => {
            log::error!("{}", err);
            return Err(Box::new(err));
        }
    };
    //    println!("{:?}", ddateiliste);

    log::trace!("Lokales Verzeichnis: {:?}", locdir);
    let copyfrom = locdir.clone();
    let qdateiliste: HashMap<_, _> = fs::read_dir(locdir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            //            println!("entry {:?}",entry);
            let file_path = entry.path();
            let file = entry.path();
            let filename = match file.file_name() {
                Some(name) => name.to_string_lossy(),
                None => return None,
            };
            //            println!("Filename {:?}",filename);
            let mut _time = 0;
            if (file_path.is_dir()) {
                return None;
            }
            let metadata = fs::metadata(file_path)
                .map_err(|err| {
                    log::error!("faild to read metadata: {}", err);
                    err
                })
                .ok()?;
            //                println!("{:?}",metadata);

            let _time: time::SystemTime = metadata.accessed().unwrap();
            let _time_duration = _time.duration_since(SystemTime::UNIX_EPOCH);
            let tv_sec: i64; // = 0;
                             //                println!("{ctime:?}");
            match _time.duration_since(SystemTime::UNIX_EPOCH) {
                //                if let Ok(ctime) = metadata.created() {

                //                    match ctime.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => {
                    tv_sec = n.as_secs() as i64;
                }
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            }
            //                println!("Filename, time: {:?} {:?}", filename, _time);
            if (filename.starts_with(".")) {
                return None;
            }

            Some((filename.to_string(), tv_sec as i64))
        })
        .collect();

    //    println!("{:?}", qdateiliste);

    // Abgleich in neue Liste
    cprintln!("\n<w!>Dateien werden verglichen:</w!>\n");
    let jobliste: HashMap<_, _> = qdateiliste
        .iter()
        .filter_map(|(datei, alter)| {
            log::trace!("{:?} {:?}", datei, alter);
            let mut d_alter = 0;
            d_alter = match ddateiliste.get(datei) {
                Some(&dalter) => dalter,
                _ => {
                    log::trace!("{:?} noch nicht auf Zielverzeichnis", d_alter);
                    jobliste.insert(datei.to_string(), *alter as i64);
                    return None;
                }
            };
            cprintln!(
                "*\n*   <blue>Datei {:?}, Alter auf Ziel {:?}, Alter auf Quelle {:?}</blue> \n*",
                datei,
                d_alter,
                alter
            );
            if alter > &d_alter {
                cprintln!("<green>zu kopieren: {:?} </green>", datei);
                Some((datei.to_string(), *alter as i64))
            } else {
                cprintln!("<yellow>zu Ueberspringen: {:?} </yellow>", datei);
                None
            }
        })
        .collect();
    //    println!("{:?}", jobliste);

    const TBs1: u64 = 1024 * 1024 * 1024 * 1024;
    const TBn1: u64 = 1024 * 1024 * 1024 * 1024 - 1;
    // Liste hochladen

    if fehlercnt == 0 {
        cprintln!("\n<w!>Kopieren wird gestartet:</w!>\n");
        for (datei, a) in jobliste.iter() {
            let mut quelle = String::from(copyfrom.clone());
            let mut ziel = String::from(remdir.clone());
            quelle.push_str(&datei.to_string());
            ziel.push_str(&datei.to_string());
            log::trace!("zu kopieren: {:?} ", quelle);
            let quellpfad = Path::new(&quelle);
            let l_mdata = fs::metadata(quellpfad)?;
            let l_size = l_mdata.len().clone();
            //            let m_date = l_mdata.modified();
            //            let m_time: time::SystemTime = l_mdata.modified().unwrap();
            //            let m_time_duration = m_time.duration_since(SystemTime::UNIX_EPOCH);
            let mut mtime = SystemTime::now();
            if let Ok(time) = l_mdata.modified() {
                mtime = time;
            } else {
                println!("Time Error");
            }

            let mut metadata = remotefs::fs::Metadata::default();
            metadata.size = l_size.clone();
            metadata.mode = {
                Some(remotefs::fs::UnixPex::new(
                    remotefs::fs::UnixPexClass::new(true, true, true),
                    remotefs::fs::UnixPexClass::new(true, true, true),
                    remotefs::fs::UnixPexClass::new(false, false, false),
                ))
            };

            metadata.modified = Some(mtime);

            let zielpfad = Path::new(&ziel);
            let b: Box<dyn std::io::Read> = Box::new(std::fs::File::open(quellpfad)?);
            let (size, unit) = match l_size {
                0..=1023 => (l_size as f64, "Bytes"),
                1024..=1048575 => (l_size as f64 / 1024 as f64, "KB"),
                1048576..=TBn1 => (l_size as f64 / 1048576 as f64, "MB"),
                TBs1.. => (l_size as f64 / TBs1 as f64, "TB"),
            };

            cprintln!(
                "<w>Datei wird kopiert: {:?}, Größe {:.2} {}</w>",
                &datei.to_string(),
                size,
                unit
            );
            match client.create_file(zielpfad, &metadata, b) {
                Ok(result) => {
                    log::trace!("{:?}", result);
                    okcnt = okcnt + 1;
                }
                Err(err) => {
                    log::error!("{:?} {:?} {:?}", err, quellpfad, zielpfad);
                    fehlercnt = fehlercnt + 1;
                }
            }
        }
    }
    //    client.copy();

    if (!(fehlerlog.is_empty())) & (fehlercnt > 0) {
        let mut erFile = match File::create(fehlerlog) {
            Ok(efile) => {
                let mut _efile = efile;
                let txok = "kopierte Dateien ";
                let erstr = "kopierfehler Dateien:";
                let aet = "ERROR: Fehler bei sFTP put.";
                write!(_efile, "{}", aet);
                write!(_efile, "{}", txok);
                write!(_efile, "{}", erstr);
            }
            Err(err) => {
                eprintln!(
                    "controlfile /etc/ugftp/control.txt konnte nicht geöffnet werden:\n  {}",
                    err
                );
                return Err(Box::new(err));
            }
        };
    }

    client
        .disconnect()
        .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
}
