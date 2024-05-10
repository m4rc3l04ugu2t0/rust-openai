mod ais;
mod boddy;
mod error;

use crate::ais::assit;
use crate::ais::{assit::CreateConfig, new_oa_client};

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
	match start().await {
		Ok(_) => println!("\nBye!\n"),
		Err(e) => println!("\nError: {}\n", e),
	}
}

async fn start() -> Result<()> {
	let oac = new_oa_client()?;

	let asst_config = CreateConfig {
		name: "buddy-02".to_string(),
		model: "gpt-3.5-turbo-0125".to_string(),
	};
	let asst_id = assit::load_or_create_asst(&oac, asst_config, false).await?;

	println!("-->> {asst_id:?}");
	Ok(())
}
