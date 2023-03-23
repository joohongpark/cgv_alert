use quick_xml::reader::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
pub struct SeatInfo {
  pub x: String,
  pub y: String,
  pub kind: String,
  pub useable: String,
}

pub fn parser(xml: std::string::String) -> Vec<SeatInfo> {
  let mut buf = Vec::new();
  let mut result = Vec::new();
  let mut seat_begin = false;
  let mut element = Vec::new();
  let mut x = String::new();
  let mut y = String::new();
  let mut kind = String::new();
  let mut useable = String::new();

  let mut reader = Reader::from_str(&xml);
  reader.trim_text(true);
  loop {
    match reader.read_event_into(&mut buf) {
        // Ok(Event::Start(e)) => println!("start : {:?}", e),
        Ok(Event::Start(e)) => {
          match e.name().as_ref() {
            b"SEAT_INFO" => seat_begin = true,
            name => element = name.to_owned(),
          }
        },
        // Ok(Event::Text(e)) => println!("{}", e.unescape().unwrap().into_owned()),
        Ok(Event::Text(e)) => {
          match element.as_slice() {
              b"LOC_Y_NM" => y = e.unescape().unwrap().into_owned(),
              b"SEAT_NO" => x = e.unescape().unwrap().into_owned(),
              b"RATING_NM" => kind = e.unescape().unwrap().into_owned(),
              b"SEAT_STATE" => useable = e.unescape().unwrap().into_owned(),
              _ => (),
          }
        },
        // Ok(Event::End(e)) => println!("end : {:?}", e),
        Ok(Event::End(e)) => {
          match e.name().as_ref() {
            b"SEAT_INFO" => {
              seat_begin = false;
              result.push(SeatInfo {
                x: String::from(x.to_owned()),
                y: String::from(y.to_owned()),
                kind: String::from(kind.to_owned()),
                useable: String::from(useable.to_owned()),
              });
            },
            _ => element = Vec::new(),
          }
        },

        // exits the loop when reaching end of file
        Ok(Event::Eof) => break,
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        // There are several other `Event`s we do not consider here
        _ => (),
    }
    // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    buf.clear();
  }
  result
}