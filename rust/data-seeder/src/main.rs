use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{
    error::DisplayErrorContext,
    types::{error::TableNotFoundException, AttributeValue},
    Client,
};
use shared::models::model::CacheableItem;

async fn load_data(client: &Client) {
    let mut children = vec![];

    for c in 0..100 {
        let clone_client = client.clone();
        let handle = tokio::spawn(async move {
            for j in 0..1000 {
                let i = CacheableItem::default();

                match clone_client
                    .put_item()
                    .item("id".to_string(), AttributeValue::S(i.id_as_str()))
                    .item(
                        "first_name".to_string(),
                        AttributeValue::S(i.first_name.clone()),
                    )
                    .item(
                        "last_name".to_string(),
                        AttributeValue::S(i.last_name.clone()),
                    )
                    .item(
                        "updated_at".to_string(),
                        AttributeValue::N(i.updated_at.timestamp().to_string()),
                    )
                    .item(
                        "created_at".to_string(),
                        AttributeValue::N(i.created_at.timestamp().to_string()),
                    )
                    .table_name("CacheableTable".to_string())
                    .send()
                    .await
                {
                    Ok(_) => {
                        println!("(Item)={:?}", i);
                    }
                    Err(e) => {
                        println!("There was an unhandled error: {}", DisplayErrorContext(&e));
                    }
                }
            }
        });
        children.push(handle);
    }

    for t in children {
        t.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let client = new_client().await;
    load_data(&client).await
}

pub async fn new_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let sdk_config = aws_config::from_env().region(region_provider).load().await;

    let config = aws_sdk_dynamodb::config::Builder::from(&sdk_config).build();
    Client::from_conf(config)
}
