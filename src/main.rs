use anyhow::{Error, Result};
use fmod::c;
use tracing::info;

fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let mut builder = unsafe { fmod::studio::SystemBuilder::new()? };
    info!("Builder created");

    builder
        .core_builder()
        .software_format(0, fmod::SpeakerMode::FivePointOne, 0)?;

    let system = builder.build(
        1024,
        fmod::studio::InitFlags::NORMAL,
        fmod::InitFlags::NORMAL,
    )?;
    info!("System created");

    system.load_bank_file(
        c!("fmod/main/Build/Desktop/Master.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    system.load_bank_file(
        c!("fmod/main/Build/Desktop/Master.strings.bank"),
        fmod::studio::LoadBankFlags::NORMAL,
    )?;
    info!("Banks loaded");

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
