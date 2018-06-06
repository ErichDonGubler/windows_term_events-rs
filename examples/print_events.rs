extern crate windows_term_events;
#[macro_use]
extern crate quicli;

use windows_term_events::ConsoleEventReader;

main!(|| {
    let mut reader = ConsoleEventReader::from_stdin(&Default::default())?;
    println!(
        "events: {:#?}",
        reader.read(200)?
    );
});
