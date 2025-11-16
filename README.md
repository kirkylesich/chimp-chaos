# Chimp Chaos Operator

Minimal Kubernetes operator for chaos engineering experiments.

## Features

- Type-safe CRD definitions
- Simple reconciliation loop
- Three chaos scenarios: PodKiller, CpuStress, NetworkDelay
- Clean logging
- Minimal dependencies

## Quick Start

### 1. Install CRD

```bash
kubectl apply -f crd.yaml
```

### 2. Build and run operator

```bash
cargo build --release
cargo run
```

### 3. Create experiment

```bash
kubectl apply -f examples/pod-killer.yaml
```

### 4. Watch experiments

```bash
kubectl get chaos -w
```

### 5. Delete experiment

```bash
kubectl delete chaos pod-killer-test
```

## Supported Scenarios

- **PodKiller** - pod termination simulation
- **CpuStress** - CPU load simulation
- **NetworkDelay** - network latency simulation

## CRD Structure

```yaml
apiVersion: chaos.io/v1
kind: ChaosExperiment
metadata:
  name: my-experiment
  namespace: default
spec:
  scenario: PodKiller
  duration: 300
  targetNamespace: default
```

## Experiment Phases

- **Pending** - experiment created, waiting to start
- **Running** - experiment is executing
- **Succeeded** - experiment completed successfully
- **Failed** - experiment failed

## Development

```bash
cargo check
cargo fmt
cargo clippy
```
