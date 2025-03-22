mod path;

use anyhow::{Error, Result};
use fmod::c;
use tracing::info;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let mut builder = unsafe { fmod::studio::SystemBuilder::new()? };
    info!("Builder created");

    builder
        .core_builder()
        .software_format(0, fmod::SpeakerMode::Stereo, 0)?;

    let system = builder.build(
        1024,
        fmod::studio::InitFlags::NORMAL,
        fmod::InitFlags::NORMAL,
    )?;
    info!("System created");

    let bank1_path_cstring = path::path("Master.bank");
    let bank1_path = fmod::Utf8CStr::from_cstr(&bank1_path_cstring).unwrap();
    system.load_bank_file(&bank1_path, fmod::studio::LoadBankFlags::NORMAL)?;
    let bank2_path_cstring = path::path("Master.strings.bank");
    let bank2_path = fmod::Utf8CStr::from_cstr(&bank2_path_cstring).unwrap();
    system.load_bank_file(&bank2_path, fmod::studio::LoadBankFlags::NORMAL)?;

    info!("Banks loaded");

    info!("Loading event");
    let event_jump_description = system.get_event(c!("event:/Jump"))?;
    let event_jump_instance = event_jump_description.create_instance()?;
    info!("Event created");

    let mut last_jump_started = std::time::Instant::now();
    let mut event_jump_plays = 0;
    loop {
        system.update()?;

        if last_jump_started.elapsed().as_secs() > 1 {
            event_jump_instance.start()?;
            event_jump_plays += 1;
            last_jump_started = std::time::Instant::now();
            info!("Event started");
        }

        if event_jump_plays > 5 {
            break;
        }
    }

    unsafe {
        system.release()?;
    }
    info!("System released");

    Ok(())
}
