use color_eyre::{eyre::Context, Result};
use env_logger::{Builder, Target};
use log::LevelFilter;

pub fn config_logger(verbose_level: u8, target: Target) -> Result<()> {
    let mut builder = Builder::from_default_env();

    builder
        .target(target)
        .default_format()
        .format_indent(Some(4))
        .format_module_path(false)
        .format_timestamp_millis()
        .write_style(env_logger::WriteStyle::Always);

    if verbose_level == 1 {
        builder.filter(None, LevelFilter::Debug);
    } else if verbose_level == 2 {
        builder.filter(None, LevelFilter::Trace);
    } else {
        builder.filter(None, LevelFilter::Info);
    }

    builder.try_init().with_context(|| "Error config logger")
}
