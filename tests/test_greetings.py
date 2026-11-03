"""Exercise the built Windows app's dictionary paths, with no model/runtime present."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


def check(executable):
    fixtures = json.loads(Path(__file__).with_name("greeting_dictionary.json").read_text(encoding="utf-8"))
    # Isolate the executable so accidental model use fails instead of masking a miss.
    with tempfile.TemporaryDirectory(prefix="lugha-greetings-") as directory:
        exe = Path(directory) / "lugha-desk.exe"
        shutil.copy2(executable, exe)
        cases = []
        for row in fixtures:
            source = row["source"]
            for variant in (source, source + ".", "  " + source + "?!  ",
                            source.lower(), source.upper(), source.replace(" ", "\t  ")):
                cases.append((row["direction"], variant, row["target"]))
        misses = [
            ("en-ln", "Good morning."), ("en-wo", "Good morning, Mary."),
            ("wo-en", "Nanga def? Hello."), ("en-lg", "She said good morning."),
            ("lg-en", "Wasuze otya, Rehema?"), ("en-yo", "Good\nmorning"),
            ("yo-en", "Ẹ káàárọ̀, bàbá."), ("en-wo", "Good afternoon."),
            ("en-yo", "Not a good morning."), ("en-ha", "Good morning."),
        ]
        cases.extend((direction, source, None) for direction, source in misses)
        wire = "".join(json.dumps({"direction": d, "text": s}) + "\n" for d, s, _ in cases)
        result = subprocess.run([str(exe), "--session-worker"], input=wire.encode("utf-8"),
                                capture_output=True, timeout=60, check=True)
        messages = [json.loads(line) for line in result.stdout.decode("utf-8").splitlines()]
        responses = [m for m in messages if "result" in m or "error" in m]
        assert len(responses) == len(cases), (len(responses), len(cases))
        for (direction, source, target), response in zip(cases, responses):
            if target is None:
                assert "runtime is missing" in response.get("error", ""), (direction, source, response)
            else:
                assert response.get("result") == target, (direction, source, target, response)
        # --translate uses the same worker::run entry point as the desktop UI.
        for row in fixtures:
            for mode in ("--translate", "--native-worker"):
                result = subprocess.run([str(exe), mode, row["direction"]],
                                        input=row["source"].encode("utf-8"),
                                        capture_output=True, timeout=10, check=True)
                assert result.stdout.decode("utf-8") == row["target"], (mode, row, result.stdout)
        print(f"Greeting integration: {len(cases)} session checks and {2 * len(fixtures)} entry-point checks passed.")


if __name__ == "__main__":
    check(Path(sys.argv[1]).resolve(strict=True))
