#!/usr/bin/env python3
"""
Usage:
    python3 scripts/migrate_variable_module.py <fighter_folder> [<fighter_folder> ...]
    python3 scripts/migrate_variable_module.py --all
"""
import argparse
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
SRC = REPO_ROOT / "src"

KIND_OF_TYPE = {"bool": "FLAG", "i32": "INT", "f32": "FLOAT"}
GET_FN = {"FLAG": "is_flag", "INT": "get_int", "FLOAT": "get_float"}
SET_FN = {"FLAG": "set_flag", "INT": "set_int", "FLOAT": "set_float"}

DECL_RE = re.compile(
    r'^(?P<indent>[ \t]*)(?P<pub>pub\s+)?static mut (?P<name>[A-Za-z_][A-Za-z0-9_]*)'
    r'\s*:\s*\[\s*(?P<ty>bool|i32|f32|u8|u16|u32|u64|i8|i16|i64|usize)\s*;\s*8\s*\]'
    r'\s*=\s*\[[^\]]*\]\s*;\s*$'
)

FN_START_RE = re.compile(
    r'(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+[A-Za-z_][A-Za-z0-9_]*\s*\('
)

ENTRY_ID_DEF_RE = re.compile(
    r'^\s*let\s+(?:mut\s+)?(ENTRY_ID|entry_id)\b[^=]*=\s*WorkModule::get_int\s*\(\s*'
    # Owner-lookup idioms like `WorkModule::get_int(&mut *boma, ...)` or
    # `WorkModule::get_int(&mut *owner_boma, ...)` are extremely common
    # (grab/throw code, weapon-owner lookups via LINK_OWNER, ...) - strip an
    # optional `&mut *`, `&mut `, or `*` prefix and capture just the base
    # identifier, since that's the same underlying accessor either way.
    r'(?:&mut\s*\*\s*|&mut\s+|\*\s*)?([A-Za-z_][A-Za-z0-9_.]*)\s*,\s*\*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID\s*\)'
)

# Parameter patterns used to seed the accessor when ENTRY_ID/entry_id is a
# function parameter rather than derived locally.
PARAM_FIGHTER_RE = re.compile(r'\b([A-Za-z_][A-Za-z0-9_]*)\s*:\s*&mut\s+L2C(?:FighterCommon|AgentBase)\b')
PARAM_BOMA_RE = re.compile(
    r'\b([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(?:&mut|\*mut)\s+(?:smash::app::)?BattleObjectModuleAccessor\b'
)

# All of these need a leading \b: with distinct tracked variables like HOLD
# and IS_HOLD both present, an unanchored `HOLD\[...\]` would also match the
# tail end of `IS_HOLD[...]`. WRITE_RE_TMPL doesn't need one since it's
# anchored to the start of the line via `^\s*`, which already forces an exact
# match (`IS_HOLD[...]` can never satisfy `^\s*HOLD\[` - "IS_" isn't
# whitespace) - but every other template matches anywhere in the line/text.
WRITE_RE_TMPL = r'^(?P<indent>\s*){name}\[(?:ENTRY_ID|entry_id)\]\s*(?P<op>\+=|-=|=)(?!=)\s*(?P<rhs>.*);\s*$'
READ_RE_TMPL = r'\b{name}\[(?:ENTRY_ID|entry_id)\]'
ANY_TOKEN_RE_TMPL = r'\b{name}\b'
INDEXED_ANY_RE_TMPL = r'\b{name}\[[^\]]*\]'


class VarInfo:
    def __init__(self, name, ty, decl_file, decl_line_no):
        self.name = name
        self.ty = ty
        self.kind = KIND_OF_TYPE.get(ty)
        self.decl_file = decl_file
        self.decl_line_no = decl_line_no
        self.id = None
        self.safe = self.ty in KIND_OF_TYPE
        self.skip_reasons = [] if self.safe else [f"unsupported type `{ty}`"]

    def mark_unsafe(self, reason):
        self.safe = False
        self.skip_reasons.append(reason)

    def const_name(self, folder_name):
        return f"FIGHTER_{folder_name.upper()}_INSTANCE_WORK_ID_{self.kind}_{self.name}"


def find_function_spans(text):
    """Return list of (body_start, body_end, header_text) using brace matching."""
    spans = []
    for m in FN_START_RE.finditer(text):
        i = m.end() - 1  # at '('
        depth = 0
        while i < len(text):
            if text[i] == '(':
                depth += 1
            elif text[i] == ')':
                depth -= 1
                if depth == 0:
                    break
            i += 1
        header_end = i + 1
        j = header_end
        while j < len(text) and text[j] != '{':
            if text[j] == ';':  # trait fn decl with no body
                j = -1
                break
            j += 1
        if j == -1 or j >= len(text):
            continue
        brace_start = j
        depth = 0
        k = brace_start
        while k < len(text):
            if text[k] == '{':
                depth += 1
            elif text[k] == '}':
                depth -= 1
                if depth == 0:
                    break
            k += 1
        body_end = k + 1
        spans.append((brace_start, body_end, text[m.start():header_end]))
    return spans


def seed_accessor_from_header(header):
    m = PARAM_FIGHTER_RE.search(header)
    if m:
        return f"{m.group(1)}.module_accessor"
    m = PARAM_BOMA_RE.search(header)
    if m:
        return m.group(1)
    return None


def line_offsets(text):
    offsets = []
    start = 0
    for line in text.splitlines(keepends=True):
        offsets.append((start, start + len(line)))
        start += len(line)
    return offsets


def make_line_no_lookup(text):
    offs = line_offsets(text)

    def line_no_of_offset(off):
        lo, hi = 0, len(offs) - 1
        while lo < hi:
            mid = (lo + hi + 1) // 2
            if offs[mid][0] <= off:
                lo = mid
            else:
                hi = mid - 1
        return lo + 1  # 1-indexed

    return line_no_of_offset


def collect_files(folder):
    return sorted((SRC / folder).rglob("*.rs"))


_MENTION_CACHE = {}


def mentions(name, text):
    """Word-boundary-aware substring check: HOLD must not match inside IS_HOLD."""
    key = name
    pat = _MENTION_CACHE.get(key)
    if pat is None:
        pat = re.compile(r'\b' + re.escape(name) + r'\b')
        _MENTION_CACHE[key] = pat
    return pat.search(text) is not None


def find_declarations(files):
    decls = []
    for f in files:
        text = f.read_text()
        for i, line in enumerate(text.splitlines(), start=1):
            m = DECL_RE.match(line)
            if m:
                decls.append(VarInfo(m.group("name"), m.group("ty"), f, i))
    return decls


def check_bare_usage_safety(decls, files):
    """Any occurrence of a tracked name that isn't NAME[ENTRY_ID]/NAME[entry_id]
    anywhere (foreign indexing, whole-array ops, references, ...) -> unsafe.
    Independent per variable - no shared-line concerns since nothing is written."""
    for var in decls:
        if not var.safe:
            continue
        name = var.name
        any_token_re = re.compile(ANY_TOKEN_RE_TMPL.format(name=re.escape(name)))
        indexed_any_re = re.compile(INDEXED_ANY_RE_TMPL.format(name=re.escape(name)))
        for f in files:
            text = f.read_text()
            for i, raw_line in enumerate(text.splitlines(), start=1):
                if f == var.decl_file and i == var.decl_line_no:
                    continue
                if not any_token_re.search(raw_line):
                    continue
                for idx_m in indexed_any_re.finditer(raw_line):
                    inner = idx_m.group(0)
                    if not re.fullmatch(re.escape(name) + r'\[(?:ENTRY_ID|entry_id)\]', inner):
                        var.mark_unsafe(
                            f"{f.relative_to(REPO_ROOT)}:{i}: indexed by something other than ENTRY_ID/entry_id ({inner})"
                        )
                for tok_m in any_token_re.finditer(raw_line):
                    after = raw_line[tok_m.end():tok_m.end() + 1]
                    if after != '[':
                        var.mark_unsafe(
                            f"{f.relative_to(REPO_ROOT)}:{i}: bare usage not of the form {name}[ENTRY_ID] ({raw_line.strip()})"
                        )


def walk_shared(decls, files, render, folder_name=None):
    """
    Single combined walk over every function body in `files`, considering all
    currently-safe variables in `decls` together (so lines shared by more than
    one tracked variable are handled correctly instead of one clobbering
    another).

    render=False: validation only. Marks variables unsafe as problems are
      found (line has >1 statement, no accessor determinable, compound
      assignment on a flag). Writes nothing.
    render=True: assumes every variable still marked safe is now final.
      Builds replacement lines for those variables' occurrences and writes
      each modified file once. Occurrences belonging to a variable that
      became unsafe (found during the render=False pass) are left as-is,
      even on a line shared with a still-safe variable.
    """
    by_name = {v.name: v for v in decls}
    write_res = {v.name: re.compile(WRITE_RE_TMPL.format(name=re.escape(v.name))) for v in decls}
    read_res = {v.name: re.compile(READ_RE_TMPL.format(name=re.escape(v.name))) for v in decls}

    for f in files:
        text = f.read_text()
        relevant_names = [v.name for v in decls if v.safe and mentions(v.name, text)]
        if not relevant_names:
            continue

        lines = text.splitlines(keepends=True)
        spans = find_function_spans(text)
        line_no_of_offset = make_line_no_lookup(text)
        file_modified = False

        for body_start, body_end, header in spans:
            start_line = line_no_of_offset(body_start)
            end_line = line_no_of_offset(body_end - 1)
            current_accessor = seed_accessor_from_header(header)

            for ln in range(start_line, end_line + 1):
                stripped = lines[ln - 1].rstrip("\n\r")

                acc_m = ENTRY_ID_DEF_RE.match(stripped)
                if acc_m:
                    current_accessor = acc_m.group(2)

                touching = [by_name[n] for n in relevant_names if by_name[n].safe and mentions(n, stripped)]
                if not touching:
                    continue

                write_var, write_match = None, None
                for v in touching:
                    wm = write_res[v.name].match(stripped)
                    if wm:
                        write_var, write_match = v, wm
                        break  # a statement has exactly one assignment target

                if current_accessor is None:
                    if not render:
                        for v in touching:
                            v.mark_unsafe(f"{f.relative_to(REPO_ROOT)}:{ln}: no accessor could be determined ({stripped.strip()})")
                    continue
                acc_expr = f"({current_accessor}) as *mut _"

                if write_var is not None:
                    if stripped.count(';') != 1:
                        if not render:
                            write_var.mark_unsafe(
                                f"{f.relative_to(REPO_ROOT)}:{ln}: more than one statement on the line ({stripped.strip()})"
                            )
                        continue
                    op = write_match.group('op')
                    if op in ('+=', '-=') and write_var.kind == 'FLAG':
                        if not render:
                            write_var.mark_unsafe(f"{f.relative_to(REPO_ROOT)}:{ln}: compound assignment on a flag ({stripped.strip()})")
                        continue
                    if not render:
                        continue  # nothing further to validate for this line
                    if not write_var.safe:
                        continue  # became unsafe elsewhere; leave this line untouched

                    indent = write_match.group('indent')
                    rhs = write_match.group('rhs')
                    # Substitute reads of ANY safe variable within the RHS (self-
                    # referencing, e.g. `N[ID] = N[ID] + 1;`, and cross-variable).
                    for v2 in decls:
                        if not v2.safe:
                            continue
                        rhs = read_res[v2.name].sub(
                            f"VariableModule::{GET_FN[v2.kind]}({acc_expr}, {v2.const_name(folder_name)})", rhs
                        )
                    wname = write_var.const_name(folder_name)
                    if op == '=':
                        new_stmt = f"VariableModule::{SET_FN[write_var.kind]}({acc_expr}, {rhs}, {wname});"
                    elif write_var.kind == 'INT' and rhs.strip() == '1':
                        new_stmt = f"VariableModule::{'inc_int' if op == '+=' else 'dec_int'}({acc_expr}, {wname});"
                    else:
                        sign = '+' if op == '+=' else '-'
                        new_stmt = (
                            f"VariableModule::{SET_FN[write_var.kind]}({acc_expr}, "
                            f"VariableModule::{GET_FN[write_var.kind]}({acc_expr}, {wname}) {sign} ({rhs}), {wname});"
                        )
                    lines[ln - 1] = indent + new_stmt + "\n"
                    file_modified = True
                elif render:
                    new_line = stripped
                    changed = False
                    for v in touching:
                        if not v.safe:
                            continue
                        if read_res[v.name].search(new_line):
                            new_line = read_res[v.name].sub(
                                f"VariableModule::{GET_FN[v.kind]}({acc_expr}, {v.const_name(folder_name)})", new_line
                            )
                            changed = True
                    if changed:
                        lines[ln - 1] = new_line + "\n"
                        file_modified = True

        if render and file_modified:
            f.write_text("".join(lines))


def write_declarations(safe_vars, folder_name):
    by_file = {}
    for v in safe_vars:
        by_file.setdefault(v.decl_file, []).append(v)
    for f, vars_in_file in by_file.items():
        text = f.read_text()
        lines = text.splitlines(keepends=True)
        for v in vars_in_file:
            lines[v.decl_line_no - 1] = f"static {v.const_name(folder_name)} : i32 = {v.id};\n"
        f.write_text("".join(lines))


def migrate_folder(folder_name):
    files = collect_files(folder_name)
    if not files:
        print(f"[{folder_name}] no .rs files found, skipping")
        return
    decls = find_declarations(files)
    if not decls:
        print(f"[{folder_name}] no convertible static mut [T; 8] declarations found")
        return

    check_bare_usage_safety(decls, files)
    walk_shared(decls, files, render=False)

    safe = [v for v in decls if v.safe]
    unsafe = [v for v in decls if not v.safe]
    for i, v in enumerate(safe):
        v.id = i

    # Usage-site edits first (declarations still hold their old array form,
    # which the accessor/pattern regexes don't look at), then declarations.
    walk_shared(decls, files, render=True, folder_name=folder_name)
    write_declarations(safe, folder_name)

    print(f"\n=== {folder_name} ===")
    print(f"Converted {len(safe)}/{len(decls)} variables:")
    for v in safe:
        print(f"  {v.name} ({v.ty}) -> {v.const_name(folder_name)} = {v.id}  [{v.decl_file.relative_to(REPO_ROOT)}:{v.decl_line_no}]")
    if unsafe:
        print(f"Skipped {len(unsafe)} variables (left untouched):")
        for v in unsafe:
            print(f"  {v.name} ({v.ty}) [{v.decl_file.relative_to(REPO_ROOT)}:{v.decl_line_no}]")
            for reason in v.skip_reasons:
                print(f"    - {reason}")


def all_fighter_folders():
    exclude = {
        "state_manager", "variable_module", "s_macros", "config", "config_apply",
        "util", "controls", "common", "cpu", "template",
    }
    return sorted(
        p.name for p in SRC.iterdir()
        if p.is_dir() and p.name not in exclude and not p.name.startswith('.')
    )


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("folders", nargs="*", help="fighter folder names under src/, e.g. miifighter")
    ap.add_argument("--all", action="store_true", help="run on every non-infrastructure folder under src/")
    args = ap.parse_args()

    targets = all_fighter_folders() if args.all else args.folders
    if not targets:
        print("Nothing to do - pass folder names or --all")
        sys.exit(1)

    for folder in targets:
        migrate_folder(folder)


if __name__ == "__main__":
    main()
