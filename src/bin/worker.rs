#[macro_use] extern crate chan;
extern crate chan_signal;
#[macro_use] extern crate clap;
extern crate groupme_targeted_autokick;
#[macro_use] extern crate error_chain;
use groupme_targeted_autokick::*;
use groupme_targeted_autokick::errors::*;

use std::io::Write;

quick_main!(run);

fn iteration(g: &mut groupme::Group, targetname: &str) -> Result<()> {
    try!(g.refresh());
    for m in g.members.iter() {
        if m.nickname.as_str() == targetname {
            try!(g.remove(m.clone()));
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    let _matches = clap::App::new("GroupMe targeted autokicker").version(crate_version!()).author(crate_authors!()).get_matches();
    let targetname = try!(std::env::var("TARGET_NAME")).to_string();
    let groupname = try!(std::env::var("GROUP_NAME")).to_string();
    let mut group = None;
    {
        let mut allgroups = try!(groupme::Group::list());
        allgroups.sort_by(|a, b| a.members.len().cmp(&b.members.len()));
        while let Some(g) = allgroups.pop() {
            if group.is_none() && g.name == groupname && g.members.len() > 2 {
                std::mem::replace(&mut group, Some(g));
                break;
            }
        }
    }
    let mut group = group.expect("Cannot find Group");
    let signal = chan_signal::notify(&[chan_signal::Signal::TERM, chan_signal::Signal::INT, chan_signal::Signal::HUP]);
    println!("Alive!");
    loop {
        if let Err(e) = iteration(&mut group, &targetname) {
            try!(std::io::stderr().write(format!("\x07ERROR: {}", e).as_bytes()));
            for e in e.iter().skip(1) {
                try!(std::io::stderr().write(format!("caused by: {}", e).as_bytes()));
            }
            if let Some(backtrace) = e.backtrace() {
                try!(std::io::stderr().write(format!("backtrace: {:?}", backtrace).as_bytes()));
            }
            try!(std::io::stderr().write(format!("If you see this error repeatedly, please fix it.").as_bytes()));
        }
        println!("Tick.");
        let start_of_sleep = std::time::Instant::now();
        while start_of_sleep.elapsed().as_secs() < 1 {
            chan_select! {
                default => std::thread::sleep(std::time::Duration::new(0,250)),
                signal.recv() -> signal => {
                    println!("Exiting in receipt of {:?}.", signal);
                    return Ok(());
                }
            }
        }
    }
}
