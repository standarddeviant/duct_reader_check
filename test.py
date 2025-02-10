
import logging
import sys
import time

logging.basicConfig(format='%(asctime)s %(message)s', level=logging.INFO)


time.sleep(2.0)
logging.info("hello to stdout AFTER init-sleep")

while True:
    tmp = sys.stdin.read()
    if len(tmp) > 0:
        break;
logging.info("hello to stdout AFTER non-zero-length stdin.read()")

time.sleep(2.0)
logging.info("hello to stdout AFTER post-read-sleep")

sys.stdout.flush()
sys.exit(0)

