use clap::Parser;

#[derive(Parser)]
#[command(name = "uncensor-icon")]
#[command(
    about = "A tool to compare and extract different icons between Global and CN WoW versions"
)]
pub struct Args {
    /// Path to global version's `_retail_\BlizzardInterfaceArt\Interface\ICONS` folder
    #[arg(long)]
    pub global_dir: String,

    /// Path to CN version's `_retail_\BlizzardInterfaceArt\Interface\ICONS` folder
    #[arg(long)]
    pub cn_dir: String,

    /// Path to output folder where different icons will be copied
    #[arg(long)]
    pub output_dir: String,

    /// Blacklist of files to exclude (comma-separated)
    #[arg(
        long,
        default_value = "XP_ICON.BLP,XPBonus_Icon.blp,Garr_CurrencyIcon-Xp.blp"
    )]
    pub blacklist: String,
}
