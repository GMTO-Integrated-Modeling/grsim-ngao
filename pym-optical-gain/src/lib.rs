use std::{f64::consts::PI, fmt::Display, sync::Arc};

use gmt_dos_clients_crseo::ResidualM2modes;
use gmt_dos_clients_io::optics::M2modes;
use interface::{Data, Read, UniqueIdentifier, Update, Write};

/// Probe amplitude
const PROBE_AMPLITUDE: f64 = 1e-8;
/// Probing signals for outer segment
const OUTERS_PROBE: ProbingSignals<6> = ProbingSignals {
    sid: [1, 3, 5, 2, 4, 6],
    mode: [7, 40, 105, 192, 318, 460],
    frequency: [210f64; 6],
};
const CENTER_PROBE: ProbingSignals<4> = ProbingSignals {
    sid: [7; 4],
    mode: [7, 105, 251, 401],
    frequency: [80., 210., 310., 133.],
};

/// Delay
///
/// Sends an empty vector until the counter
/// crosses the specified threshold (default: 0).
#[derive(Debug, Default)]
pub struct Delay<T = f64> {
    data: Arc<Vec<T>>,
    count: usize,
    threshold: usize,
}

impl<T: Default> Delay<T> {
    /// Creates a new [Delay] instance with the specified threshold.
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            ..Default::default()
        }
    }
}

impl<T: Send + Sync> Update for Delay<T> {}

impl<T, U> Read<U> for Delay<T>
where
    U: UniqueIdentifier<DataType = Vec<T>>,
    Delay<T>: Update,
{
    fn read(&mut self, data: Data<U>) {
        self.count += 1;
        if self.count > self.threshold {
            self.data = data.into_arc();
        }
    }
}
impl<T, U> Write<U> for Delay<T>
where
    U: UniqueIdentifier<DataType = Vec<T>>,
    Delay<T>: Update,
{
    fn write(&mut self) -> Option<Data<U>> {
        Some((&self.data).into())
    }
}

/// Optical gain probe signal
#[derive(Debug, Default)]
pub struct Probe {
    sid: u8,
    mode: usize,
    n_mode: usize,
    amplitude: f64,
    frequency: f64,
    sampling_frequency: f64,
    signal: Vec<f64>,
    filtered: Vec<f64>,
    i: usize,
    gain: f64,
}

impl Display for Probe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            " * S{}#{:>3}({:3.0}Hz): {:.3}",
            self.sid, self.mode, self.frequency, self.gain
        )
    }
}

fn variance(data: &[f64]) -> f64 {
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    variance
}

impl Probe {
    /// Returns the probe index
    pub fn id(&self) -> usize {
        (self.sid as usize - 1) * self.n_mode + self.mode
    }
    /// Adds modulation to a given signal
    pub fn modulate(&mut self, signal: &mut f64) {
        let w = 2. * PI * self.frequency * self.i as f64 / self.sampling_frequency;
        let m = self.amplitude * w.sin();
        *signal += m;
        self.signal.push(*signal);
        self.i += 1
    }
    /// Evaluates the optical gain
    pub fn gain(&mut self) {
        let (re_s, im_s, re_f, im_f) = self.signal.iter().zip(&self.filtered).enumerate().fold(
            (0f64, 0f64, 0f64, 0f64),
            |(re_s, im_s, re_f, im_f), (i, (s, f))| {
                let (sin, cos) =
                    (2. * PI * self.frequency * i as f64 / self.sampling_frequency).sin_cos();
                (
                    re_s + s * cos,
                    im_s + s * sin,
                    re_f + f * cos,
                    im_f + f * sin,
                )
            },
        );
        let d_s = re_s * re_s + im_s * im_s;
        let d_f = re_f * re_f + im_f * im_f;
        self.gain = (d_f / d_s).sqrt();
    }
    pub fn signal_variance(&self) -> f64 {
        variance(&self.signal)
    }
    pub fn filtered_variance(&self) -> f64 {
        variance(&self.filtered)
    }
    pub fn variance_ratio(&self) -> f64 {
        self.filtered_variance() / self.signal_variance()
    }
}

/// Probing signals
pub struct ProbingSignals<const N: usize> {
    sid: [u8; N],
    mode: [usize; N],
    frequency: [f64; N],
}

impl<const N: usize> IntoIterator for ProbingSignals<N> {
    type Item = (u8, usize, f64);

    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(
            self.sid
                .into_iter()
                .zip(self.mode)
                .zip(self.frequency)
                .map(|((sid, mode), frequency)| (sid, mode, frequency)),
        )
    }
}

/// Optical gain client
#[derive(Default)]
pub struct OpticalGain {
    probes: Vec<Probe>,
    data: Vec<f64>,
}

impl Display for OpticalGain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Optical Gain:")?;
        self.probes.iter().map(|probe| probe.fmt(f)).collect()
    }
}
impl OpticalGain {
    /// Creates a new [OpticalGain] instance with the specified sampling frequency.
    pub fn new(sampling_frequency: f64, n_mode: usize) -> Self {
        let probes: Vec<_> = OUTERS_PROBE
            .into_iter()
            .chain(CENTER_PROBE.into_iter())
            .map(|(sid, mode, frequency)| Probe {
                sid,
                mode,
                n_mode,
                amplitude: PROBE_AMPLITUDE,
                frequency,
                sampling_frequency,
                ..Default::default()
            })
            .collect();
        Self {
            probes,
            data: vec![0f64; n_mode * 7],
        }
    }
    /// Evaluates the optical gain for each probe
    pub fn gain(&mut self) -> &mut Self {
        self.probes.iter_mut().for_each(Probe::gain);
        self
    }
    pub fn gain_from_variance(&self) -> Vec<f64> {
        self.probes
            .iter()
            .map(Probe::variance_ratio)
            .map(|x| (2. * x).sqrt())
            .collect()
    }
}

impl Update for OpticalGain {
    fn update(&mut self) {
        self.probes.iter_mut().for_each(|probe| {
            probe.modulate(&mut self.data[probe.id()]);
        });
    }
}

impl Write<M2modes> for OpticalGain {
    fn write(&mut self) -> Option<Data<M2modes>> {
        Some(self.data.clone().into())
    }
}

impl Read<M2modes> for OpticalGain {
    fn read(&mut self, data: Data<M2modes>) {
        self.data = data.into();
    }
}

impl Read<ResidualM2modes> for OpticalGain {
    fn read(&mut self, data: Data<ResidualM2modes>) {
        if !data.is_empty() {
            self.probes.iter_mut().for_each(|probe| {
                probe.filtered.push(data[probe.id()]);
            });
        }
    }
}
