from selenium import webdriver
import os
import requests

from selenium.webdriver.firefox.options import Options
from selenium.webdriver.firefox.firefox_profile import FirefoxProfile
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

x = requests.get(os.getenv("lighthouse_host", "http://lighthouse-service.production.svc.cluster.local")+"/api/native-v1/crawler/queue")
data = x.json()
print(f"Crawling... {data['data'][0]["url"]}")

options = Options()
firefox_profile = FirefoxProfile()
firefox_profile.set_preference("general.useragent.override", "LighthouseBot/1.0")
options.profile = firefox_profile

driver = webdriver.Firefox(options=options)
driver.implicitly_wait(0.5)
driver.install_addon("../extension", temporary=True)
driver.get(data['data'][0]["url"])
title = driver.title
WebDriverWait(driver, 30).until(lambda d: d.current_url == "about:blank")
# driver.implicitly_wait(0.5)
driver.quit()