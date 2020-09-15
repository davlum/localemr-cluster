import time
import requests
from main import FOUR_HUNDRED_ERROR, Status


def test_post_batch_succeeds():
    url = 'http://localhost:8998/batch/sleep-1'
    r = requests.post(url, json={'args': ['sleep', '10']})
    r.raise_for_status()
    assert r.json() == {'id': 'sleep-1', 'status': 'PENDING', 'log': None}
    while r.json()['status'] == 'PENDING':
        time.sleep(3)
        r = requests.get(url)
        r.raise_for_status()
    assert r.json()['status'] == 'RUNNING'
    while r.json()['status'] == 'RUNNING':
        time.sleep(3)
        r = requests.get(url)
        r.raise_for_status()
    assert r.json()['status'] == 'SUCCEEDED'


def test_post_batch_fails():
    url = 'http://localhost:8998/batch/sleep-2'
    r = requests.post(url, json={'args': ['sleep', '20', '&&', 'ls', '-lthra']})
    r.raise_for_status()
    assert r.json() == {'id': 'sleep-2', 'status': 'PENDING', 'log': None}
    while r.json()['status'] == 'PENDING':
        time.sleep(3)
        r = requests.get(url)
        r.raise_for_status()
    assert r.json()['status'] == 'FAILED'


def test_post_bad_batch_raises():
    url = 'http://localhost:8998/batch/3'
    r = requests.post(url, data={'args': ['ls', '-lthra']})
    assert r.json() == FOUR_HUNDRED_ERROR
    r = requests.post(url, json={'not_args': ['ls', '-lthra']})
    assert r.json() == FOUR_HUNDRED_ERROR


def test_double_post_batch_raises():
    url = 'http://localhost:8998/batch/2'
    requests.post(url, json={'args': ['ls', '-lthra']})
    r = requests.post(url, json={'args': ['ls', '-lthra']})
    assert r.status_code == 400
    assert r.json()['error_message'] == "Batch `2` already exists."


def test_get_batch():
    url = 'http://localhost:8998/batch/sleep-1'
    r = requests.get(url)
    while r.json()['status'] in (str(Status.RUNNING), str(Status.PENDING)):
        time.sleep(2)
        r = requests.get(url)

    assert r.json() == {'id': 'sleep-1', 'status': 'SUCCEEDED', 'log': None}


def test_get_batch_not_found():
    r = requests.get('http://localhost:8998/batch/4')
    assert r.status_code == 404
    assert r.json()['error_message'] == "Batch `4` not found."


def test_health_check():
    r = requests.get('http://localhost:8998/health')
    assert r.status_code == 200
    assert r.json()['status'] == "OK"


def test_delete_batch():
    url = 'http://localhost:8998/batch/sleep-3'
    r = requests.post(url, json={'args': ['sleep', '20']})
    r.raise_for_status()
    assert r.json() == {'id': 'sleep-3', 'status': 'PENDING', 'log': None}
    r = requests.delete(url)
    assert r.status_code == 400
    r = requests.get(url)
    while r.json()['status'] != 'RUNNING':
        time.sleep(3)
        r = requests.get(url)
        r.raise_for_status()

    r = requests.delete(url)
    r.raise_for_status()
    assert r.json() == {'id': 'sleep-3', 'status': 'CANCELLED', 'log': None}
    while r.json()['status'] == 'CANCELLED':
        time.sleep(3)
        r = requests.get(url)
        r.raise_for_status()

    assert r.json() == {'id': 'sleep-3', 'status': 'FAILED', 'log': None}


def test_delete_batch_not_found():
    r = requests.delete('http://localhost:8998/batch/4')
    assert r.status_code == 404
    assert r.json()['error_message'] == "Batch `4` not found."
