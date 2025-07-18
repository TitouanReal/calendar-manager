mod application;
mod config;
mod utils;
mod widgets;

use self::application::CalendarManagerApplication;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib, prelude::*};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

fn main() -> glib::ExitCode {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("calendar_manager=debug,ccm=debug,warn"));

    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(env_filter))
        .init();

    glib::set_application_name("CalendarManager");

    bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
        .expect("Unable to set the text domain encoding");
    textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    let resources = gio::Resource::load(PKGDATADIR.to_owned() + "/calendar-manager.gresource")
        .expect("Could not load resources");
    gio::resources_register(&resources);

    let app = CalendarManagerApplication::new(
        "io.gitlab.TitouanReal.CalendarManager",
        &gio::ApplicationFlags::empty(),
    );

    app.run()
}
