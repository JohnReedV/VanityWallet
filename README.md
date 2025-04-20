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
┌────────┬──────────────┬──────────────────┐
│ Prefix │ Combinations │ Time @ 1.49 M/sec │
├────────┼──────────────┼──────────────────┤
│ 1      │ 58           │ 0.00004 s        │
│ 2      │ 3.36 K       │ 0.00226 s        │
│ 3      │ 195.11 K     │ 0.13094 s        │
│ 4      │ 11.32 M      │ 7.59 s           │
│ 5      │ 656.36 M     │ 7.34 m           │
│ 6      │ 38.07 B      │ 7.10 h           │
│ 7      │ 2.21 T       │ 17.15 d          │
│ 8      │ 128.06 T     │ 2.73 y           │
│ 9      │ 7.43 P       │ 158.06 y         │
│ 10     │ 430.80 P     │ 9.17 k y         │
└────────┴──────────────┴──────────────────┘
```
