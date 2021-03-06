extern crate rustc_serialize;
extern crate clap;
extern crate groupme_targeted_autokick;
use groupme_targeted_autokick::*;
use groupme_targeted_autokick::errors::*;

#[macro_use] extern crate error_chain;

quick_main!(|| -> Result<()> {
    println!("You are a member of the following groups:");
    println!("");
    println!("#Ppl\tID      \tName");
    println!("====\t========\t====");
    for g in try!(groupme::Group::list()) {
        println!("{}\t{}\t{}", g.members.len(), g.group_id, g.name);
    }
    Ok(())
});
