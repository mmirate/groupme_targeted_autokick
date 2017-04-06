#![recursion_limit = "2048"]
#![deny(warnings)]
extern crate chrono;
extern crate hyper;
extern crate multipart;
extern crate rand;
extern crate regex;
extern crate rustc_serialize;
extern crate rusttype;
extern crate scraper;
extern crate time;
extern crate url;
extern crate users;
extern crate image;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate error_chain;
pub mod groupme;
pub mod render;

pub mod errors {
    error_chain! {
        foreign_links {
            Hyper (::hyper::Error);
            Io (::std::io::Error);
            JsonEncoding (::rustc_serialize::json::EncoderError);
            JsonDecoding (::rustc_serialize::json::DecoderError);
            JsonParsing (::rustc_serialize::json::ParserError);
            UrlParsing (::url::ParseError);
            DateFormatParsing (::chrono::format::ParseError);
            EnvironmentVar (::std::env::VarError);
        }
        errors {
            RLE {}
            HttpError(s: ::hyper::status::StatusCode) {
                from()
                description("HTTP request returned error.")
                display("HTTP error: {:?}.", s)
            }
            TextTooLong(text: String, attachments: Option<Vec<::rustc_serialize::json::Json>>) {
                description("A message was too long for GroupMe.")
                display("Message {:?} cannot be sent via GroupMe.", text)
            }
        }
    }
}
            //BadPixelFormat {}
            //BadBufferSize {}
//    use futures::Future;
//    use futures::stream::Stream;
//    pub type Future<T> = Future<Item=T, Error=Error>;
//    pub type Stream<T> = Stream<Item=T, Error=Error>;

//pub mod groupme_syncer {
//    use syncer;
//    use postgres;
//    use groupme;
//    use groupme::BidirRecipient;
//    use errors::*;

//    #[derive(Debug)] pub struct GroupmeSyncer { pub group: groupme::Group, pub last_message_id: Option<String>, pub members: Vec<groupme::Member>, conn: postgres::Connection, }
//    impl GroupmeSyncer {
//        pub fn new(group: groupme::Group) -> Result<GroupmeSyncer> {
//            let conn = try!(syncer::setup());
//            let last_message_id = syncer::read(&conn, (group.group_id.clone() + "last_message_id").as_str()).ok();
//            Ok(GroupmeSyncer { group: group, last_message_id: last_message_id, members: vec![], conn: conn })
//        }
//        pub fn update_messages(&mut self) -> Result<Vec<groupme::Message>> {
//            let last_message_id = self.last_message_id.clone();
//            let selector = last_message_id.clone().map(groupme::MessageSelector::After);
//            //let selector = match last_message_id {
//            //    Some(ref m) => Some(groupme::MessageSelector::After(m.clone())),
//            //    None => None,
//            //};
//            let ret = try!(self.group.slurp_messages(selector.clone()));
//            self.last_message_id = if ret.len() > 0 { Some(ret[ret.len()-1].id.clone()) } else { last_message_id };
//            if let Some(ref last_message_id) = self.last_message_id { try!(syncer::write(&self.conn, (self.group.group_id.clone() + "last_message_id").as_str(), last_message_id.as_str())); }
//            if let Some(_) = selector { Ok(ret) } else { Ok(vec![]) }
//        }
//    }
//}
