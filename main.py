import os
import time
import argparse
import signal
from typing import List, Optional
from enum import Enum
import subprocess
from multiprocessing import Queue, Process
from bottle import request, response, post, run, get, delete


class Status(Enum):
    PENDING = 'PENDING'
    RUNNING = 'RUNNING'
    FAILED = 'FAILED'
    SUCCEEDED = 'SUCCEEDED'
    CANCELLED = 'CANCELLED'

    def __str__(self):
        return str(self.value)


class Batch:
    def __init__(self, batch_id: str, status: Status, cli_args: List[str], pid: Optional[int] = None, log: Optional[str] = None):
        self.id = batch_id
        self.cli_args = cli_args
        self.status = status
        self.log = log
        self.pid = pid

    def to_dict(self):
        return {
            'id': self.id,
            'status': str(self.status),
            'log': self.log,
        }


FOUR_HUNDRED_ERROR = {
    'error': 'ValidationError',
    'error_message': 'Must post request body of form {"args":[...]}',
}


@post('/batch/<batch_id:path>')
def post_batch(batch_id):
    global batch_dict

    if not request.json:
        response.status = 400
        return FOUR_HUNDRED_ERROR
    cli_args = request.json.get('args')
    if not cli_args or not isinstance(cli_args, list):
        response.status = 400
        return FOUR_HUNDRED_ERROR
    if batch_id in batch_dict:
        response.status = 400
        return {
            'error': 'ValidationError',
            'error_message': f"Batch `{batch_id}` already exists."
        }
    batch = Batch(batch_id, Status.PENDING, cli_args)
    batch_dict[batch_id] = batch
    process_queue.put(batch)
    return batch.to_dict()


def update_batch_dict():
    global batch_dict

    while not status_queue.empty():
        status: Batch = status_queue.get()
        batch_dict[status.id] = status


@get('/batch/<batch_id:path>')
def get_batch(batch_id):
    update_batch_dict()
    maybe_batch = batch_dict.get(batch_id)
    if maybe_batch:
        return maybe_batch.to_dict()

    response.status = 404
    return {
        'error': 'NotFound',
        'error_message': f"Batch `{batch_id}` not found."
    }


@delete('/batch/<batch_id:path>')
def delete_batch(batch_id):
    global batch_dict

    update_batch_dict()
    maybe_batch: Batch = batch_dict.get(batch_id)
    if not maybe_batch:
        response.status = 404
        return {
            'error': 'NotFound',
            'error_message': f"Batch `{batch_id}` not found."
        }

    if maybe_batch.status == Status.PENDING:
        response.status = 400
        return {
            'error': 'ValueError',
            'error_message': f"Batch `{batch_id}` still pending, unable to delete."
        }

    os.kill(maybe_batch.pid, signal.SIGKILL)
    maybe_batch.status = Status.CANCELLED
    batch_dict[batch_id] = maybe_batch
    return maybe_batch.to_dict()


@get('/health')
def get_health():
    return {"status": "OK"}


def poll_batch(process_q: Queue, status_q: Queue):
    while True:
        if process_q.empty():
            time.sleep(5)
        else:
            batch: Batch = process_q.get()
            result = run_batch(status_queue, batch)
            status_q.put(result)


def run_batch(status_q: Queue, batch: Batch):
    base_log_dir = f'/var/log/steps/{batch.id}'
    os.makedirs(base_log_dir, exist_ok=True)
    stderr_path = f'{base_log_dir}/stderr.log'
    stdout_path = f'{base_log_dir}/stdout.log'
    with open(stderr_path, 'w') as stderr, open(stdout_path, 'w') as stdout, \
            subprocess.Popen(batch.cli_args, stdout=stdout, stderr=stderr, env=os.environ, bufsize=8192) as proc:

        batch.status = Status.RUNNING
        batch.pid = proc.pid
        status_q.put(batch)

    return_code = proc.wait()
    with open(stderr_path) as f:
        batch.status = Status.SUCCEEDED if return_code == 0 else Status.FAILED
        batch.log = f.read() or None
        return batch


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="A container that dangerously batches arbitrary commands")
    parser.add_argument('-p', '--port', help='Port to run the service on', type=int, default=8998)
    parser.add_argument("-H", "--host", type=str, help="Which host to bind", default="0.0.0.0")
    args = parser.parse_args()
    batch_dict = {}
    process_queue = Queue()
    status_queue = Queue()
    batch_runner_process = Process(target=poll_batch, args=(process_queue, status_queue))
    batch_runner_process.daemon = True
    batch_runner_process.start()
    run(host=args.host, port=args.port)
