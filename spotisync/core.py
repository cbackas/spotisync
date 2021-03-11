import time
from utils import log
from auth import authenticate
from spotify import perform_sync

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
        log(f'Not authenticated. Starting auth loop again.')