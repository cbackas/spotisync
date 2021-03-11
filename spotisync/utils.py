from datetime import datetime

def current_timestamp():
    return datetime.now().strftime("%Y/%m/%d %H:%M:%S")

def log_error(error_text):
    log(f'[ERROR] {error_text}')

def log(text):
    print(f'[{current_timestamp()}] {text}')