use std::{env, path::Path};

use gmt_dos_actors::actorscript;
use gmt_dos_clients::{print::Print, Integrator, Tick, Timer};
use gmt_dos_clients_crseo::{
    crseo::{
        atmosphere,
        wavefrontsensor::{LensletArray, Pyramid},
        Atmosphere, FromBuilder, Gmt,
    },
    Calibration, DetectorFrame, OpticalModel, Processor, PyramidCalibrator, PyramidMeasurements,
    ResidualM2modes,
};
use gmt_dos_clients_io::optics::{M2modes, WfeRms};
use pym_optical_gain::{Delay, OpticalGain};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sampling_frequency = 1000_f64;

    let data_repo =
        env::var("DATA_REPO").expect("missing `DATA_REPO` env var, set it to `pwd`/data");

    let n_lenslet: usize = 92;
    let pym = Pyramid::builder()
        .lenslet_array(LensletArray {
            n_side_lenslet: n_lenslet,
            n_px_lenslet: 10,
            d: 0f64,
        })
        .modulation(2., 64);

    let processor: Processor<_> = Processor::try_from(&pym)?;

    let (m2_modes, n_mode) = ("ASM_DDKLs_S7OC04184_675kls", 500);
    let calibrator: Calibration<PyramidCalibrator> = {
        let filename = format! {"pym-{m2_modes}-{n_mode}.bin"};
        if let Ok(pymtor) =
            PyramidCalibrator::try_from(Path::new(&data_repo).join(&filename).to_str().unwrap())
        {
            pymtor
        } else {
            let mut pymtor = PyramidCalibrator::builder(pym.clone(), m2_modes, n_mode)
                .n_thread(7)
                .build()?;
            pymtor.h00_estimator()?;
            pymtor.save(filename)?
        }
        .into()
    };

    let atm_builder = Atmosphere::builder().ray_tracing(
        atmosphere::RayTracing::default()
            .duration(5f64)
            .filepath(Path::new(&data_repo).join("atmosphere.bin")),
    );

    let optical_model = OpticalModel::<Pyramid>::builder()
        .gmt(
            Gmt::builder()
                .m1_truss_projection(false)
                .m2(m2_modes, n_mode),
        )
        .sensor(pym)
        .atmosphere(atm_builder)
        .sampling_frequency(sampling_frequency)
        .build()?;

    let metronome: Timer = Timer::new(100);

    let prt = Print::default();

    let pym_ctrl = Integrator::<ResidualM2modes>::new(n_mode * 7).gain(0.5);

    actorscript!(
        #[model(name=ngao)]
        1: metronome[Tick] -> optical_model[WfeRms<-9>] -> prt
        1: optical_model[DetectorFrame]
            -> processor[PyramidMeasurements]
                -> calibrator[ResidualM2modes]
                    -> pym_ctrl[M2modes]!
                        -> optical_model
    );

    let optical_gain = OpticalGain::new(sampling_frequency as f64, n_mode);
    let wait_for_it = Delay::default();
    let metronome: Timer = Timer::new(1000);

    actorscript!(
        #[model(name=ngao_optical_gain )]
        1: metronome[Tick] -> optical_model
        1: optical_model[DetectorFrame]
            -> processor[PyramidMeasurements]
                -> calibrator[ResidualM2modes]
                    -> pym_ctrl[M2modes]!
                        -> optical_gain[M2modes]!
                            -> optical_model
        1: calibrator[ResidualM2modes]
            -> wait_for_it[ResidualM2modes]!
                -> optical_gain
    );

    println!("{}", optical_gain.lock().await.gain());

    Ok(())
}
