use std::fs;
use std::io;
use std::io::Write;

pub struct TailwindCSS;

impl TailwindCSS {
    pub fn create_tailwindcss_files(mut workdir: String) -> io::Result<()> {
        workdir.push_str("/src/styles/");

        fs::remove_dir_all(&workdir)?;
        fs::create_dir(&workdir)?;

        workdir.push_str("/globals.css");

        let mut f = fs::File::create(&workdir).expect("Failed to remake the global CSS file.");

        f.write_all(TAILWINDCSS_CSS_FILE.as_bytes())
            .expect("Failed to fill the tailwindCSS file with text.");

        Ok(())
    }

    pub fn setup_tailwind_config(mut workdir: String) {
        workdir.push_str("/tailwind.config.js");

        let mut f =
            fs::File::create(&workdir).expect("Failed to recreate the Tailwind config file");

        f.write_all(TAILWINDCSS_CONFIG_FILE.as_bytes())
            .expect("Failed to fill the tailwind config file :(");
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
