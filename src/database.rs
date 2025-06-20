use std::{
    io::{self, Cursor},
    process,
};

use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{error, warn};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Item {
    pub name: String,
    pub count: usize,
}

impl Item {
    fn new(name: String, count: usize) -> Self {
        Self { name, count }
    }
}

pub struct Database(File);

impl Database {
    pub async fn new() -> Self {
        let file: File = match OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open("database.bson")
            .await
        {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open database file: {e}");

                process::exit(1)
            }
        };

        Self(file)
    }

    pub async fn add_item(&mut self, name: String, count: usize) {
        let item = Item::new(name, count);

        let bson_bytes: Vec<u8> = match bson::to_vec(&item) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to serialize item: {e}");

                process::exit(1)
            }
        };

        if let Err(e) = self.0.write_all(&bson_bytes).await {
            error!("Failed to write item to database file: {e}");

            process::exit(1);
        }
    }

    pub async fn get_all_items(&mut self) -> Result<Vec<Item>, io::Error> {
        let mut buffer: Vec<u8> = Vec::new();

        match self.0.read_to_end(&mut buffer).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to read items from the database file: {e}");

                return Err(e);
            }
        }

        let mut items: Vec<Item> = Vec::new();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(buffer);

        while let Ok(doc) = bson::from_reader(&mut cursor) {
            match bson::from_document(doc) {
                Ok(item) => items.push(item),
                Err(e) => {
                    warn!("Failed to deserialize an item, skipping: {e}");
                }
            }
        }

        Ok(items)
    }

    pub async fn get_item_by_name(&mut self, name: String) -> Result<Option<Item>, io::Error> {
        let items: Vec<Item> = self.get_all_items().await?;

        Ok(items.into_iter().find(|item| item.name == name))
    }

    pub async fn update_item_by_name(
        &mut self,
        name: &str,
        count_change: usize,
        operation: &str,
    ) -> Result<Item, Box<dyn std::error::Error>> {
        let mut items: Vec<Item> = self.get_all_items().await?;
        let mut item_f: Option<Item> = None;

        for item in &mut items {
            if item.name == name {
                match operation {
                    "+" => {
                        item.count += count_change;
                    }
                    "-" => {
                        item.count -= count_change;
                    }
                    _ => {
                        return Err("Invalid operation, must be '+' or '-'".into());
                    }
                }
                item_f = Some(item.clone());
                break;
            }
        }

        if let Some(found_item) = item_f {
            Self::write_all_items_to_file(&items).await?;
            Ok(found_item)
        } else {
            warn!("Attempted to update item '{name}', but it was not found.");
            Err("Item not found".into())
        }
    }

    pub async fn delete_item_by_name(
        &mut self,
        name: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut items: Vec<Item> = self.get_all_items().await?;
        let original_len: usize = items.len();

        items.retain(|item: &Item| item.name != name);

        if items.len() < original_len {
            Self::write_all_items_to_file(&items).await?;
            Ok(())
        } else {
            warn!("Attempted to delete item '{name}', but it was not found.");
            Err("Item not found".into())
        }
    }

    async fn write_all_items_to_file(items: &[Item]) -> Result<(), io::Error> {
        let temp_path: &'static str = "database.bson.tmp";
        let mut temp_file: File = File::create(temp_path).await?;

        for item in items {
            let bson_bytes: Vec<u8> = bson::to_vec(item).map_err(io::Error::other)?;
            temp_file.write_all(&bson_bytes).await?;
        }

        temp_file.sync_all().await?;

        fs::rename(temp_path, "database.bson").await?;
        Ok(())
    }
}
