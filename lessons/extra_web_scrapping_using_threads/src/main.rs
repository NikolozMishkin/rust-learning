use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use ureq::{Agent, AgentBuilder};
fn main() -> Result<(), ureq::Error> {
    let webpages = vec![
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
        "https://gist.github.com/recluze/1d2989c7e345c8c3c542",
        "https://gist.github.com/recluze/a98aa1804884ca3b3ad3",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/460157afc6a7492555bb",
        "https://gist.github.com/recluze/5051735efe3fc189b90d",
        "https://gist.github.com/recluze/c9bc4130af995c36176d",
    ];

    let agent = ureq::AgentBuilder::new().build();
    let now = Instant::now();

    for web_page in &webpages {
        let web_body = agent.get(web_page).call()?.into_string()?;
    }
    println!("Time taken wihtout Threads: {:.2?}", now.elapsed());

    let now = Instant::now();
    let agent = Arc::new(agent);
    let t0 = Instant::now();
    let mut handles: Vec<thread::JoinHandle<Result<(Duration, Duration), ureq::Error>>> =
        Vec::new();

    for web_page in webpages {
        let agent_thread = agent.clone();
        let t = thread::spawn(move || {
            let started = t0.elapsed();
            let web_body = agent_thread.get(web_page).call()?.into_string()?;

            Ok((started, t0.elapsed()))
        });
        handles.push(t);
    }

    // Каждый поток вернул, когда он начал и когда закончил (в мс от общего старта t0).
    // Плюс замеряем, сколько main реально простоял на каждом join.
    let mut rows = Vec::new();
    for (i, handle) in handles.into_iter().enumerate() {
        let before_join = Instant::now();
        let (start, end) = handle.join().unwrap()?;
        rows.push((i, start, end, before_join.elapsed()));
    }

    let total = t0.elapsed();
    let width = 60.0;
    let scale = |d: Duration| (d.as_secs_f64() / total.as_secs_f64() * width).round() as usize;

    println!("\n0{:>width$}", format!("{:.0?}", total), width = width as usize);
    for (i, start, end, waited) in &rows {
        let lead = scale(*start);
        let len = scale(*end).saturating_sub(lead).max(1);
        println!(
            "#{i:2} |{:lead$}{:=<len$}{:>pad$}| start {:>7.1?}  end {:>7.1?}  join waited {:>9.2?}",
            "",
            "",
            "",
            start,
            end,
            waited,
            lead = lead,
            len = len,
            pad = (width as usize).saturating_sub(lead + len),
        );
    }

    println!("\nTime taken using Threads: {:.2?}", now.elapsed());
    Ok(())
}
