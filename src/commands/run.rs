use kv_storage::StorageSled;
use micro_async_module::App;

/// sub command run egccri modules.
#[derive(clap::Parser, Debug)]
pub struct RunCommand {}

impl RunCommand {
    pub fn execute(self) -> anyhow::Result<()> {
        let app = App::new("egccri".to_string(), 4, 100);
        app.add_module(StorageSled).run();
        Ok(())
    }
}
