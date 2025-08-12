mod airlist;

use pelican_ui::{Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS, HardwareContext, runtime::Services};
use maverick_os::hardware::camera::ImageSettings;

pub struct MyApp;

start!(MyApp);