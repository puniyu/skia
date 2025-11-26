mod store;

use std::sync::{LazyLock, RwLock};

use skia_safe::textlayout::{FontCollection, TypefaceFontProvider};

static FONT_STORE: LazyLock<RwLock<store::FontStore>> =
    LazyLock::new(|| RwLock::new(store::FontStore::default()));

pub struct FontManger {
    font_collection: FontCollection,
    font_provider: TypefaceFontProvider,
}

impl Default for FontManger {
    fn default() -> Self {
        Self::new()
    }
}

impl FontManger {
    /// 创建新的字体管理器
    pub fn new() -> Self {
        let mut font_collection = FontCollection::new();
        let font_provider = TypefaceFontProvider::new();

        font_collection.set_asset_font_manager(Some(font_provider.clone().into()));

        Self {
            font_collection,
            font_provider,
        }
    }

    /// 注册字体
    ///
    /// # 参数
    /// - `font_data`: 字体文件的二进制数据
    /// - `font_family`: 可选的字体族名,如果为 None 则使用字体内置的族名
    ///
    /// # 返回
    /// - `Ok(String)`: 成功注册,返回字体族名
    /// - `Err(String)`: 注册失败,返回错误信息
    pub fn register_font(
        &mut self,
        font_data: &[u8],
        font_family: Option<&str>,
    ) -> Result<String, String> {
        let mut store = FONT_STORE
            .write()
            .map_err(|e| format!("获取字体存储锁失败: {}", e))?;

        if store.insert(font_data, font_family) {
            let family_name = font_family
                .map(|s| s.to_string())
                .or_else(|| {
                    skia_safe::FontMgr::default()
                        .new_from_data(font_data, None)
                        .map(|tf| tf.family_name())
                })
                .ok_or_else(|| "无法获取字体族名".to_string())?;
            if let Some(typeface) = store.get(&family_name) {
                self.font_provider
                    .register_typeface(typeface.clone(), Some(family_name.as_str()));
                self.font_collection
                    .set_asset_font_manager(Some(self.font_provider.clone().into()));

                Ok(family_name)
            } else {
                Err("字体注册到存储成功,但无法从存储中获取".to_string())
            }
        } else {
            Err("无效的字体数据".to_string())
        }
    }

    /// 卸载字体
    ///
    /// # 参数
    /// - `font_family`: 要卸载的字体族名
    ///
    pub fn unregister_font(&mut self, font_family: &str) -> Result<(), String> {
        let mut store = FONT_STORE
            .write()
            .map_err(|e| format!("获取字体存储锁失败: {}", e))?;

        if store.remove(font_family) {
            self.font_provider = TypefaceFontProvider::new();

            for (family, typeface) in store.iter() {
                self.font_provider
                    .register_typeface(typeface.clone(), Some(family.as_str()));
            }
            self.font_collection
                .set_asset_font_manager(Some(self.font_provider.clone().into()));

            Ok(())
        } else {
            Err(format!("字体族 '{}' 不存在", font_family))
        }
    }

    pub fn font_collection(&self) -> &FontCollection {
        &self.font_collection
    }
}
