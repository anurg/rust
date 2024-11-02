### Anchor Framework for Programming on Solana

### What is Anchor?
Anchor is a framework used to write safe, secure, and high-level programs on Solana, which abstracts the low-level construction of accounts and modification of the interfaces to your Solana programs.

A framework acts as a foundation so that the users don't have to create unnecessary logic from scratch, helps users avoid redundant code, and help them write clean and secure programs. Specifically, Anchor provides a deserialized accounts and instruction data through boilerplates, a command line interface (CLI), and a workspace for developing Solana dapps.

### How does Anchor work?
Anchor uses macros and traits to generate boilerplate Rust code for developer. Every Anchor program consists of three components:

- declare_id -a macro used for declaring the program’s on-chain address

- #[program] - attribute macro used to denote the module containing the program’s instruction logic

- #[account] - attribute macro used to define custom account types for the program

Here is the basic structure of an Anchor program:
![BAsic_Anchor_program](images/1703690818-basic-structure-of-an-anchor-program.avif)

### 1. Create a Default Keypair
When we build an Anchor program for the first time, it generates a new key pair which serves as a default keypair to deploy the program.

```
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS")
```

The public key should be used as the programID specified in the declare_id! Marco.

### 2. Instruct the Program
Then, we can separately instruct the program using the #[program] attribute. Each instruction function requires a parameter of type Context and can optionally include additional function parameters representing instruction data. Anchor will automatically handle instruction data deserialization so that the user can work with instruction data as Rust types.
```
#[program]mod 
hello_anchor {    
    use super::*;    
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {        
        Ok(())   
 }}

```
### 3. Implement an Account Deserializer
#[derive(Accounts)] implements an Account deserializer on the given struct and removes the need to deserialize each account manually. This is responsible for performing all requisite constraint checks and ensuring the accounts meet the conditions required for the program to run securely.
```
#[derive(Accounts)]
pub struct Initialize {}
```
### 4. Invoke the Initialize Function
When the Initialize function is invoked, the program:

Checks that the accounts passed into the instruction match the account types specified in the Initialize struct

Checks the accounts against any additional constraints specified

This is how an Anchor program works and allows you to get started with building Anchor programs. To know more about Anchor programs, it is highly recommended to check out their documentation.
