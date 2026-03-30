use anyhow::{Result, bail};

struct Message {
    content: Option<String>,
    stop_reason: Option<String>,
}

fn get_message(val: u8) -> Result<Message> {
    match val {
        1 => Ok(Message {
            content: Some("Content".to_owned()),
            stop_reason: None,
        }),
        2 => Ok(Message {
            content: None,
            stop_reason: Some("Done".to_owned()),
        }),
        _ => bail!(""),
    }
}

fn trying_let_else() -> Result<String> {
    let mut val: u8 = 1;
    while let Ok(message) = get_message(val) {
        if let Some(stp_reason) = message.stop_reason {
            println!("stopping because of {stp_reason}");
            break;
        }
        let Some(mssg) = message.content else {
            bail!("message")
        };
        println!("messgae: {}", mssg);
        val += 1;
    }
    Ok("Result".to_owned())
}

pub fn run() -> anyhow::Result<()> {
    trying_let_else()?;
    Ok(())
}
