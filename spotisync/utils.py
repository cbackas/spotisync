from os import getenv
from datetime import datetime

def current_timestamp():
    return datetime.now().strftime("%Y/%m/%d %H:%M:%S")

def log_error(error_text):
    verbose = getenv('verbose', 'false')
    if (verbose.lower == 'true'):
        log(f'[ERROR] {error_text}')

def log(text):
    print(f'[{current_timestamp()}] {text}')