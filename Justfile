build:
    just update-metadata
    uv build

update-metadata:
    #!/usr/bin/env python
    import json
    import re
    from pathlib import Path

    package_data = json.loads(Path("package.json").read_text())
    version = package_data["version"]

    pyproject_content = Path("pyproject.toml").read_text()

    pyproject_content = re.sub(
        r'version = "[^"]*"',
        f'version = "{version}"',
        pyproject_content
    )

    Path("pyproject.toml").write_text(pyproject_content)

needs-version-update:
    #!/usr/bin/env python
    import json
    import re
    from pathlib import Path

    package_data = json.loads(Path("package.json").read_text())
    version = package_data["version"]

    pyproject_content = Path("pyproject.toml").read_text()

    m = re.search(r'version\s*=\s*"([^"]*)"', pyproject_content)
    current = m.group(1) if m else ""

    print("change" if current != version else "nochange")
