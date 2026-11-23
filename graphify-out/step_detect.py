import json
from pathlib import Path
from graphify.detect import detect
from graphify.paths import out_path
from graphify.watch import _read_build_excludes, _read_build_gitignore

root_dir = Path('.').resolve()
Path('graphify-out').mkdir(parents=True, exist_ok=True)
Path('graphify-out/.graphify_python').write_text(r'C:\Users\ashis\AppData\Local\Programs\Python\Python311\python.exe', encoding='utf-8')
Path('graphify-out/.graphify_root').write_text(str(root_dir), encoding='utf-8')

result = detect(root_dir, extra_excludes=_read_build_excludes(out_path()), gitignore=_read_build_gitignore(out_path()))
Path('graphify-out/.graphify_detect.json').write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding='utf-8')

print(f"Total files detected: {result.get('total_files', 0)}")
for cat, files in result.get('files', {}).items():
    if files:
        print(f"  {cat}: {len(files)} files")
