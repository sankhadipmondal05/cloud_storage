# ☁️ Cloud Storage dApp (Soroban on Stellar)

## 📌 Project Description
<img width="1918" height="900" alt="image" src="https://github.com/user-attachments/assets/b351df9f-0e80-416b-b600-1031a5b803d7" />

This project is a **decentralized cloud storage application** built using **Soroban smart contracts** on the **Stellar blockchain**, with a modern **React-based frontend**.

It enables users to securely store, retrieve, and manage their data in a **trustless and decentralized environment**, eliminating reliance on traditional centralized cloud providers.

The system combines **on-chain storage (Soroban)** with **off-chain storage (IPFS)** to create a scalable and efficient cloud storage solution.

---

## 🚀 What It Does

* Connects user wallets (via Freighter)
* Uploads files to decentralized storage (IPFS)
* Stores file references (hash/URL) on the Stellar blockchain
* Retrieves stored file data anytime
* Allows users to delete their stored records

---

## 🧠 How It Works

1. User uploads a file from the frontend
2. File is stored on **IPFS** → returns a unique URL/hash
3. The URL/hash is stored in the **Soroban smart contract**
4. Users can later fetch or delete their stored data

---

## ✨ Features

* 🔐 **Wallet Authentication** (Freighter)
* ☁️ **Decentralized Storage** (IPFS + Soroban)
* 🔑 **User-Owned Data** – Only the owner can modify their data
* 📦 **Key-Value Storage Model**
* 🗑️ **Delete Functionality**
* ⚡ **Fast & Lightweight Transactions**
* 🌐 **Fully Decentralized Architecture**

---

## 🧱 Tech Stack

### Blockchain

* Soroban (Smart Contracts)
* Stellar Network (Testnet)

### Frontend

* React (Vite)
* JavaScript

### Storage

* IPFS (via Infura or public gateway)

### Wallet

* Freighter Wallet

---

## 📜 Smart Contract

### 🔗 Deployed Contract

CAQXCXWRSG2F6F6NGX73ZP4WHM5QPPJM7Z6BQMP4Y73PSWC6CMRUA755

---

## 📂 Smart Contract Functions

### `upload(user, key, value)`

Stores a value (e.g., IPFS URL) under a specific key for a user.

### `get(user, key)`

Retrieves stored data associated with a key.

### `delete(user, key)`

Deletes stored data for a given key.

---

## 🖥️ Frontend Features

* Connect Wallet (Freighter)
* Upload File → Store on IPFS
* Save IPFS URL to blockchain
* Fetch stored files
* Delete stored entries

---

## ⚙️ Installation & Setup

### 1. Clone the Repository

```
git clone https://github.com/your-username/cloud-storage-dapp
cd cloud-storage-dapp
```

---

### 2. Install Dependencies

```
npm install
```

---

### 3. Run the App

```
npm run dev
```

---

### 4. Environment Variables

Create a `.env` file:

```
VITE_CONTRACT_ID=CAQXCXWRSG2F6F6NGX73ZP4WHM5QPPJM7Z6BQMP4Y73PSWC6CMRUA755
VITE_SOROBAN_RPC=https://soroban-testnet.stellar.org
```

---

## 🧪 Usage

1. Connect your Freighter wallet
2. Upload a file
3. File is stored on IPFS
4. IPFS URL is saved on-chain
5. Retrieve or delete your files anytime

---

## ⚠️ Limitations

* Large files are not stored on-chain (only references)
* No built-in encryption (should be handled client-side)
* Basic UI (can be extended to full cloud dashboard)

---

## 🔮 Future Improvements

* 🔐 End-to-end encryption
* 📂 File explorer UI (like Google Drive)
* 👥 File sharing (public/private links)
* 🧾 Metadata support (file name, size, type)
* 💰 Storage pricing model
* 📊 Dashboard analytics

---

## 🧑‍💻 Author

**Sankhadip Mondal**

---

## 📄 License

MIT License

---

## 💡 Vision

This project demonstrates how blockchain can be used to build **next-generation cloud storage systems** where:

* Users own their data
* No centralized authority controls access
* Storage is censorship-resistant
* Data is permanently verifiable

---

## ⭐ Support

If you like this project, consider giving it a ⭐ on GitHub!
