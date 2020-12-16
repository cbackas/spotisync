import time
from auth import authenticate
from spotify import perform_sync

sp = None
while sp == None:
    cachedAuth = authenticate()
    if cachedAuth != None:
        sp = cachedAuth
        
        # do the sync every 60 seconds
        while True:
            perform_sync(sp)
            time.sleep(60)
    else:
        print("NO AUTH?!")