use rltk::rex::XpFile;

rltk::embedded_resource!(MENU_IMAGE, "../resources/mq_80x50.xp");
rltk::embedded_resource!(MAP_TEST1, "../resources/map_test.xp");
rltk::embedded_resource!(WFC_DEMO1, "../resources/wfc-demo1.xp");
rltk::embedded_resource!(REX_MAP1, "../resources/rex/map/rex_map.xp");
rltk::embedded_resource!(REX_MAP2, "../resources/rex/map/rex_map_pop.xp");
pub struct RexAssets {
    pub menu: XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(MENU_IMAGE, "../resources/mq_80x50.xp");
        rltk::link_resource!(MAP_TEST1, "../resources/map_test.xp");
        rltk::link_resource!(WFC_DEMO1, "../resources/wfc-demo1.xp");
        rltk::link_resource!(REX_MAP1, "../resources/rex/map/rex_map.xp");
        rltk::link_resource!(REX_MAP2, "../resources/rex/map/rex_map_pop.xp");

        RexAssets{
            menu: XpFile::from_resource("../resources/mq_80x50.xp").unwrap()
        }
    }
}
