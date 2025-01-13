// ******************************************************************************

//THIS KIND OF CODE IS ONLY WRITTEN FOR MOCKING PURPOSE. IN PRODUCTION ONE IS NOT SUPPOSE TO WRITE IT.

// ******************************************************************************
use crate::{ctx::Ctx, web, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};

//Tickets Types model
#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,
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
    /// Create a new ticket with the given title and add it to the store.
    ///
    /// Generates a unique ID for the ticket based on the current length of the store.
    ///
    /// # Arguments
    ///
    /// * `ticket_fc` - A `TicketForCreate` struct containing the title of the ticket to be created.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `Ticket` if successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue locking the ticket store.

    pub async fn create_ticket(&self, ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();
        let id = store.len() as u64 + 1;
        let ticket = Ticket {
            id,
            cid:ctx.user_id(),
            title: ticket_fc.title,
        };
        store.push(Some(ticket.clone()));
        Ok(ticket)
    }
    /// List all tickets in the store.
    ///
    /// The returned list is filtered to only include non-None elements.
    pub async fn list_tickets(&self, _ctx:Ctx) -> Result<Vec<Ticket>> {
        let store = self.tickets_store.lock().unwrap();

        let tickets = store.iter().filter_map(|ticket| ticket.clone()).collect();
        Ok(tickets)
    }
    /// Delete a ticket with the given `id`.
    ///
    /// This function will return an error if there is an issue locking the ticket store.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the ticket to be deleted.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the deleted `Ticket` if successful.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is an issue locking the ticket store, or if the
    /// ticket with the given ID does not exist.
    pub async fn delete_ticket(&self, id: u64,_ctx:Ctx) -> Result<Ticket> {
        let mut store = self.tickets_store.lock().unwrap();

        let ticket = store.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
