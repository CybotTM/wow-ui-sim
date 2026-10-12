import socket
import time

path = "/tmp/.X11-unix/X22"
deadline = time.monotonic() + 10
last_error = None
while time.monotonic() < deadline:
    try:
        with socket.socket(socket.AF_UNIX) as connection:
            connection.settimeout(1)
            connection.connect(path)
        break
    except OSError as error:
        last_error = error
        time.sleep(0.1)
else:
    raise SystemExit(f"Private X display readiness failed: {last_error}")
