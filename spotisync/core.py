import time
from auth import authenticate
from spotify import perform_sync

sync_interval = 2.5 # minutes
sp = None
while sp == None:
    cachedAuth = authenticate()
    if cachedAuth != None:
        sp = cachedAuth
        
        while True:
            perform_sync(sp)
            time.sleep(sync_interval * 60)
    else:
        print("NO AUTH?!")