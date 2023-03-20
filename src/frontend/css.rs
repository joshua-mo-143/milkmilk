use std::fs;
use std::io;

use crate::utils::Utils;

pub struct TailwindCSS;

impl TailwindCSS {
    pub fn create_tailwindcss_files(mut workdir: String) -> io::Result<()> {
        workdir.push_str("/src/styles/");

        fs::remove_dir_all(&workdir)?;
        fs::create_dir(&workdir)?;

        workdir.push_str("/globals.css");

        Utils::write_to_file(&workdir, TAILWINDCSS_CSS_FILE).expect("Failed to write TailwindCSS CSS file.");

        Ok(())
            }

    pub fn setup_tailwind_config(mut workdir: String) {
        workdir.push_str("/tailwind.config.js");

        Utils::write_to_file(&workdir, TAILWINDCSS_CONFIG_FILE).expect("Failed to write TailwindCSS config file.");
            }
}

const TAILWINDCSS_CSS_FILE: &str = r#"@tailwind base;
@tailwind components;
@tailwind utilities;"#;

const TAILWINDCSS_CONFIG_FILE: &str = r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
"#;
