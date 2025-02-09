use azure_storage::prelude::StorageCredentials;
use azure_storage_datalake::prelude::*;
use futures::stream::StreamExt;
use std::num::NonZeroU32;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let data_lake_client = create_data_lake_client();

    let file_system_name = format!(
        "azurerustsdk-datalake-example00-{}",
        OffsetDateTime::now_utc().unix_timestamp()
    );
    let file_system_client = data_lake_client.file_system_client(file_system_name.to_string());

    let mut fs_properties = Properties::new();
    fs_properties.insert("AddedVia", "Azure SDK for Rust");

    println!("creating file system '{}'...", &file_system_name);
    let create_fs_response = file_system_client
        .create()
        .properties(fs_properties.clone())
        .await?;
    println!("create file system response == {:?}\n", create_fs_response);

    println!("listing file systems...");
    let mut stream = data_lake_client
        .list_file_systems()
        .max_results(NonZeroU32::new(3).unwrap())
        .into_stream();
    while let Some(list_fs_response) = stream.next().await {
        println!("list file system response == {:?}\n", list_fs_response);
    }

    println!("getting file system properties...");
    let get_fs_props_response = file_system_client.get_properties().await?;
    println!(
        "get file system properties response == {:?}\n",
        get_fs_props_response
    );

    println!("setting file system properties...");
    fs_properties.insert("ModifiedBy", "Iota");
    let set_fs_props_response = file_system_client.set_properties(fs_properties).await?;
    println!(
        "set file system properties response == {:?}\n",
        set_fs_props_response
    );

    println!("getting file system properties...");
    let get_fs_props_response = file_system_client.get_properties().await?;
    println!(
        "get file system properties response == {:?}\n",
        get_fs_props_response
    );

    println!("deleting file system...");
    let delete_fs_response = file_system_client.delete().await?;
    println!("delete file system response == {:?}\n", delete_fs_response);

    Ok(())
}

fn create_data_lake_client() -> DataLakeClient {
    let account_name = std::env::var("ADLSGEN2_STORAGE_ACCOUNT")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCOUNT first!");
    let account_key = std::env::var("ADLSGEN2_STORAGE_ACCESS_KEY")
        .expect("Set env variable ADLSGEN2_STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::Key(account_name.clone(), account_key);
    DataLakeClient::new(account_name, storage_credentials)
}
