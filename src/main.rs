
use std::io::{ self, Read };
use std::fs::File;
use std::process::ExitCode;

use palaver;
use zip::{ read::ZipArchive, result::ZipError };

use worst::reader;
use worst::interpreter::*;
use worst::builtins;

fn error(msg: &str) -> io::Error {
    io::Error::new(io::ErrorKind::Other, msg)
}

fn zip_open_self() -> io::Result<Option<ZipArchive<File>>> {
    let exe = palaver::env::exe()?;
    match ZipArchive::new(exe) {
        Ok(z) => Ok(Some(z)),
        Err(ZipError::Io(e)) => Err(e),
        Err(ZipError::FileNotFound) => Err(io::Error::new(io::ErrorKind::NotFound, "not found")),
        Err(ZipError::InvalidArchive(_) | ZipError::UnsupportedArchive(_)) => Ok(None),
    }
}

fn open_zip_initw() -> io::Result<Option<Interpreter>> {
    let Some(mut archive) = zip_open_self()? else { return Ok(None); };
    let mut initw = match archive.by_name("init.w") {
        Ok(f) => f,
        Err(ZipError::Io(e)) => return Err(e),
        Err(ZipError::FileNotFound) => return Ok(None),
        Err(ZipError::InvalidArchive(msg)) => 
            return Err(io::Error::new(io::ErrorKind::InvalidData, msg)),
        Err(ZipError::UnsupportedArchive(msg)) => 
            return Err(io::Error::new(io::ErrorKind::Other, msg)),
    };
    let mut s = String::new();
    initw.read_to_string(&mut s)?;
    let body = reader::read_all(&mut s.chars()).unwrap();
    Ok(Some(Interpreter::new(body)))
}

fn open_arg1() -> io::Result<Option<Interpreter>> {
    let Some(f) = std::env::args().nth(1) else { return Ok(None); };
    let contents = std::fs::read_to_string(f)?;
    let body = reader::read_all(&mut contents.chars()).unwrap();
    Ok(Some(Interpreter::new(body)))
}

fn find_entrypoint() -> io::Result<Interpreter> {
    if let Some(r) = open_zip_initw()? { return Ok(r); }
    if let Some(r) = open_arg1()? { return Ok(r); }
    Err(error("no entrypoint"))
}

fn main_ioresult() -> io::Result<()> {
    let mut i = find_entrypoint()?;
    builtins::install(&mut i);
    if let Err(_e) = i.run() {
        Err(error("interpreter error"))
    } else {
        Ok(())
    }
}

fn main() -> ExitCode {
    if let Err(e) = main_ioresult() {
        eprintln!("{}", e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

