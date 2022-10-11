#[cfg(test)]
mod tests {

    use anyhow::Result;

    fn count_concurrent(len: usize, sleep: u64) -> anyhow::Result<i32> {
        let wg = crossbeam::sync::WaitGroup::new();
        let (s, r) = crossbeam::channel::bounded(len);
        s.send(0)?;
        for _ in 0..len {
            std::thread::spawn({
                let wg = wg.clone();
                let s = s.clone();
                let r = r.clone();
                move || -> Result<()> {
                    std::thread::sleep(std::time::Duration::from_millis(sleep));
                    let count = r.recv()?;
                    s.send(count + 1)?;
                    wg.wait();
                    Ok(())
                }
            });
        }
        wg.wait();
        Ok(r.recv()?)
    }

    #[test]
    fn should_count_concurrently() -> Result<()> {
        let now = std::time::Instant::now();
        assert_eq!(count_concurrent(2, 5_00)?, 2);
        println!("operation complete: {:.2?}", now.elapsed());
        Ok(())
    }
}
