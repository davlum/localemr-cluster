import os
import sys
import time
import argparse
from typing import List, Optional
from enum import Enum
import subprocess
from multiprocessing import Queue, Process
from bottle import request, response, post, run, get


class Status(Enum):
    PENDING = 'PENDING'
    RUNNING = 'RUNNING'
    FAILED = 'FAILED'
    SUCCEEDED = 'SUCCEEDED'

    def __str__(self):
        return str(self.value)


class Result:
    def __init__(self, batch_id: str, status: Status, log: Optional[str] = None):
        self.id = batch_id
        self.status = status
        self.log = log

    def to_dict(self):
        return {
            'id': self.id,
            'status': str(self.status),
            'log': self.log,
        }


class Proc:
    def __init__(self, batch_id: str, cli_args: List[str]):
        self.batch_id = batch_id
        self.cli_args = cli_args


FOUR_HUNDRED_ERROR = {
    'error': 'ValidationError',
    'error_message': 'Must post request body of form {"args":[...]}',
}


@post('/batch/<batch_id:path>')
def post_batch(batch_id):
    global step_dict

    if not request.json:
        response.status = 400
        return FOUR_HUNDRED_ERROR
    cli_args = request.json.get('args')
    if not cli_args or not isinstance(cli_args, list):
        response.status = 400
        return FOUR_HUNDRED_ERROR
    if batch_id in step_dict:
        response.status = 400
        return {
            'error': 'ValidationError',
            'error_message': f"Batch `{batch_id}` already exists."
        }
    result = Result(batch_id, Status.PENDING)
    step_dict[batch_id] = result
    process_queue.put(Proc(batch_id, cli_args))
    return result.to_dict()


@get('/batch/<batch_id:path>')
def get_batch(batch_id):
    global step_dict

    while not status_queue.empty():
        status: Result = status_queue.get()
        step_dict[status.id] = status 
    maybe_step = step_dict.get(batch_id)
    if maybe_step:
        return maybe_step.to_dict()

    response.status = 404
    return {
        'error': 'NotFound',
        'error_message': f"Batch `{batch_id}` not found."
    }


@get('/health')
def get_health():
    return {"status": "OK"}


def poll_step(process_q: Queue, status_q: Queue):
    while True:
        if process_q.empty():
            time.sleep(5)
        else:
            step: Proc = process_q.get()
            status_q.put(Result(step.batch_id, Status.RUNNING))
            result = run_step(step.batch_id, step.cli_args)
            status_q.put(result)


def run_step(batch_id, cli_args):
    base_log_dir = f'/var/log/steps/{batch_id}'
    os.makedirs(base_log_dir, exist_ok=True)
    stderr_path = f'{base_log_dir}/stderr.log'
    stdout_path = f'{base_log_dir}/stdout.log'
    with open(stderr_path, 'w') as stderr, open(stdout_path, 'w') as stdout, \
            subprocess.Popen(cli_args, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             universal_newlines=True, env=os.environ) as proc:

        for line in proc.stdout:
            sys.stdout.write(line)
            stdout.write(line)
        for line in proc.stderr:
            sys.stderr.write(line)
            stderr.write(line)

    return_code = proc.wait()
    with open(stderr_path) as f:
        status = Status.FAILED if return_code > 0 else Status.SUCCEEDED
        return Result(batch_id, status, f.read() or None)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="A container that dangerously batches arbitrary commands")
    parser.add_argument('-p', '--port', help='Port to run the service on', type=int, default=8998)
    parser.add_argument("-H", "--host", type=str, help="Which host to bind", default="0.0.0.0")
    args = parser.parse_args()
    step_dict = {}
    process_queue = Queue()
    status_queue = Queue()
    step_runner_process = Process(target=poll_step, args=(process_queue, status_queue))
    step_runner_process.daemon = True
    step_runner_process.start()
    run(host=args.host, port=args.port)
