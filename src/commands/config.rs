/// commands/config.rs
/// Command related to configuration.
    
use crate::utils::editor::open_in_default_editor;

pub fn run_edit() -> anyhow::Result<()> {
    let conf_path = crate::config::get_config_file();

    open_in_default_editor(&conf_path)?;

    Ok(())
}