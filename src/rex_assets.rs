use rltk::rex::XpFile;

rltk::embedded_resource!(MENU_IMAGE, "../resources/mq_80x50.xp");
rltk::embedded_resource!(WFC_DEMO1, "../resources/wfc-demo1.xp");
rltk::embedded_resource!(WFC_DEMO2, "../resources/wfc-demo2.xp");

pub struct RexAssets {
    pub menu: XpFile
}

impl RexAssets {
    
    pub fn new() -> RexAssets {
        rltk::link_resource!(MENU_IMAGE, "../resources/mq_80x50.xp");
        rltk::link_resource!(WFC_DEMO1, "../resources/wfc-demo1.xp");
        rltk::link_resource!(WFC_DEMO2, "../resources/wfc-demo2.xp");

        RexAssets{
            menu: XpFile::from_resource("../resources/mq_80x50.xp").unwrap()
        }
    }
}
