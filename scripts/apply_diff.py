import os
import shutil
import tkinter as tk
from tkinter import filedialog

def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    removed_txt = os.path.join(script_dir, "removed_files.txt")
    diff_mods_dir = os.path.join(script_dir, "ultimate", "mods")

    print("Please select your 'mods' folder in the dialog window...")
    root = tk.Tk()
    root.withdraw()
    mods_dir = filedialog.askdirectory(
        title="Select your 'mods' folder (the one containing 'Ultimate S Arcropolis')"
    )
    root.destroy()

    if not mods_dir:
        print("No folder selected. Exiting.")
        input("\nPress Enter to exit.")
        return

    mods_dir = os.path.normpath(mods_dir)
    print(f"Mods folder: {mods_dir}")
    print()

    removed = 0
    removed_missing = 0
    copied = 0
    errors = []

    if os.path.exists(removed_txt):
        print("--- Removing deleted files ---")
        with open(removed_txt, 'r') as f:
            lines = [l.strip() for l in f.readlines() if l.strip()]

        for line in lines:
            norm = line.replace('\\', '/')
            marker = "ultimate/mods/"
            idx = norm.lower().find(marker)
            relative = norm[idx + len(marker):] if idx != -1 else norm

            target = os.path.normpath(os.path.join(mods_dir, relative))
            if os.path.isfile(target):
                try:
                    os.remove(target)
                    print(f"  Removed: {relative}")
                    removed += 1
                except Exception as e:
                    print(f"  ERROR removing {relative}: {e}")
                    errors.append(str(e))
            else:
                print(f"  Already gone: {relative}")
                removed_missing += 1
    else:
        print("No removed_files.txt found, skipping removal step.")

    print()

    if os.path.exists(diff_mods_dir):
        print("--- Copying updated files ---")
        for root_dir, dirs, files in os.walk(diff_mods_dir):
            for filename in files:
                src = os.path.join(root_dir, filename)
                relative = os.path.relpath(src, diff_mods_dir)
                dst = os.path.normpath(os.path.join(mods_dir, relative))

                os.makedirs(os.path.dirname(dst), exist_ok=True)
                try:
                    shutil.copy2(src, dst)
                    print(f"  Copied: {relative}")
                    copied += 1
                except Exception as e:
                    print(f"  ERROR copying {relative}: {e}")
                    errors.append(str(e))
    else:
        print(f"ERROR: Could not find diff files at {diff_mods_dir}")

    print()
    print("=== Done! ===")
    print(f"  Files removed:        {removed}")
    print(f"  Already gone:         {removed_missing}")
    print(f"  Files copied/updated: {copied}")
    if errors:
        print(f"  Errors:               {len(errors)}")
        for e in errors:
            print(f"    - {e}")

    input("\nPress Enter to exit.")

if __name__ == "__main__":
    main()