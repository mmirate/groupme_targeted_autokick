# GroupMe Targeted Autokicker

## Background

If your GroupMe group is being hassled by someone who leaves before they can be kicked, this should help.

## Prerequisites

- A [Heroku](https://heroku.com) account.
- An [Uptime Robot](https://uptimerobot.com) account.

## Dependencies

This program depends on the Rust-language compiler, toolkit, and many third-party libraries. However, one part of the software used to deploy the program to Heroku (namely, the "buildpack") downloads all of this onto storage space associated with your Heroku account, so you do not need to install any software on your own machine.

## How to Use

(Technical users: it is possible to run this on your own machine if it's a modern Unix-like, but you'll need to figure everything out.)

### Deploy

Click this button:

[![Deploy](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

... and fill out the form with the information needed to deploy a copy of this program onto Heroku. Once you've clicked the big purple "Deploy" button at the bottom of that page, grab a donut or something while the program compiles. Finally, after completion, click the "View" button for further instructions.

You should leave the program up-and-running on Heroku until it triggers and kicks the troublemaker.

## Limitations

- Error handling is primitive (`try!(x :: Result<T, Box<Error>>)` at best; `unwrap()` at worst). Most errors will print an error and halt the current poll; non-transient errors (e.g. network down) will additionally spam stderr with highly-cryptic messages and possibly abort the program; if you're running it locally, you should take care that the program is restarted upon aborting.

