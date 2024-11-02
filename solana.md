### Solana/Anchor Notes

### Solana Account Model
Everything on Solana can be considered an account, and accounts are the way the protocol “organizes” the data on the blockchain.

###  What is a Solana account and how does it work?
Solana accounts can be thought of as storage buckets, capable of holding every data type: from tokens such as SOL to a program’s state variables (e.g. integers, strings, public keys), and even entire programs themselves. Every account has a specified owner, and a single owner can own many different accounts.

The Account Model is often compared to a computer file system:
![Solana_AccounT_Model](images/1703697235-computer-file-and-solana-account.avif)

### What are Solana account types?
Solana account types are of two main types of accounts: executable and non-executable. This is because Solana programs do not store state like Ethereum smart contracts do, so it needs to separate both type of actions.

### What are executable programs?
Executable programs are comprised of immutable code that own and create other accounts that store the state. That code is written in a language like Rust and then compiled to eBPF, which is a form of bytecode.

The most common example of executable programs is the core System Program of Solana, responsible for the creation of the accounts (wallets) that people use to access their tokens and NFTs.

### What are non-executable programs?
Non-executable programs are “storage” accounts which contain all other types of data like program variables, token balances, NFTs, fungible currencies, etc. Through non-executable accounts, the protocol reflects the state changes that have occurred after every transaction.

### Ethereum Smart Contracts vs. Solana Programs
A smart contract comes prebuilt with a special compartment called “storage” where its state variables are stored. Smart contracts then uses its executable code to modify those state variables. An Ethereum contract can be seen as a unified place for executable and non-executable code to co-exist.

Let’s take the USDC token for example. Its contract contains a mapping called “balances” which maps a user’s address to his own USDC amount. Upon sending or receiving USDC tokens, the contract reflects the changes by itself thanks to its executable code.

Solana handles this in a different manner. There is a USDC token program that creates a new account for each individual token holder. These accounts store the holder’s available USDC amount. When a user propagates a transaction, the USDC Token account looks for the associated token accounts of the users that want to make the exchange and then reflect the change in them.
![Solana_ATA](images/1703697343-solana-associated-token-account-structure.avif)

## What are Solana account categories?
Executable and non-executable accounts can be divided into several additional categories such as native programs, program accounts, and data accounts.

### What are executable native programs?
Executable native programs are “native” to the Solana ecosystem and are responsible for tasks like maintaining and running the validator nodes. The most well-known example of this type is the System Program, which is in charge of creating new system accounts (what we commonly know as “wallets”), transfer of SOL, etc. Other instances of Native Programs are the Stake Program, which is responsible for the staking mechanism, and the BPF Loader, which is similar to Ethereum’s EVM.

### What are executable program accounts?
Executable program accounts are pre-made Solana programs that create and store other programs. A good example of these is the Solana Program Library (SPL) — a collection of programs that support a number of on-chain activities, including creating, swapping, and lending tokens, generating stake pools, and maintaining an on-chain name service. A prominent representative of the SPL is the Token Program, which creates and manages tokens.

Solana’s USDC token is just a Token account, managed by the Token Program. However, the Token account does not store the users’ token balances. This is handled by a member of the third type of accounts: data accounts.

### What are non-executable data accounts?
While the Native Programs and Program accounts are considered executables, data accounts are the opposite: they are a registry that reflects the state changes for specific programs and the Solana protocol as a whole.

Data accounts can be divided into:

- Token accounts - accounts created by the Token Program

- Associated Token Accounts (ATA) - accounts that store the token balances for each individual user

- System Owned Accounts - store data and allow for the signing of transactions

Token accounts represent each individual fungible currency (USDC, USDT, etc.), and contain basic information for the token like supply, number of decimals, name, etc. For each unique Token account a different associated token account (ATA) is generated. For instance, if you own both USDC and USDT, you have access to two ATAs that have been created by the USDC and the USDT Token accounts respectively.
![Solana_Account_Type](images/1703697445-solana-account-types-and-categories.avif)


### What are Solana account elements?
Each account element is a metadata set, so the protocol can easily understand the account’s type and additional information. Metadata consists of:

- lamports - the account’s SOL balance, denominated in lamports where 1 lamport is equivalent to one billionth of a SOL token

- owner - this is the address of the program that owns the account

- executable - a boolean variable that indicates if the account contains executable code

- data - the raw data byte array, stored in the account, be it storage variables or executable code

- rent_epoch - indicates the next epoch at which the account will owe rent

The difference between an owner and a holder is that the owner is the program that controls the account and a holder is the user who holds the private key to the account. For example, the System Owned Account is owned by Solana’s Native System Program.

### What is Solana account rent?
Every account pays a rent fee to use memory on the blockchain, and is a preventative measure to keep attackers from clogging the network by using all of its memory.

For a blockchain to satisfy all its users, it must include some kind of economic incentive. This applies especially to the validators, who supply their hardware and computational power to verify all the transactions. Validators need to maintain a working copy of all the state changes and receive rent as a reward.

An account that maintains a minimum balance equivalent to 2 years of rent fees is exempt. If not, it is charged when it is referenced by a transaction or at every epoch, which currently equates to two days. If an account does not contain the minimum amount of SOL to be rent exempt, its data is deleted from the chain.

### How to Create a Solana Account
Creating a System Account on Solana is what Solana wallets do in the background each time a user creates a new “wallet”. A wallet creates a new account by generating a 64-byte keypair that lies on an ED25519 elliptic curve. The first 32 bytes are the private key, which is used when the user wants to create a transaction and interact with another account or program. The second set of 32 bytes is the public key of the wallet.


### What is an Solana Associated Token Account on Solana?




### PDA- Program Derived Addresses
https://www.alchemy.com/overviews/program-derived-address

### How to Calculate Rent on Solana
https://www.alchemy.com/overviews/how-to-calculate-rent-for-solana-programs