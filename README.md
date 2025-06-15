# 🔐 Simple Crypto Wallet CLI

A beginner-friendly command-line crypto wallet written in Rust — fully offline, no blockchain dependencies! Perfect for experimenting with keys, digital signatures, and basic transaction modeling.

---

## 🧠 What It Does

This project:

- ✅ Generates ECDSA public/private key pairs using P-256 and SHA-256
- ✍️ Digitally signs a message (i.e., your transaction)
- 🧾 Collects transaction data from the user via CLI
- 📦 Stores the signed transaction in a local JSON file
- 📂 Allows for basic transaction recording and testing logic — no networking, no node syncing

---

## 📦 Requirements

- Rust (latest stable)
- `serde`, `serde_json` (for JSON serialization)
- `chrono` (for timestamps)
- `ring` (for cryptography)

You can install dependencies with:

```bash
cargo add serde serde_json chrono ring
```

#✅ How to Run
```
cargo run
```


# Built with ❤ by Kiyarash
