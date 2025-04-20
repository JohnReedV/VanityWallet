# 🏹 Vanity Wallet: SS58 Prefix Hunter

> **Dream** in prefixes—hunt your perfect Substrate wallet **faster than ever**.

---
## 🚀 Usage
```bash
./target/release/vanity_wallet <YOUR_PREFIX>
```

Example:
```bash
cargo build --release
```
```bash
./target/release/vanity_wallet ART
```

> **Output**  
> ```
> Tried     1,234,567 addresses | elapsed 00:00:05 | 246,913 addr/sec
> 🎉 Found it in 00:00:07 after 1,750,000 tries!
> Address:  5eARTxyz123abcdef...
> Mnemonic: excess grace odor ...
> ```

## 📊 Benchmarks
**I914900Ks:**
```m
┌────────┬──────────────┬─────────────────────┐
│ Prefix │ Combinations │ Time @ 32,038/sec   │
├────────┼──────────────┼─────────────────────┤
│ 1      │ 58           │ 0.00181 s           │
│ 2      │ 3.36 K       │ 0.10500 s           │
│ 3      │ 195.11 K     │ 6.09 s              │
│ 4      │ 11.32 M      │ 5 m 53 s            │
│ 5      │ 656.36 M     │ 5 h 41 m            │
│ 6      │ 38.07 B      │ 13 d 18 h           │
│ 7      │ 2.21 T       │ 2.19 y              │
│ 8      │ 128.06 T     │ 126.75 y            │
│ 9      │ 7.43 P       │ 7.35 k y            │
│ 10     │ 430.80 P     │ 426.39 k y          │
└────────┴──────────────┴─────────────────────┘
```
