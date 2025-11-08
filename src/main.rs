mod scanner;
mod nerdfont;
mod color_mapper;

use clap::{Arg, Command, ArgAction};
use colored::*;

fn get_dotfiles(dir: &str) -> Vec<String> {
    let mut dotfiles = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry_res in entries {
            if let Ok(entry) = entry_res {
                if let Ok(name) = entry.file_name().into_string() {
                    if name.starts_with('.') && entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        dotfiles.push(name);
                    }
                }
            }
        }
    }
    dotfiles.sort_unstable();
    dotfiles
}

fn build_app() -> Command {
    Command::new("extscan")
        .version("0.1.0")
        .author("Prateek Maru | https://github.com/xprateek")
        .about("File extension analyzer with color and icon support")
        .arg(Arg::new("color").short('c').help("Enable colored output").action(ArgAction::SetTrue))
        .arg(Arg::new("nerdfont").short('n').help("Enable Nerd Font icons").action(ArgAction::SetTrue))
        .arg(Arg::new("count").short('x').help("Show counts next to extensions").action(ArgAction::SetTrue))
        .arg(
            Arg::new("path")
                .help("Path of directory to scan (default current dir)")
                .index(1)
                .num_args(1)
                .default_value("."),
        )
        .arg(Arg::new("sort1").short('1').help("Sort files alphabetically including dotfiles first").action(ArgAction::SetTrue))
        .arg(Arg::new("hidden").short('h').help("Include hidden files and dotfiles").action(ArgAction::SetTrue))
        .arg(Arg::new("list").short('l').help("List dotfiles separately, then extensions").action(ArgAction::SetTrue))
}

fn main() -> std::io::Result<()> {
    let matches = build_app().get_matches();

    let enable_color = matches.get_one::<bool>("color").copied().unwrap_or(false);
    let enable_nerdfont = matches.get_one::<bool>("nerdfont").copied().unwrap_or(false);
    let show_counts = matches.get_one::<bool>("count").copied().unwrap_or(false);
    let scan_path = matches.get_one::<String>("path").map(|s| s.as_str()).unwrap_or(".");
    let sort_1 = matches.get_one::<bool>("sort1").copied().unwrap_or(false);
    let include_hidden = matches.get_one::<bool>("hidden").copied().unwrap_or(false);
    let list_mode = matches.get_one::<bool>("list").copied().unwrap_or(false);

    let counts = scanner::visit_dirs_and_count(scan_path)?;

    let mut counts_vec: Vec<(&String, &usize)> = counts.iter().collect();

    if !include_hidden {
        counts_vec.retain(|(ext, _)| !ext.starts_with('.'));
    }

    if sort_1 {
        counts_vec.sort_by(|a, b| {
            let a_is_dot = a.0.starts_with('.');
            let b_is_dot = b.0.starts_with('.');
            match (a_is_dot, b_is_dot) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.0.to_lowercase().cmp(&b.0.to_lowercase()),
            }
        });
    } else {
        counts_vec.sort_by(|a, b| b.1.cmp(a.1));
    }

    let total_count: usize = counts.values().sum();

    let dotfiles = if include_hidden || list_mode {
        get_dotfiles(scan_path)
    } else {
        vec![]
    };

    if (list_mode || include_hidden) && !dotfiles.is_empty() {
        println!("Hidden Files");
        for df in &dotfiles {
            println!("{}", df);
        }
        println!();
    }

    if list_mode {
        if !counts_vec.is_empty() {
            println!("Files In Directory");
            for (ext, count) in &counts_vec {
                let icon = if enable_nerdfont {
                    nerdfont::get_icon(ext)
                } else {
                    None
                };
                let color = color_mapper::get_color(ext);
                let display_icon = icon.map(|c| c.to_string()).unwrap_or_else(|| "?".to_string());

                if enable_color {
                    if show_counts {
                        println!(
                            "{}  {} ({})",
                            display_icon.color(color),
                            ext.color(color).bold(),
                            count
                        );
                    } else {
                        println!("{}  {}", display_icon.color(color), ext.color(color).bold());
                    }
                } else {
                    if show_counts {
                        println!("{} ({})", ext, count);
                    } else {
                        println!("{}", ext);
                    }
                }
            }
        }

        println!();
        println!("Total files: {}", total_count);
        println!("Extensions:{}", counts_vec.iter().map(|(e, _)| format!(" {}", e)).collect::<String>());
    } else {
        if !enable_color {
            for (ext, count) in &counts_vec {
                if enable_nerdfont {
                    let icon = nerdfont::get_icon(ext);
                    let icon_str = icon.map(|c| c.to_string()).unwrap_or_else(|| "?".to_string());

                    if show_counts {
                        print!("{} {}({}) ", icon_str, ext, count);
                    } else {
                        print!("{} {} ", icon_str, ext);
                    }
                } else {
                    if show_counts {
                        print!("{}({}) ", ext, count);
                    } else {
                        print!("{} ", ext);
                    }
                }
            }
            println!();
        } else {
            for (ext, count) in &counts_vec {
                let icon = if enable_nerdfont {
                    nerdfont::get_icon(ext)
                } else {
                    None
                };
                let color = color_mapper::get_color(ext);

                let display_icon = match icon {
                    Some(ch) => ch.to_string(),
                    None => "?".to_string(),
                };

                if show_counts {
                    print!(
                        "{} {} {} ",
                        display_icon.color(color),
                        ext.color(color).bold(),
                        format!("({})", count).color(color)
                    );
                } else {
                    print!("{} {} ", display_icon.color(color), ext.color(color).bold());
                }
            }
            println!();
        }

        println!("Total files: {}", total_count);
        println!("Extensions:{}", counts_vec.iter().map(|(e, _)| format!(" {}", e)).collect::<String>());
    }

    Ok(())
}
