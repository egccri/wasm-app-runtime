use crate::warg::{WargConfig, WargWrapper};
use crate::RegistryError::WargWrapperError;
use crate::{RegistryError, APP_STORE};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

// name: grpc-service, the package name of the warg, is equal App name.
// component_id: generate component id when publish
// path: sha256 of the warg generated, equal the path of content.
pub struct App {
    name: String,
    component_id: String,
    // the path of component file
    path: Option<PathBuf>,
}

pub struct AppStore {
    base_dir: PathBuf,
    apps: HashMap<String, App>,
    warg_wrapper: WargWrapper,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppStoreConfig {
    base_idr: PathBuf,
    warg_config: WargConfig,
    auth: AppStoreAuth,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppStoreAuth {}

impl<'a> AppStore {
    // 这个位置的是early bound lifetime，编译器生命周期检查规则是有关系的。
    // 具体意思是这个'a的生命周期实例是在这个WargWrapper实例创建时就应该单态化。
    // 生命周期参数 'a 指定了这个引用的有效期限，它与 WargWrapper 的实例绑定。
    // 具体来说，'a 生命周期表示这个引用的有效期限与 WargWrapper 实例相同。
    // 也就是说，当通过调用 get 函数获取 WargWrapper 实例的引用时，这个引用将保持有效，
    // 直到对应的 WargWrapper 实例被销毁。
    pub fn get(config: AppStoreConfig) -> &'a AppStore {
        APP_STORE.get_or_init(|| Self::init(config))
    }

    pub fn init(config: AppStoreConfig) -> AppStore {
        let warg_wrapper = WargWrapper::new(config.warg_config);
        AppStore {
            base_dir: config.base_idr,
            apps: HashMap::new(),
            warg_wrapper,
        }
    }

    pub async fn install_app(app_name: &str) -> Result<App, RegistryError> {
        let mut app_store = APP_STORE
            .get()
            .ok_or_else(|| WargWrapperError("App store is not initial.".to_string()))?;
        let app_path = app_store.warg_wrapper.download(app_name, None).await?;
        let app = App {
            name: app_name.to_string(),
            component_id: "".to_string(),
            path: Some(app_path),
        };
        // add app to the store
        Ok(app)
    }

    pub async fn update_app() -> Result<App, RegistryError> {
        todo!()
    }
}
