import time
import requests
from main import FOUR_HUNDRED_ERROR, Status


def test_post_batch():
    r = requests.post('http://localhost:8998/batch/1', json={'args': ['ls', '-lthra']})
    r.raise_for_status()
    assert r.json() == {'id': '1', 'status': 'PENDING', 'log': None}


def test_post_bad_batch_raises():
    r = requests.post('http://localhost:8998/batch/3', data={'args': ['ls', '-lthra']})
    assert r.json() == FOUR_HUNDRED_ERROR
    r = requests.post('http://localhost:8998/batch/3', json={'not_args': ['ls', '-lthra']})
    assert r.json() == FOUR_HUNDRED_ERROR


def test_double_post_batch_raises():
    headers = {'content-type': 'application/json'}
    requests.post('http://localhost:8998/batch/2', json={'args': ['ls', '-lthra']}, headers=headers)
    r = requests.post('http://localhost:8998/batch/2', json={'args': ['ls', '-lthra']}, headers=headers)
    assert r.status_code == 400
    assert r.json()['error_message'] == "Batch `2` already exists."


def test_get_batch():
    r = requests.get('http://localhost:8998/batch/1')
    while r.json()['status'] in (str(Status.RUNNING), str(Status.PENDING)):
        time.sleep(2)
        r = requests.get('http://localhost:8998/batch/1')

    assert r.json() == {'id': '1', 'status': 'SUCCEEDED', 'log': None}


def test_get_batch_not_found():
    r = requests.get('http://localhost:8998/batch/4')
    assert r.status_code == 404
    assert r.json()['error_message'] == "Batch `4` not found."
