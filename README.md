# ckb-miner

| PoW algorithm | Since | 
| ----          | ----    |
|   Eaglesong   | v0.19.0 |

A miner for CPUs, avx2 CPUs, avx512 CPUs and GPUs.

Using avx512, the hash rate can be up to 10M per thread on 2.5Ghz CPUs.

## How to build
By default, GPU miner is not built, you can build GPU miner with this parameters:

```
# for cuda
cargo build --release --features cuda

#for opencl
cargo build --release --features opencl
```

### CUDA runtime

By default, the cuda lib path is C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v10.1\lib\x64 for Windows and /usr/local/cuda/lib64 for other platforms, you can change this by set `CUDA_LIB_DIR`.

For Linux and OSX:

```
export CUDA_LIB_DIR=<PATH> # <PATH> is the path where you install CUDA, eg export CUDA_LIB_DIR=/usr/local/cuda/lib64

```

For windows:

```
set CUDA_LIB_DIR=<PATH> # <PATH> is the path where you install CUDA, eg set CUDA_LIB_DIR=C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v10.1\\lib\\x64
```

Cuda is essential to build and run miners based on Cuda.

#### Performance

| GPU       | 1060 | 1070 ti | P106 | 2060 |
| ---       | ---  | ---     | ---- | ---  |
| Hash Rate | 400M  | 630M     | 420M  |  650M   |

### OpenCL runtime
With OpenCL, the miner can run on any GPUs.

By default, the OpenCL include path is C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v10.1\include for Windows and /usr/include for other platforms, you can change this by set `OPENCL_INCLUDE_DIR`; the OpenCL lib path is C:\Program Files\NVIDIA GPU Computing Toolkit\CUDA\v10.1\lib\x64 for Windows and default lib path for other platforms, you can change this by set `OPENCL_LIB_DIR`

#### Performance

| GPU       | 1060 | 1070 ti | P106 | 2060 | VEGA64 | RX5700 | RX580 | RX570 |
| ---       | ---  | ---     | ---- | ---  | ---  | ---    | ---  | --- |
| Hash Rate | 380M | 620M    | 410M | 610M | 820M | 560M   | 340M |  280M   |

## How to use
1. Modify `ckb-miner.toml` in this repo and copy it to your working directory. You can specify the CPUs and GPUs you want to use.

2. Copy `ckb-miner` (you can build it yourself or download it) to your working directory.

3. run the miner

```
./ckb-miner
```

