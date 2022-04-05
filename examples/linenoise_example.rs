extern crate linenoise;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::Mutex;

struct Dictionary
{
  map: BTreeMap< String, String >,
}

impl Dictionary
{
  fn new( ) -> Self {
    Self { map: BTreeMap::new( ) }
  }

  fn insert( &mut self, key: &str, val: &str ) {
    self.map.insert( key.to_string( ), val.to_string( ) );
  }

  fn matches( &mut self, input: &str ) -> Vec< String > {
    let mut result: Vec< String > = vec![ ];
    let range = self.map.range( input.to_string( ) .. );
    for item in range {
      if item.0.starts_with( input ) {
        result.push( item.0.to_string( ) );
      }
    }
    result
  }
}

fn main() {
  let mut dict = Dictionary::new( );

  dict.insert( "clear", "Clear screen" );
  dict.insert( "help", "This info" );
  dict.insert( "history", "Is wot happened before innit" );

  let ptr = Arc::new( Mutex::new( dict ) );
  let weak = Arc::downgrade( &ptr );
  linenoise::set_callback_with_fn( move | input | {
    if let Some( dict ) = weak.upgrade( ) {
      let mut lock = dict.lock( ).unwrap( );
      lock.matches( input )
    }
    else {
      Vec::new( )
    }
  } );

  linenoise::set_multiline(0);

  loop {
    let val = linenoise::input("Hello Dave -> ");
    match val {
      None => { break }
      Some(input) => {
        println!("> {}", input);
        linenoise::history_add(input.as_ref());
        let is: &str = input.as_ref();
        if is == "clear" {
          linenoise::clear_screen();
        } else if is == "history" {
          let mut index = 0;
          loop {
            match linenoise::history_line(index) {
              None => { break; },
              Some(line) => {
                println!("{}: {}", index, line);
              }
            };
            index = index + 1;
          }
        }
      }
    }
  }
  linenoise::history_free( );
}
