import subprocess
import sys

if __name__ == "__main__":
    # Run the FastAPI app with Gunicorn
    cmd = [
        "gunicorn",
        "--worker-class", "uvicorn.workers.UvicornWorker",
        "--workers", "4",
        "--bind", "0.0.0.0:8080",
        "--timeout", "120",
        "app:app"
    ]

    try:
        subprocess.run(cmd, check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error starting Gunicorn server: {e}")
        sys.exit(1)
    except KeyboardInterrupt:
        print("\nShutting down server...")
        sys.exit(0)
