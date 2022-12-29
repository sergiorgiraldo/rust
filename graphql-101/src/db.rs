use std::error::Error;
use std::sync::{Arc, Mutex};
use csv;
use crate::models::*;

pub struct DataContext {
    pub clients: Arc<Mutex<Vec<ClientModel>>>,
    pub invoices: Arc<Mutex<Vec<InvoiceModel>>>,
    pub invoice_items: Arc<Mutex<Vec<InvoiceItemsModel>>>,
    pub stores: Arc<Mutex<Vec<StoreModel>>>,
}

impl DataContext {
    pub fn init() -> Result<DataContext, Box<dyn Error>> {
        let clients = read_from_file("./data/Clients.csv")?;
        let invoices = read_from_file("./data/Invoices.csv")?;
        let invoice_items = read_from_file("./data/InvoiceItems.csv")?;
        let stores = read_from_file("./data/Stores.csv")?;

        Ok(DataContext {
            clients: Arc::new(Mutex::new(clients)),
            invoices: Arc::new(Mutex::new(invoices)),
            invoice_items: Arc::new(Mutex::new(invoice_items)),
            stores: Arc::new(Mutex::new(stores)),
        })
    }
}

fn read_from_file<T>(path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let mut reader = csv::Reader::from_path(path)?;

    let mut results = Vec::new();

    for result in reader.deserialize() {
        let record: T = result?;

        results.push(record);
    }

    Ok(results)
}
