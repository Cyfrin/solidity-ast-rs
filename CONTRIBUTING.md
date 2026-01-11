Every few months, we need to do the following to stay upto date with foundry.

Checkout a new branch and do upgrade and do the following:

1. See the latest tag (vX.Y.Z) from [Foundry Compilers](https://github.com/foundry-rs/compilers/tags). Set the value in [Cargo.toml](https://github.com/Cyfrin/solidity-ast-rs/blob/main/Cargo.toml) to refelect that version.

See here:

<img width="878" height="692" alt="Screenshot 2026-01-10 at 12 58 21 PM" src="https://github.com/user-attachments/assets/9c1a90fc-d9d3-49f9-80b4-ae07d8e2cdcd" />

Set the value here:

<img width="1308" height="677" alt="Screenshot 2026-01-11 at 1 14 42 PM" src="https://github.com/user-attachments/assets/b4304db2-8e1a-4c9d-a3e3-e7291139e696" />


2. Now, make a git commit. (Acts like a checkpoint)

3. See the latest nightly of foundry from [Foundry](https://github.com/foundry-rs/foundry/releases) releases. Copy the tag by clicking on the nightly as shown below as shown 

<img width="1260" height="611" alt="Screenshot 2026-01-10 at 12 57 21 PM" src="https://github.com/user-attachments/assets/dc2ef1a0-f71e-428a-b613-b3fb969f6ebc" />

4. Once you have copied the nightly tag `nightly-XXXXX`, adjust the following values in the uprade branch:

    A. Now adjust the values of Cagro.toml with appended values

    <img width="1298" height="718" alt="Screenshot 2026-01-10 at 1 01 11 PM" src="https://github.com/user-attachments/assets/06cfbf62-e977-46a7-8ee4-fd6e1a69cdc9" />

    
    B. Now adjust .cargo/config.toml
 
    <img width="1298" height="583" alt="Screenshot 2026-01-10 at 1 02 38 PM" src="https://github.com/user-attachments/assets/72e0e527-6f37-470a-879c-2bb90bbe708f" />

5. Now make another git commit. (Acts like a checkpoint)

6. Run `cargo vendor foundry-rs-config >> .cargo/config.toml` 

7. Discard any changes it makes to `.cargo/config.toml` file.
   
8. Now make another git commit. (Acts like a checkpoint)

9. At this point, you should run `cargo build`. Resolve any errors it reports such as incompatible version of `rayon` etc. if any by changing `Cargo.toml`.  Also if foundry-compilers has breaking changes,
   then some of the library interactions in [src](https://github.com/Cyfrin/solidity-ast-rs/tree/main/src) would be broken. Relearn the API, ask Dani Popes or Matthew S (foundry maintainer) on Telegram
   if help is needed. Make sure `cargo test` passes successfully. Keep iterating until it does.

10. Make a Git commit and Push the changes to remote branch. Make sure integration tests for solidity-ast-rs pass in CI.

11. Now, before merging to main, you need to test if these remote changes in solidity-ast-rs are compatible with downstream Aderyn. To do that checkout a new branch locally in Aderyn and uncomment the highlited line below in [Aderyn's Cargo.toml](https://github.com/Cyfrin/aderyn/blob/dev/Cargo.toml) and replace the value of branch i.e `feat/replace-me` with the name of the branch you pushed to in the above step.

Also comment out the line above the highlited line.

<img width="1296" height="581" alt="Screenshot 2026-01-10 at 1 10 55 PM" src="https://github.com/user-attachments/assets/df9beb55-64c4-4abf-bc17-dd942c9bef1a" />

12. Run `cargo test` inside Aderyn's repository. You may need to adjust some dependency versions of Aderyn. (We must give into whatever versions foundry choses and adjust our side of the code)

Also push Aderyn's testing branch to remote so CI can do integration tests (temporary branch). 

**Once they all pass, merge solidity-ast-rs upgrade branch to main. (because it is now safe to merge) **

13. After merging upgrade branch in solidity-ast-rs, cut a new release in solidity-ast-rs. Click in the place shown below and Draft a new release.

<img width="1256" height="689" alt="Screenshot 2026-01-10 at 1 17 25 PM" src="https://github.com/user-attachments/assets/b40bd89b-365f-4aed-af5f-18ff09abd726" />

Click on `Select Tag` and type `v0.0.1-alpha.beta.X` where X is 1 more than the current latest release found [here](https://github.com/Cyfrin/solidity-ast-rs/releases) in the releases section.

Click on `Create Tag`

Then chose to auto generate title and release notes.

Select pre-release:

<img width="1032" height="186" alt="Screenshot 2026-01-10 at 1 21 11 PM" src="https://github.com/user-attachments/assets/ec4bae4f-4566-44d5-9514-86a0dcb1048c" />

foundry-compilers is not yet 1.0 therefore we will always be behind. (That's okay)

14. Once the upgraded release is made on soldiity-ast-rs, go back to to Aderyn's branch. This time, uncomment the first line (highlited below). And uncomment the line below it (which you commented above).

 Bump up the value of the `tag` to point to the latest soldiity-ast-rs version that was just released. 

<img width="1269" height="617" alt="Screenshot 2026-01-10 at 1 22 50 PM" src="https://github.com/user-attachments/assets/09126e1c-6fc1-4770-9408-f3aea5852bdf" />

15. Push the local aderyn changes to remote branch. Let the CI re-run integration tests. (We know for sure this time, that it will pass)

16. Merge Aderyn changes to `dev` branch (which acts like main branch)

17. Done!
