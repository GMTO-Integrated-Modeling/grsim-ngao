use gmt_dos_actors::actorscript;
use gmt_dos_clients::{print::Print, Tick, Timer};
use gmt_dos_clients_crseo::OpticalModel;
use gmt_dos_clients_crseo::{GuideStar, WavefrontStats};
use gmt_dos_clients_io::optics::WfeRms;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let optical_model = OpticalModel::builder().build()?;

    let metronome: Timer = Timer::new(10);

    let stats: WavefrontStats = Default::default();

    let prt = Print::new(3);

    actorscript!(
        1: metronome[Tick] -> optical_model[GuideStar] -> stats[WfeRms<-9>] -> prt
    );

    Ok(())
}
