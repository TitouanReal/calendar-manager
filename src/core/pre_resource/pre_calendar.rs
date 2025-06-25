use gtk::gio;
use tracing::error;
use tsparql::{prelude::*, SparqlConnection};

pub struct PreCalendar {
    pub uri: String,
    pub collection_uri: String,
    pub name: String,
}

impl PreCalendar {
    /// Retrieves a calendar resource from a URI.
    ///
    /// # Panics
    ///
    /// This function may panic if the given URI is invalid or does not point to a calendar resource.
    pub fn from_uri(read_connection: &SparqlConnection, uri: &str) -> Result<Self, ()> {
        let cursor = read_connection
            .query(
                &format!(
                    "SELECT ?calendar_name ?collection
                    FROM ccm:Calendar
                    WHERE {{
                        \"{}\" rdfs:label ?calendar_name ;
                            ccm:collection ?collection .
                    }}",
                    uri
                ),
                None::<&gio::Cancellable>,
            )
            .unwrap();

        match cursor.next(None::<&gio::Cancellable>) {
            Err(e) => {
                error!("Encountered glib error: {}", e);
                Err(())
            }
            Ok(false) => {
                error!("Resource {uri} was created but is not found in database");
                Err(())
            }
            Ok(true) => {
                let calendar_name = cursor.string(0).unwrap();
                let collection_uri = cursor.string(1).unwrap();
                let calendar = Self {
                    uri: uri.to_string(),
                    collection_uri: collection_uri.to_string(),
                    name: calendar_name.to_string(),
                };

                Ok(calendar)
            }
        }
    }
}
