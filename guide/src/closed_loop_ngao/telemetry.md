# Telemetry

A new dependency is required in order to log data from any model:
 ```bash
cargo add gmt_dos-clients_arrow
```

Telemetry or data logging is achieve by appending a `$` [token](https://docs.rs/gmt_dos-actors_dsl/latest/gmt_dos_actors_dsl/macro.actorscript.html#client-output-pair) to an output.
The size of the ouput that is recorded needs to be know by the data logger and it is provided by the [Size](https://docs.rs/gmt_dos-actors-clients_interface/1.0.0/gmt_dos_actors_clients_interface/trait.Size.html) trait,
if it is implemented for the client of this output like [`WfeRms`](https://docs.rs/gmt_dos-clients_crseo/latest/gmt_dos_clients_crseo/struct.OpticalModel.html#impl-Size%3CWfeRms%3E-for-OpticalModel%3CT%3E).
The size of the output can also be passed on by inserting it within braces.
```rust,no_run,noplayground
{{#include ../../../optical-model-pym/src/main.rs:54}}
{{#include ../../../optical-model-pym/src/main.rs:60}}

```
The source wavefront error RMS is logged into the [Apache Parquet](https://parquet.apache.org/) file: `model-data_1.parquet`
and M2 modal coefficients are decimated by a factor 10 and written to `model-data_10.parquet`.

The [Parquet](https://parquet.apache.org/) files can be read in Python with `pandas` and the data can be loaded into `numpy` arrays, e.g.
```python
import pandas as pd
import numpy as np
df = pd.read_parquet("model-data_1.parquet")
wfe_rms = np.vstack(df["WfeRms<-9"])
df = pd.read_parquet("model-data_10.parquet")
modes = np.vstack(df['M2modes'])
```
Each row of the `numpy` array is a time sample.

Sometimes, only data from the last step of a simulation is needed.
Noting that actor's client are persistent, meaning that they outlive the model they are part of,
another model can then be used to read some data from clients from a previously completed model:
```rust,no_run,noplayground
{{#include ../../../optical-model-pym/src/main.rs:63:67}}
