mod airlist;

use maverick_os::runtime::ServiceList;
use pelican_ui::{Context, Plugins, Plugin, maverick_start, start, Application, PelicanEngine, MaverickOS, HardwareContext, runtime::Services, include_assets};
use pelican_ui::drawable::Drawable;

pub struct MyApp;

impl Services for MyApp {
    fn services() -> ServiceList {
        ServiceList::default()
    }
}

impl Plugins for MyApp {
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
        vec![]
    }
}

impl Application for MyApp {
    //looks like where I declare all sprites that will be used.
    async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
        //which folder I can load assets from.
        ctx.assets.include_assets(include_assets!("./assets"));
        let mut illustrations = ctx.theme.brand.illustrations.clone();
        illustrations.insert(ctx, "spaceship", "spaceship.png");
        illustrations.insert(ctx, "b2", "b2.png");
        illustrations.insert(ctx, "tiki_fly", "tiki_fly.png");
        illustrations.insert(ctx, "northrop", "northrop.png");
        illustrations.insert(ctx, "bullet_downward", "bullet_downward.png");
        illustrations.insert(ctx, "bullet_blue", "bullet_blue.png");
        illustrations.insert(ctx, "explosion", "explosion.png");
        illustrations.insert(ctx, "f117", "f117.png");
        illustrations.insert(ctx, "player_lives", "player_lives.png");
        illustrations.insert(ctx, "background_blue", "background_blue.png");
        illustrations.insert(ctx, "bomb", "bomb.png");
        illustrations.insert(ctx, "green_terrain", "green_terrain.png");
        ctx.theme.brand.illustrations = illustrations;

        let game = Games::Airstrike.init(ctx);
        Box::new(Interface::new(ctx, game, None))
    }
}

start!(MyApp);