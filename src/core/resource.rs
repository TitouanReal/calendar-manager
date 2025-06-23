use gtk::gio;
use tracing::error;
use tsparql::SparqlConnection;

use crate::core::{Calendar, Collection, Event, Provider};

#[derive(Debug)]
pub enum Resource {
    Provider(Provider),
    Collection(Collection),
    Calendar(Calendar),
    Event(Event),
}

impl Resource {
    pub fn from_uri(read_connection: &SparqlConnection, uri: &str) -> Result<Self, ()> {
        let is_event = {
            let statement = read_connection
                .query_statement(
                    "ASK {
                        ~uri a ccm:Event .
                    }",
                    None::<&gio::Cancellable>,
                )
                .unwrap()
                .unwrap();
            statement.bind_string("uri", uri);

            let cursor = match statement.execute(None::<&gio::Cancellable>) {
                Ok(cursor) => cursor,
                Err(err) => {
                    error!("Failed to execute query: {err}");
                    return Err(());
                }
            };

            match cursor.next(None::<&gio::Cancellable>) {
                Ok(true) => cursor.is_boolean(0),
                Ok(false) => {
                    error!("resource has no type");
                    return Err(());
                }
                Err(err) => {
                    error!("Failed to fetch resource type: {err}");
                    return Err(());
                }
            }
        };

        if is_event {
            return Ok(Self::Event(Event::from_uri(read_connection, uri)?));
        }

        let is_calendar = {
            let statement = read_connection
                .query_statement(
                    "ASK {
                        ~uri a ccm:Calendar .
                    }",
                    None::<&gio::Cancellable>,
                )
                .unwrap()
                .unwrap();
            statement.bind_string("uri", uri);

            let cursor = match statement.execute(None::<&gio::Cancellable>) {
                Ok(cursor) => cursor,
                Err(err) => {
                    error!("Failed to execute query: {err}");
                    return Err(());
                }
            };

            match cursor.next(None::<&gio::Cancellable>) {
                Ok(true) => cursor.is_boolean(0),
                Ok(false) => {
                    error!("resource has no type");
                    return Err(());
                }
                Err(err) => {
                    error!("Failed to fetch resource type: {err}");
                    return Err(());
                }
            }
        };

        if is_calendar {
            return Ok(Self::Calendar(Calendar::from_uri(read_connection, uri)?));
        }

        let is_collection = {
            let statement = read_connection
                .query_statement(
                    "ASK {
                        ~uri a ccm:Collection .
                    }",
                    None::<&gio::Cancellable>,
                )
                .unwrap()
                .unwrap();
            statement.bind_string("uri", uri);

            let cursor = match statement.execute(None::<&gio::Cancellable>) {
                Ok(cursor) => cursor,
                Err(err) => {
                    error!("Failed to execute query: {err}");
                    return Err(());
                }
            };

            match cursor.next(None::<&gio::Cancellable>) {
                Ok(true) => cursor.is_boolean(0),
                Ok(false) => {
                    error!("resource has no type");
                    return Err(());
                }
                Err(err) => {
                    error!("Failed to fetch resource type: {err}");
                    return Err(());
                }
            }
        };

        if is_collection {
            return Ok(Self::Collection(Collection::from_uri(
                read_connection,
                uri,
            )?));
        }

        let is_provider = {
            let statement = read_connection
                .query_statement(
                    "ASK {
                        ~uri a ccm:Provider .
                    }",
                    None::<&gio::Cancellable>,
                )
                .unwrap()
                .unwrap();
            statement.bind_string("uri", uri);

            let cursor = match statement.execute(None::<&gio::Cancellable>) {
                Ok(cursor) => cursor,
                Err(err) => {
                    error!("Failed to execute query: {err}");
                    return Err(());
                }
            };

            match cursor.next(None::<&gio::Cancellable>) {
                Ok(true) => cursor.is_boolean(0),
                Ok(false) => {
                    error!("resource has no type");
                    return Err(());
                }
                Err(err) => {
                    error!("Failed to fetch resource type: {err}");
                    return Err(());
                }
            }
        };

        if is_provider {
            return Ok(Self::Provider(Provider::from_uri(read_connection, uri)?));
        }

        error!("Resource is of unknown type");
        Err(())
    }
}
