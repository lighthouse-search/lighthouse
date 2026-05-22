from selenium import webdriver
import os
import requests

from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

host = os.getenv("lighthouse_host", "http://lighthouse-service.production.svc.cluster.local")
extension_dir = os.getenv(
    "EXTENSION_DIR",
    os.path.join(os.path.dirname(__file__), "..", "extension"),
)

# Ask the crawl queue API for the next URL to crawl. The server reads it from
# SQL and atomically claims it for this node, so it won't be handed to another.
resp = requests.get(host + "/api/native-v1/crawler/queue", timeout=10)
resp.raise_for_status()
queue = resp.json().get("data", [])

if not queue:
    print("Crawl queue empty, nothing to do.")
    raise SystemExit(0)

job = queue[0]
url = job["url"]
print(f"Crawling... {url}")

# Selenium 4 manages a per-session profile under TMPDIR (a tmpfs emptyDir in
# the container), so no FirefoxProfile() juggling is needed.
options = Options()
options.add_argument("-headless")
options.set_preference("general.useragent.override", "LighthouseBot/1.0")

driver = webdriver.Firefox(options=options)
driver.implicitly_wait(0.5)
driver.install_addon(extension_dir, temporary=True)
driver.get(url)
title = driver.title
# The content script (borderify.js) navigates to about:blank after POSTing
# discovered URLs back — that's our signal that the page is done.
WebDriverWait(driver, 30).until(lambda d: d.current_url == "about:blank")
driver.quit()
