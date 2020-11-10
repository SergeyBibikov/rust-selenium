## Docs

[docs](https://docs.rs/selenium_webdriver/)

# Purpose

The main purpose of this crate is to provide the means of interacting with the selenium server.
Provides basic functionality for UI automated tests and scripts.

# Requirements

* Selenium server running on localhost:4444
* A chromedriver or geckodriver

# Limitations

* Chromedriver: currently ChromeOptions are not fully supported, but arguments may be passed to the session. (See the following source for possible args: https://peter.sh/experiments/chromium-command-line-switches/)

* Geckodriver session does not support any customization possibilities at the moment

# TO DO

1) ChromeOptions
2) Firefox session adjustments
3) Useful methods which are implemented in other Selenium implementations