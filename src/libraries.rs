use crate::library::Library;
use crate::Result;

pub struct Libraries {
    pub client: Library,
    pub engine: Library,
    pub materialsystem: Library,
    pub vguimatsurface: Library,
    pub vgui2: Library,
    pub inputsystem: Library,
    pub vphysics: Library,
    pub localize: Library,
    //pub panorama: Library,
    pub fs_stdio: Library,
    pub matchmaking: Library,
}

impl Libraries {
    pub fn new() -> Result<Self> {
        let client = Library::client()?;
        let engine = Library::engine()?;
        let materialsystem = Library::materialsystem()?;
        let vguimatsurface = Library::vguimatsurface()?;
        let vgui2 = Library::vgui2()?;
        let inputsystem = Library::inputsystem()?;
        let vphysics = Library::vphysics()?;
        let localize = Library::localize()?;
        //let panorama = Library::panorama()?;
        let fs_stdio = Library::fs_stdio()?;
        let matchmaking = Library::matchmaking()?;

        Ok(Self {
            client,
            engine,
            materialsystem,
            vguimatsurface,
            vgui2,
            inputsystem,
            vphysics,
            localize,
            //panorama,
            fs_stdio,
            matchmaking,
        })
    }
}
