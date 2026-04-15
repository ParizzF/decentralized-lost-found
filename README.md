# Decentralized Lost & Found DApp

**Decentralized Lost & Found DApp** - Blockchain-Based Lost Item Tracking System

## 📌 Project Description

Decentralized Lost & Found DApp is a smart contract application built on the Stellar blockchain using Soroban SDK. It provides a transparent and tamper-proof system for reporting, claiming, and confirming lost items using blockchain technology.

Unlike traditional lost-and-found systems that rely on centralized databases, this application stores all data on-chain, ensuring data integrity, transparency, and trustless interaction between users.

Users can report lost items, claim found items, and confirm item returns securely through smart contract functions.

---

## 🎯 Project Vision

Our vision is to improve lost-and-found systems by:

* **Decentralizing Data**: Eliminating reliance on centralized platforms
* **Ensuring Ownership**: Only item owners can confirm item returns
* **Preventing Manipulation**: Data stored on blockchain cannot be altered
* **Building Trustless Interaction**: No need for intermediaries
* **Increasing Transparency**: All actions are recorded on-chain

We aim to create a reliable and fair system for managing lost items in real-world environments such as campuses, offices, and public places.

---

## 🚀 Key Features

### 1. Report Lost Items

* Users can report lost items with name and description
* Each item is stored permanently on the blockchain
* Unique ID is automatically generated

### 2. View All Items

* Retrieve all lost items in one function call
* Real-time blockchain data
* Structured format for easy integration

### 3. Claim Found Items

* Other users can claim items they have found
* Prevents multiple claims using status validation
* Tracks finder address

### 4. Confirm Item Return

* Only the original owner can confirm item return
* Ensures ownership validation
* Updates item status securely

### 5. Status Tracking System

* `0 = Lost`
* `1 = Claimed`
* `2 = Returned`

---

## 🔗 Smart Contract Details

* Platform: Stellar (Soroban)
* Language: Rust
* Contract ID:

```
CDGEEV7SJDOFAWOZJNUUE25CW4LPGLIUGT5YYMS45GFIYJSXVVFEPAOO
```

---

## 🖼️ Screenshots

(Add screenshots from Soroban Studio)

Recommended screenshots:
<img width="1892" height="749" alt="image" src="https://github.com/user-attachments/assets/577b040f-f0bc-4453-96d2-4f8a1b827be9" />

* Contract deployment
* Report lost item
* Claim item
* Confirm return
* Get items

---

## ⚙️ How It Works

1. A user reports a lost item using `report_lost`
2. Another user claims the item using `claim_item`
3. The original owner confirms the return using `confirm_return`

All actions are recorded on the blockchain and cannot be modified.

---

## 🛠️ Functions

* `report_lost(user, name, description)`
* `get_items()`
* `claim_item(user, id)`
* `confirm_return(user, id)`

---

## 🔮 Future Scope

### Short-Term

* Add item location tracking
* Add timestamp for each transaction
* Improve UI integration

### Medium-Term

* Reward system for finders
* Image/hash attachment for items
* Notification system

### Long-Term

* Cross-platform integration
* Decentralized frontend (IPFS)
* Mobile application
* Integration with identity systems

---

## 🧠 Notes

This project demonstrates a real-world blockchain use case beyond cryptocurrency by implementing a transparent and trustless lost-and-found system.

---

## 👤 Author

Ahmad Faris
