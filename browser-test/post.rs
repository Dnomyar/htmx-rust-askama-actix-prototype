mod tests {
    use std::{
        path::{Path, PathBuf},
        time::Duration,
    };

    use playwright::Playwright;
    #[tokio::test]
    async fn test_posts() -> Result<(), playwright::Error> {
        println!("Initializing playwright");
        let playwright = Playwright::initialize().await?;
        // playwright.install_firefox()?; // Install browsers
        let chromium = playwright.chromium();
        let browser = chromium
            .launcher()
            .headless(true)
            .timeout(3000.0)
            .executable(Path::new(
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            ))
            .launch()
            .await?;
        let context = browser.context_builder().build().await?;
        println!("Navigating to http://localhost:8080");
        let page = context.new_page().await?;
        page.goto_builder("http://localhost:8080").goto().await?;

        // // Exec in browser and Deserialize with serde
        let s: String = page.eval("() => location.href").await?;
        assert_eq!(s, "http://localhost:8080/");
        page.screenshot_builder()
            .path(PathBuf::from("screenshots/1.png"))
            .screenshot()
            .await?;
        println!("Clicking login button");
        page.click_builder(".login-button").click().await?;
        println!("Clicking on add post button");
        page.click_builder(".add-post-button").click().await?;
        println!("Filling out form");
        page.fill_builder(".post-title-input", "Hello, world! 123")
            .fill()
            .await?;
        page.fill_builder(".post-content-input", "My first post!")
            .fill()
            .await?;
        page.screenshot_builder()
            .path(PathBuf::from("screenshots/2.png"))
            .screenshot()
            .await?;
        tokio::time::sleep(Duration::from_millis(3000)).await;
        println!("Submitting form");
        page.click_builder(".submit-post-button").click().await?;
        println!("Waiting for post to appear");
        page.screenshot_builder()
            .path(PathBuf::from("screenshots/3.png"))
            .screenshot()
            .await?;
        let post_title: String = page
            .eval("() => document.querySelector('.posts .post-title').innerText")
            .await?;
        assert_eq!(post_title, "Hello, world! 123");

        let post_content: String = page
            .eval("() => document.querySelector('.posts .post-content').innerText")
            .await?;
        assert_eq!(post_content, "My first post!");
        Ok(())
    }
}
