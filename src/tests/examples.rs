#[cfg(test)]
mod tests {

    use anyhow::Result;

    fn count_concurrent(len: usize, sleep: u64) -> Result<i32> {
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
        let target = 2468;
        let count = count_concurrent(target, 1_000)?;
        assert_eq!(count as usize, target);
        // within slight margin of error, operation should complete
        // concurrently in about 1,000 ms, even with a sleep
        // and as target number grows or shrinks
        let elapsed = now.elapsed().as_millis();
        assert!(elapsed >= 1_000 && elapsed <= 2_000);
        println!("operation complete: {:.2?}", now.elapsed());
        Ok(())
    }
}
