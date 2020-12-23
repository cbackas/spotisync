import time
from datetime import datetime
from auth import authenticate
from spotify import perform_sync

def current_timestamp():
    return datetime.now().strftime("%Y/%m/%d %H:%M:%S")

sync_interval = 2.5 # minutes
sp = None
while sp == None: # auth loop
    cachedAuth = authenticate()
    if cachedAuth != None:
        sp = cachedAuth
        
        while True: # sync loop
            perform_sync(sp)
            time.sleep(sync_interval * 60)
    else:
        print(f'[{current_timestamp()}] Not authenticated. Starting auth loop again.')