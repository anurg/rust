### Ackee Solana Developer Bootcamp Notes
### Overview
- An app interacts with Solana Cluster (Devnet, Testnet, Mainnet-Beta) by sending it transaction with one or more instructions
- The Solana runtime passes these instructions to programs which has been deployed by app Developers beforehand.
- Instructions are executed sequentially and atomiccally for each transaction by the program.
- If any instructions is invalid, all account changes in the transaction are discarded.


### What are Solana Program?
- It is  a piece of code that is deployed (included) to and runs on Node (Validator).
- Similar to regular Rust binaries
    - main() -> entrypoint()
- Programs are stateless. It means you cannot store Data in them.
- We use "Accounts" to store both Program's Code and Data.
    - We can imagine Accounts as "Files". Each Account has unique adddress. 

### What are Solana Account?
- In Solana , everything (all Data-State & Program Code) is stored within Accounts.
- Think of it as key-value pair (Like HashMap in Rust)
- Each Account has Key, Value Paid where key is Account Address (Public Key) and Value is Account Info. 
- This database is stored in the Validator.
- Address of Account - 32 Bytes Long Public Key
- Account Info- (5 Elements- lamports, owner, rent_epoch, executable Flag, Data)
    - Data - A Byte array that stores Data.If the account is a program, this stores executable code.
    - Executable- A boolean flag that indicates if account is a Program.
    - lamports - Accounts balance in lamports
    - Owner - Specifies the public key(Program ID) of the program that owns the account.
- Accounts can store upto 10MB of Data (Data or Program)
- Accounts require rent deposit in sol, proportional to the amount of data stored, which is fully refundable when account is closed.
- If you pay more then 2 years rent, then Account becomes rent exempt.That means we need to pay onnly once.
- Every account has a program "Owner". Only the program that owns the Account can modify its data or deduct its lamport balance. However anyone can increase the balance.
- Account Owner vs Account holder - Account holder has the private key of the Account.
- Programs are stateless that store the executable code.
- Data Accounts are created by Programs to store and manage the Program State.
- Native Programs are buildin programs which are part of the Validator Implementation.
    - System Program (e.g. Performing Sol Transfers)
    - BPF Loader - owner of all other program on the network
    - SYSVAR Accounts are special account that store the network cluster state.
        - Clock Account contains cluster time.
![Solana_Accounts](images/ackee_solana1.png)

## Token Program
### Fungible and Non-Fungible Tokens
- Fungible Tokens: These tokens are interchangeable with one another. They are indistinguishable and hold the same value.

- Non-Fungible Tokens (NFTs): Each NFT is a special digital asset that holds a unique information or value. NFTs can represent ownership of a specific digital or physical item, such as digital art or real estate.

### Token Program
- Token program defines a common implementation for fungible and non-fungible tokens. All tokens on Solana are created using the SPL Token program.

- Solana's SPL tokens are similar to Ethereum's ERC-20 or ERC-721 standards. However, Solana does not require you to deploy a new contract for each token you create. Instead, you simply send instructions to the Token program, which will create and mint tokens on your behalf.

![Solana_Token_program](images/solana3.png)

### Creating Tokens
A token can be created by initializing a new mint account. The mint account is used to create and hold info about new tokens, which are then stored in token accounts. Once a mint account is initialized, the mint_authority can create (mint) new tokens using the MintTo instruction.

- As long as mint account contains a valid mint_authority, it is considered to have a non-fixed supply, and the mint_authority can create new tokens with the MintTo instruction at any time.
Note- Contract is not renounced as they say in ethereum. By setting the mint authority to some NULL address, we can freeze the supply and make the token attribute unchangeable.

### Transferring Tokens
Balances can be transferred between accounts using the Transfer instruction, with the source account owner required as a signer when the accounts differ.

### Burning Tokens
The Burn instruction decreases an account's token balance without transferring to another account. The burnt tokens are permanently removed from the circulation, and this action can be verified on chain.

### Freezing Accounts
The mint account may include a freeze_authority, allowing it to invoke FreezeAccount instruction that will make account unusable. Frozen accounts can be reactivated using the ThawAccount instruction and freeze_authority can be changed using the SetAuthority instruction.

### Wrapping SOL
SOL can be wrapped and used like a Token program token when interacting with programs that use the Token program's interface. Accounts that hold wrapped SOL are associated with Native Mint. Wrapped SOL token accounts have unique behaviors, which you can learn more about here.

### Associated Token Account
- An Associated Token Account (ATA) is a token account, but its address has a special property.
- An Associated Token Account address is derived (as a PDA) from its owner's wallet address and the address of the mint.
- This means that user has a different ATA for every wallet and token mint combination. This is very convenient because programs can easily find and interact with the correct account without needing the user to provide the address.
- A user can receive tokens even if they do not yet have a token account for that mint. The sender is able to fund the creation of the receiver's ATA, enabling things like airdrop campaigns.
- Associated Token Account program facilitates the creation and management of ATAs.

### What is a Transaction?
- Transactions are sent to interact with the Solana Network.
  - User/App sends transaction to the Solana validator to interact with programs deployed in Validator.
  - Transactions include one or more instructions.
    - The execution logic for instructions is stored in the Programs deployed on Validator.
  - Execution Order of Instructions
    - Sequentially
  - Atomicity of Transactions
![Solana_Transaction](images/solana2.png)

### Transaction Lifecycle
The following steps can be thought of as an overview of what happens when an app or any user interacts with the Solana network by sending a transaction:

- The user or app submits a transaction with one or more instructions to a node that accepts RPC requests.
- The transaction is then forwarded, according to the leader schedule, to the next leader.
- The leader validates the transaction, processes it, and includes it in a new block. This block is then broadcasted to all other validators who also validate and process the transaction.

During the transaction processing, instructions are executed by the previously deployed programs, and relevant accounts are modified accordingly.

Everything happens in an isolated virtual machine. Instructions are executed sequentially and atomically, which means that either all instructions in the transaction complete successfully or none of the changes are applied.
### Typescript Code for transferring Sol

```
import {
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";

// Use Playground cluster connection
const connection = pg.connection;

// Use Playground wallet as sender, generate random keypair as receiver
const sender = pg.wallet.keypair;
const receiver = new Keypair();

// Check and log balance before transfer
const preBalance1 = await connection.getBalance(sender.publicKey);
const preBalance2 = await connection.getBalance(receiver.publicKey);

console.log("sender prebalance:", preBalance1 / LAMPORTS_PER_SOL);
console.log("receiver prebalance:", preBalance2 / LAMPORTS_PER_SOL);
console.log("\n");

// Define the amount to transfer
const transferAmount = 0.01; // 0.01 SOL

// Create a transfer instruction for transferring SOL from wallet_1 to wallet_2
const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver.publicKey,
  lamports: transferAmount * LAMPORTS_PER_SOL, // Convert transferAmount to lamports
});

// Add the transfer instruction to a new transaction
const transaction = new Transaction().add(transferInstruction);

// Send the transaction to the network
const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [sender] // signer
);

// Check and log balance after transfer
const postBalance1 = await connection.getBalance(sender.publicKey);
const postBalance2 = await connection.getBalance(receiver.publicKey);

console.log("sender postbalance:", postBalance1 / LAMPORTS_PER_SOL);
console.log("receiver postbalance:", postBalance2 / LAMPORTS_PER_SOL);
console.log("\n");

console.log(
  "Transaction Signature:",
  `https://explorer.solana.com/tx/${transactionSignature}?cluster=devnet`
);

```

### Anchor Program
An Anchor program consists of three parts. The program module, the Accounts structs 
which are marked with #[derive(Accounts)], and the declare_id macro. The program module 
is where you write your business logic. The Accounts structs is where you validate 
accounts. Thedeclare_id macro creates an ID field that stores the address of your 
program. Anchor uses this hardcoded ID for security checks and it also allows 
other crates to access your program's address.

When you start up a new Anchor project, you'll see the following:
```
use anchor_lang::prelude::*;


// declare an id for your program
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


// write your business logic here
#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}


// validate incoming accounts here
#[derive(Accounts)]
pub struct Initialize {}
```

note that the way an endpoint is connected to its corresponding Accounts struct is 
the ctx argument in the endpoint. The argument is of type Context which is generic 
over an Accounts struct, i.e. this is where you put the name of your account 
validation struct. In this example, it's Initialize.

### The Accounts Struct
The Accounts struct is where you define which accounts your instruction expects 
and which constraints these accounts should adhere to. You do this via two 
constructs: Types and constraints.


### Cross Program Invocation
A Cross Program Invocation (CPI) is a mechanism that allows Solana programs to call other programs from within an instruction. The caller is halted until execution returns from the callee.

Programs can extend their signer privileges to other programs using CPIs. CPIs are executed by calling either invoke or invoke_signed methods within their instructions:

invoke - All necessary signatures are already available before the call.
invoke_signed - Calling program needs PDAs to act as signers during the CPI.

- After a CPI to another program is made, the callee program can make further CPIs to other programs. The maximum CPI depth is 4.
- CPIs are a very powerful feature as they allow developers to make use of other deployed programs and continuously build on and expand the Solana's already existing ecosystem.

### Solana Program Library
The Solana Program Library (SPL) is a collection of on-chain programs. These programs facilitate tasks like creation and management of tokens and more.

