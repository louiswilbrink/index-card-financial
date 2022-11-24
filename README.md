# Index Card Financial


Index Card Financial is a personal finance app that provides a monthly snapshot of your finances.



It aggregates banking transactions across multiple accounts and allows for real-time catagorization and receipt attachment through slack for tax accounting purposes.



Built with Rust.



## Pages



`/`



One giant spreadsheet of all transaction across all accounts.  Includes saved annotations and attached receipts.



## API



#### `POST /transaction`



`body`



| Key  | Value |
| ---- | ----- |
|      |       |



Webhook endpoint for banking or credit card accounts to push new, raw transactions to the end user for categorization, receipt attachment, or general scrutiny.



#### `POST /update-transaction`



Webhook endpoint for end users to update a transaction with categorizations or attach receipts.



| Key        | Value           |
| ---------- | --------------- |
| `catagory` | `Category` enum |
| `receipt`  | `URL`           |



## Models



`category: enum`



Personal:

* Grocery
* Travel
* Eating Out

Business

* Marketing
* Servers
* Transportation
* Equipment
