use std::fs;

use termion::color;
use walkdir::WalkDir;
use question::{ Answer, Question };
use clap::{ AppSettings, Clap };

extern crate question;
extern crate termion;

#[derive(Clap)]
#[clap(version = "1.0", author = "Cryptobyte <me@cryptobyte.dev>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short = 'i', long)]
    input: String,

    #[clap(short = 'o', long)]
    output: String,

    #[clap(short = 'r', long)]
    recursive: bool,

    #[clap(short, long)]
    follow_links: bool,

    #[clap(short, long)]
    yes: bool,

    #[clap(short = 'v', long)]
    verbose: bool,

    #[clap(short = 'd', long, default_value = "4096")]
    depth: usize
}

fn main() {
  let opts: Opts = Opts::parse();

  let imd = fs::metadata(&opts.input);
  if !imd.is_ok() || !imd.unwrap().is_dir() {
    println!("{}", "Input directory does not exist.");
    return;
  }

  if !fs::metadata(&opts.output).is_ok() {
    if !opts.yes {
      let _create = Question::new("Output directory does not exist, create it?")
        .default(Answer::NO)
        .show_defaults()
        .confirm();

      if _create == Answer::NO {
        return;
      }
    }

    fs::create_dir_all(&opts.output)
      .unwrap_or_else(|e| panic!("Error creating dir: {}", e));
  }

  if !(fs::metadata(&opts.output).unwrap().is_dir()) {
    println!("{}", "Output directory is a file.");
  }

  if !opts.yes {
    let _question = format!("Copy directory structure from {}{}{} to {}{}{}? (recursive: {})", 
      color::Fg(color::Cyan), opts.input, color::Fg(color::Reset), 
      color::Fg(color::Cyan), opts.output, color::Fg(color::Reset), 
      opts.recursive
    );

    let _confirm = Question::new(&_question)
      .default(Answer::NO)
      .show_defaults()
      .confirm();

    if _confirm == Answer::NO {
      return;
    }
  }

  for entry in WalkDir::new(opts.input)       // Why is this so hard to do without a crate?!
    .follow_links(opts.follow_links)          // Allow the option to follow symlinks
    .into_iter()
    .filter_entry(|e| e.file_type().is_dir()) // Filter out non-directories
    .filter_map(|e| e.ok()) {                 // Filter out errors, probably not the best idea.. but okay for now!
    
    // Skip the root directory
    if entry.depth() == 0 {
      continue;
    }

    // Only use depth 1 for non-recursive to skip subdirectories
    if !opts.recursive && entry.depth() > 1 {
      continue;
    }

    if opts.recursive && entry.depth() > opts.depth {
      println!("Skipping {} > {} {}", entry.depth(), opts.depth, entry.path().display());
      continue;
    }

    let _path = entry.path().to_str().unwrap();
    let _split: Vec<&str> = _path.split('/').collect();
    let (_, a) = _split.split_at(_split.len() - entry.depth());
    let _res = format!("{}/{}", opts.output, a.join("/"));

    if opts.verbose {
      println!("Creating {}{}{} from {}{}{}", 
        color::Fg(color::Cyan), _res, color::Fg(color::Reset), 
        color::Fg(color::Cyan), entry.path().display(), color::Fg(color::Reset)
      );
    }

    if fs::metadata(&_res).is_ok() {
      if !opts.yes {
        let _msg = format!("Directory {} exists, overwrite it?", _res);
        let _overwrite = Question::new(&_msg)
          .default(Answer::NO)
          .show_defaults()
          .confirm();
  
        if _overwrite == Answer::NO {
          continue;
        }

      } else {
        continue;
      }
  
      fs::remove_dir_all(&_res)
        .unwrap_or_else(|e| panic!("Error overwriting dir: {}", e));
    }

    fs::create_dir(_res)
      .unwrap_or_else(|e| println!("Error creating dir: {}", e));
  }
}