extern crate xml;

use std::env;

use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use std::str::FromStr;


struct SvgRectSource {
    reader : EventReader<BufReader<File>>
}

impl SvgRectSource {
    fn new(file_name: &String) -> SvgRectSource {
        let file =
            File::open(file_name).expect("Cannot open input file");
        let buf_file =
            BufReader::new(file);

        let parser = EventReader::new(buf_file);

        SvgRectSource {
            reader: parser
        }
    }
}

struct Rect {
    x : f64,
    y : f64,
    w : f64,
    h : f64
}

impl Iterator for SvgRectSource {
    type Item = Rect;
    fn next(&mut self) -> Option<Rect> {
        //        for e in self.reader {
        //for e in r {
        println!("<>?<?");
        if let Err(err) = self.reader.next() {
            println!("e {}" , err);
        }
        while let Ok(e) = self.reader.next() {
            match e {
                XmlEvent::StartElement { name, attributes, ..}  => {
                    println!("++ {}", name.local_name);
                    if name.local_name == "rect" {
                        let mut x : f64 = 0.0;
                        let y : f64 = 0.0;
                        let w : f64  = 0.0;
                        let h : f64 = 0.0;
                        for att in attributes.iter() {
                            match (att.name.local_name.as_str(),
                                   f64::from_str(att.value.as_str())) {
                                ("x", Ok(x_value))  =>  {
                                    x = x_value
                                }
                                (_, _) => { }
                            }
                        }
                        return Some(Rect {
                            x: x,
                            y: y,
                            w: w,
                            h: h
                        });
                    } else {
                        return None;
                    }
                }
                XmlEvent::StartDocument {..} => {
                    
                }
                XmlEvent::EndDocument {..}  => {
                    return None;
                }
                _ => {
                    println!(">>>>");
                    return None;
                }
            }
        }
        
        return None;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Working number of arguments");
        return;
           
    }
    println!("Reading  : {}", args[1]);

    let source = SvgRectSource::new(&args[1]);

    for e in source {
        println!("-- {}", e.x);
    }

    // let file =
    //     File::open(&args[1]).expect("Cannot open input file");
    // let buf_file =
    //     BufReader::new(file);

    // let parser = EventReader::new(buf_file);
    // for e in parser {
    //     match e {
    //         Ok(XmlEvent::StartElement { name, ..}) => {
    //             println!("\t Element name: {}", name);
    //         }
    //         Ok(_) => {
    //             println!("Other kind of element");
    //         }
    //         Err(err)  =>{
    //             println!("Unexpected {}", err);
    //         }
    //     }
    // }
}
