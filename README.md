# neo-rs
A neo wallet project written in Rust Language.

## 1. Function

### Implemented
- Private Key
- Public key
- Address
- Wif Key
- nep2

### TODO
- Nep6
- Import/export account
- SGX Protection
- Network
- Transaction
- Contract


## 2 Install Rust

We recommend installing Rust using [rustup](https://www.rustup.rs/). You can install `rustup` as follows:

- macOS or Linux:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows (64-bit):  
  
  Download the [Windows 64-bit executable](https://win.rustup.rs/x86_64) and follow the on-screen instructions.

- Windows (32-bit):  
  
  Download the [Windows 32-bit executable](https://win.rustup.rs/i686) and follow the on-screen instructions.


## 3 Build & Run

Down load neo-rs from the github with command:

```bash
git clone https://github.com/Liaojinghui/neo-rs.git
```

In the terminal, move into the  `neo-rs` folder and run command  `cargo build`:

```bash
cd neo-rs && cargo build
```

To rnu the project, run command:

```bash
./target/build/neo neo
```

```bash
Public Key:      0x03f9e9a50af13ccec64feedb45d558815ba6d3a3e8c3a727be7f97bb9eeca80f52
Private Key:     0x1d9d6b11b9570e50a8511de539be9d125dda022b7d65452acc03de3aa3e87d6c
WIF:             KxDH6p2nsKiYo5rvk8pvDSS2dPC2UBLoYeQu2Mq9ZnVPm1YfauSh
Address:         AHV5J1bVXAvM3eVDrCXx34U1QQnNKeKX1F
```
