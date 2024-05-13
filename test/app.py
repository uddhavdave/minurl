import requests
import json
import random

minurl_server="http://localhost:8080/"

fb_url="https://facebook.com"
google_url="https://google.com"

def get_short_url_or_error(long_url):
    print({"long_url": f"{long_url}"})
    response = requests.post(
            url=minurl_server + "create",
            data=json.dumps({ "long_url": long_url }),
            headers={'Content-Type': 'application/json'}
            )
    if response.status_code == 200:
        data = response.json()
        if "short_url" in data:
            return data["short_url"], None
        else:
            return None, f"Key short_url not found in response"
    else:
        return None, f"Request failed with {response.status_code} and {response.text}"

def main():

    fb_token, error = get_short_url_or_error(fb_url)
    if error != None:
        print(error)
        return

    google_token, error = get_short_url_or_error(google_url)
    if error != None:
        print(error)
        return

    for _ in range(100):
        if random.randint(0,1) % 2 == 0:
            requests.get(url=minurl_server+ str(fb_token))
        else:
            requests.get(url=minurl_server+ str(google_token))

if __name__ == "__main__":
    main()
