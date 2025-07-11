

# ⏳ ChronoVote: Time-Decay Threshold Consensus

ChronoVote implements an advanced consensus and voting mechanism that balances fairness, liveness, and security through **time-decay weighted voting** and **dynamic threshold escalation**.

It is designed for decentralized systems where early participation and ongoing consensus safety are critical, such as blockchain governance, distributed networks, or fault-tolerant protocols.

---

## 🌟 Core Features

### 📈 Time-Weighted Voting System

* **Time-decay voting**: vote weight decreases over time based on configurable decay functions — incentivizing early participation while keeping later votes meaningful.
* Supported decay models:

  *  *Exponential*: aggressive early advantage.
  *  *Linear*: gradual decline.
  *  *Stepped*: discrete phases with sudden drops.
* Cryptographically verifiable vote timestamps (validator signatures + NTP).
* Minimum weight floor (e.g., 10% of original) to prevent votes from becoming worthless.
* Real-time weight calculation engine with continuous updates as votes arrive.

---

### 🚀 Dynamic Threshold Escalation Engine

* Consensus threshold starts at a **base level** (e.g., 51%) and **increases over time** via an escalation function.
* Supported escalation patterns:

  * Linear (+1% per minute)
  * Exponential growth
  * Sigmoid curve
  * Custom step functions
* **Threshold ceiling** (e.g., 90%) ensures liveness even under low participation.
* Emergency override thresholds for critical proposals.
* Built-in mathematical proof engine validates that escalation rules preserve safety & liveness.

---

### 🕒 Voting Window Management System

* Configurable voting windows: short (5m), medium (30m), long (2h), or custom.
* Window extensions if the threshold is nearly reached as time expires.
* Automatic cleanup of incomplete or expired proposals.
* Supports overlapping voting windows for concurrent proposals with varying urgency.
* Grace periods to tolerate network latency and clock drift.

---

### ⚖️ Weight Calculation Engine

* Precision arithmetic to prevent rounding errors affecting consensus.
* Weight caching for static votes to reduce computation load.
* Vote weight history tracking for audits and debugging.
* Batch updates when many votes arrive at once.
* Optional validator reputation bonuses for trusted validators.

---

### 🔗 Threshold Progression Framework

* Configurable progression profiles:

  * Conservative (slow increase)
  * Aggressive (fast increase)
  * Adaptive (based on participation)
* Scheduled thresholds — stricter at specific times of day or participation levels.
* Multi-dimensional thresholds: e.g., require both a % consensus *and* minimum vote count.
* Proposal-type specific thresholds — critical decisions demand higher consensus.
* Historical analytics to optimize progression parameters based on past data.

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/mohit-launch/chronovote.git
cd chronovote
```

Install dependencies:

```bash
cargo build
```

---

## 🛠️ Usage

Start the voting engine:

```bash
cargo run
```

---

## ⚡ Testing

Run unit tests and verification proofs:

```bash
cargo test
```

---


## 🤝 Contributing

Contributions are welcome!
Please fork the repo, create a feature branch, and open a pull request.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

## 👥 Author

* [Mohit Kumar Satpathy](https://github.com/Mohit-launch)
- blockchain
- Rust
