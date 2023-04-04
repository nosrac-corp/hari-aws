use aws_sdk_ec2::{Client, Error};

/// Gets list of all available AWS regions as a list of Strings
pub async fn get_region_list(client: &Client) -> Result<Vec<String>, Error> {
    let rsp = client.describe_regions().send().await?;

    let mut region_list: Vec<String> = vec![];

    for region in rsp.regions().unwrap_or_default() {
        region_list.push(region.region_name().unwrap().to_string());
    }

    Ok(region_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_config::meta::region::RegionProviderChain;
    use aws_sdk_ec2::{config::Region, Client};

    #[tokio::test]
    async fn test_region_list() {
        let region: Option<String> = None;

        let region_provider = RegionProviderChain::first_try(region.map(Region::new))
            .or_default_provider()
            .or_else(Region::new("us-west-2"));

        let shared_config = aws_config::from_env().region(region_provider).load().await;

        let client = Client::new(&shared_config);

        for r in get_region_list(&client).await.unwrap() {
            println!("{:?}", r);
        }
    }
}
