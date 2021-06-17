use structopt::StructOpt;
use exitfailure::{ExitFailure};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};


#[derive(StructOpt)]
struct Cli {
    public_address: String
}

#[derive(Serialize, Deserialize, Debug)]
struct WalletQuery {
    data: Data
}
#[derive(Serialize, Deserialize, Debug)]
struct Data {
    address: String,
    updated_at: String,
    quote_currency: String,
    items: Vec<Items>
}
#[derive(Serialize, Deserialize, Debug)]
struct Items {
    contract_name: String,
    balance: String,
}


impl WalletQuery {
    async fn get(public_address: &String) -> Result<Self, ExitFailure> {
        // insert your api key here
        let api_key = "";
        let url = format!("https://api.covalenthq.com/v1/1/address/{}/balances_v2/?&key={}", public_address, api_key);
        let url = Url::parse(&*url)?;
        let resp = reqwest::get(url)
            .await?
            .json::<WalletQuery>()
            .await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure>{
    let args = Cli::from_args();
    let response : WalletQuery = WalletQuery::get(&args.public_address).await?;
    println!("this wallet: {} \n", args.public_address);
    println!("have this address: {} \n", response.data.address);
    println!("last update: {} \n", response.data.updated_at);
    println!("coins: \n");
    println!("name: {}", response.data.items[0].contract_name);
    println!("balance: {} \n", response.data.items[0].balance);
    println!("name: {}", response.data.items[1].contract_name);
    println!("balance: {} \n", response.data.items[1].balance);
    println!("name: {}", response.data.items[2].contract_name);
    println!("balance: {} \n", response.data.items[2].balance);
    Ok(())
}