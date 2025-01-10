// ******************************************************************************

//THIS KIND OF CODE IS ONLY WRITTEN FOR MOCKING PURPOSE. IN PRODUCTION ONE IS NOT SUPPOSE TO WRITE IT.

// ******************************************************************************
use crate::{web, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

//Tickets Types model
#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

//Model controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub fn new() -> Result<Self> {
        Ok(ModelController {
            tickets_store: Arc::default(),
        })
    }
}

impl ModelController {
    pub async  fn create_ticket(&self, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64 + 1;
        let ticket = Ticket {
            id,
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }
    pub async  fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|ticket| ticket.clone()).collect();
        Ok(tickets)
    }
    pub async  fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    
    }
}
