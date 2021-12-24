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
        println!("loading client");
        let client = Library::client()?;
        println!("loading engine");
        let engine = Library::engine()?;
        println!("loading materialsystem");
        let materialsystem = Library::materialsystem()?;
        println!("loading vguimatsurface");
        let vguimatsurface = Library::vguimatsurface()?;
        println!("loading vgui2");
        let vgui2 = Library::vgui2()?;
        println!("loading inputsystem");
        let inputsystem = Library::inputsystem()?;
        println!("loading vphysics");
        let vphysics = Library::vphysics()?;
        println!("loading localize");
        let localize = Library::localize()?;
        println!("loading panorama");
        //let panorama = Library::panorama()?;
        println!("loading fs stdio");
        let fs_stdio = Library::fs_stdio()?;
        println!("loading matchmaking");
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
