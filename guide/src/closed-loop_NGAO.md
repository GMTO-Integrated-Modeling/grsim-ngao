# Closed-loop NGAO

Building upon the [dynamic optical model](getting_started/dyn_optical_model.md), we are going to build a closed-loop NGAO system with a pyramid wavefront sensor ([P-WFS]).
First, we define the modal basis and the number of modes for M2:
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:26}}
```
and next we setup the [P-WFS]
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:15:22}}
```
The optical model is updated to include the [P-WFS]
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:41:45}}
```
where the [GMT builder](https://docs.rs/crseo/latest/crseo/struct.GmtBuilder.html) is invoked to set M2 and an [atmospheric turbulence model](https://docs.rs/crseo/latest/crseo/atmosphere/struct.AtmosphereBuilder.html#impl-Default-for-AtmosphereBuilder) has been added as well.

We then add a new flow inside actorscript with a new ouput to the optical model for the [detector frame] of the [P-WFS] 
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:55}}
```
The [detector frame] is being process with [P-WFS] data [processor]
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:24}}
```
and within actorscript, the [processor] receives the [detector frame] and outputs the pyramid [measurements]
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:55:56}}
```
The [measurements] are then transformed into coefficients of the modal basis by the [calibrator]
```rust,no_run,noplayground
let calibrator: Calibration<PyramidCalibrator> = {
    let mut pymtor = PyramidCalibrator::builder(pym.clone(), m2_modes, n_mode)
        .n_thread(7)
        .build()?;
    pymtor.h00_estimator()?;
    pymtor
}
.into()
```
The [calibrator] is linked to the [processor] and it outputs residual coefficients
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:55:57}}
```
Finally an [integral controller]:
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:51}}
```
accumulates the residual coefficients and sends the modal coefficients to the optical model that updates M2 figure
```rust,no_run,noplayground
{{#include ../../optical-model-pym/src/main.rs:55:59}}
```

[P-WFS]: https://docs.rs/crseo/latest/crseo/wavefrontsensor/struct.Pyramid.html
[processor]: https://docs.rs/gmt_dos-clients_crseo/latest/gmt_dos_clients_crseo/struct.Processor.html
[detector frame]: https://docs.rs/gmt_dos-clients_crseo/latest/gmt_dos_clients_crseo/struct.DetectorFrame.html
[measurements]: https://docs.rs/gmt_dos-clients_crseo/latest/gmt_dos_clients_crseo/enum.PyramidMeasurements.html
[calibrator]: https://docs.rs/gmt_dos-clients_crseo/latest/gmt_dos_clients_crseo/struct.Calibration.html
[integral controller]: https://docs.rs/gmt_dos-clients/latest/gmt_dos_clients/integrator/struct.Integrator.html