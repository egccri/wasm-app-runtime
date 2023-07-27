use kv_storage::StorageSled;
use micro_async_module::App;
use runtime_server::RuntimeServer;

/// sub command run egccri modules.
#[derive(clap::Parser, Debug)]
pub struct RunCommand {
    /// Runtime server address
    pub server_addr: String,
}

impl RunCommand {
    pub fn execute(self) -> anyhow::Result<()> {
        let app = App::new("egccri".to_string(), 4, 100);
        let runtime_server = RuntimeServer::new(self.server_addr);
        app.add_module(StorageSled).add_module(runtime_server).run();

        // Run api server
        Ok(())
    }
}
