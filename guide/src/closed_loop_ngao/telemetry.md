# Telemetry

A new dependency is required in order to log data from any model:
 ```bash
cargo add gmt_dos-clients_arrow
```

Telemetry or data logging is achieve by apending a `$` [sign](https://docs.rs/gmt_dos-actors_dsl/latest/gmt_dos_actors_dsl/macro.actorscript.html#client-output-pair) to an output with the size of the output within braces:
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
