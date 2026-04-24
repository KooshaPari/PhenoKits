# Product Requirements Document — Tossy

**Version:** 1.0.0  
**Date:** 2024-03-15  
**Stack:** Rust 2024 edition (core), Python 3.10+ (CLI wrapper)  
**Spec Compliance:** FreeDesktop.org Trash Specification 1.0  
**Primary Commands:** `tossy put`, `tossy list`, `tossy restore`, `tossy rm`, `tossy empty`  

---

## Overview

Tossy is a command-line utility that provides safe, recoverable file deletion by implementing the FreeDesktop.org Trash specification. Instead of permanently deleting files, Tossy moves them to the system trash directory where they can be restored if needed.

This PRD defines the user-facing requirements for Tossy, organized by feature area. Each requirement includes acceptance criteria that define when the feature is complete.

---

## User Stories

### Core Functionality

#### FR-1: Safe File Deletion — `tossy put`

**User Story**: As a user, I want to delete files safely so that I can recover them if I delete something by mistake.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-1.1 | Move files to trash | `tossy put <file>` moves the file to the trash directory instead of deleting it |
| FR-1.2 | Move directories recursively | `tossy put <dir>` moves directories including all contents |
| FR-1.3 | Multiple file support | `tossy put <file1> <file2> <file3>` moves all specified files |
| FR-1.4 | Unique filenames | Files with duplicate names get numeric suffixes (file.txt, file.txt.1, file.txt.2) |
| FR-1.5 | Symlink handling | Symlinks are trashed as symlinks without following the target |
| FR-1.6 | Special files | Device files, sockets, and pipes are handled like regular files |

**Exit Codes**:
- `0`: All files successfully trashed
- `1`: Partial failure (some files trashed, some failed)
- `2`: Complete failure (no files trashed)

#### FR-2: Metadata Recording

**User Story**: As a user, I want metadata recorded so that my files can be restored to their original location.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-2.1 | Create .trashinfo files | Each trashed item has a corresponding `.trashinfo` metadata file |
| FR-2.2 | Record original path | `.trashinfo` contains the original absolute path (percent-encoded) |
| FR-2.3 | Record deletion date | `.trashinfo` contains ISO 8601 formatted deletion timestamp |
| FR-2.4 | Atomic writes | Metadata files are written atomically (temp file + rename) |
| FR-2.5 | Preserve permissions | Original file permissions are recorded in `.trashinfo` |

**Metadata Format**:
```
[Trash Info]
Path=/home/user/documents/report.txt
DeletionDate=2024-03-15T10:30:00
```

#### FR-3: Trash Inspection — `tossy list`

**User Story**: As a user, I want to see what files are in my trash so I can find files to restore.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-3.1 | List all trashed items | `tossy list` shows all items across all trash directories |
| FR-3.2 | Show metadata | Output includes deletion date and original path |
| FR-3.3 | Sort by date | Items sorted oldest first by default |
| FR-3.4 | Path prefix filter | `tossy list /home/user/documents` filters by original path prefix |
| FR-3.5 | Graceful degradation | Malformed `.trashinfo` files show `<unknown>` placeholders |
| FR-3.6 | Streaming output | Output streams as items are found (no buffering) |

**Output Format**:
```
2024-03-15 10:30:00  /home/user/documents/report.txt
2024-03-14 15:22:00  /home/user/downloads/image.png
```

#### FR-4: File Restoration — `tossy restore`

**User Story**: As a user, I want to restore files from trash so I can recover accidentally deleted files.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-4.1 | Interactive selection | `tossy restore` shows numbered list and prompts for selection |
| FR-4.2 | Range selection | Users can specify ranges (1-5) or comma-separated lists (1,3,5) |
| FR-4.3 | Conflict detection | Restore fails if original path is occupied |
| FR-4.4 | Overwrite option | `--overwrite` flag replaces existing files |
| FR-4.5 | Working directory filter | Running without args from a directory shows only items from that path |
| FR-4.6 | Atomic restore | Same-filesystem restores use atomic rename |

**Conflict Handling**:
```
tossy restore: cannot restore '/home/user/documents/report.txt': destination already exists
  Use --overwrite to replace existing file
```

#### FR-5: Selective Deletion — `tossy rm`

**User Story**: As a user, I want to permanently delete specific files from trash so I can free space without emptying everything.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-5.1 | Pattern matching | `tossy rm *.log` matches files by glob pattern |
| FR-5.2 | Multiple patterns | Multiple patterns can be specified |
| FR-5.3 | Dry run option | `--dry-run` shows what would be deleted without deleting |
| FR-5.4 | Strict mode | `--strict` returns error if no files match |
| FR-5.5 | Confirmation | Prompts for confirmation before deletion (unless --force) |
| FR-5.6 | Report count | Reports number of files permanently deleted |

#### FR-6: Empty Trash — `tossy empty`

**User Story**: As a user, I want to permanently delete all files in trash so I can reclaim disk space.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-6.1 | Empty all | `tossy empty` deletes all items from all accessible trash directories |
| FR-6.2 | Age-based cleanup | `tossy empty 7` deletes items older than 7 days |
| FR-6.3 | Orphan cleanup | Removes orphaned files (no `.trashinfo`) and info files (no corresponding file) |
| FR-6.4 | Force option | `--force` skips confirmation prompt |
| FR-6.5 | Size report | Reports total space freed |
| FR-6.6 | Trash-dir option | `--trash-dir <path>` targets specific trash directory |

---

## Cross-Device and Volume Support

### FR-7: Per-Volume Trash

**User Story**: As a user with external drives, I want files on different volumes trashed locally so I don't copy large files across devices.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-7.1 | Per-volume trash | Uses `.Trash-<uid>` or `.Trash/<uid>` on mounted volumes |
| FR-7.2 | Priority order | Checks `.Trash-<uid>` first, then `.Trash/<uid>` |
| FR-7.3 | Fallback to home | Falls back to home trash if per-volume trash unavailable |
| FR-7.4 | Warning on fallback | Warns user when falling back to home trash for cross-device move |
| FR-7.5 | Permission handling | Creates per-volume trash directories with correct permissions |

---

## Error Handling

### FR-8: Robust Error Handling

**User Story**: As a user, I want clear error messages so I understand why an operation failed.

**Requirements**:

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-8.1 | Permission errors | Clear message when lacking write permission on parent directory |
| FR-8.2 | File not found | `cannot trash '<path>': No such file or directory` for missing files |
| FR-8.3 | Disk full handling | Reports when trash cannot be created due to insufficient space |
| FR-8.4 | Read-only filesystem | Detects and reports when target is read-only |
| FR-8.5 | Invalid characters | Handles filenames with special characters correctly |
| FR-8.6 | Path length limits | Handles paths approaching NAME_MAX and PATH_MAX limits |

---

## Non-Goals (Explicitly Out of Scope)

| Item | Reason |
|------|--------|
| GUI or TUI | Pure command-line tool only |
| Secure file shredding | Uses standard filesystem deletion |
| System `rm` replacement | Does not alias or intercept `rm` |
| Windows native trash | Targets Linux/BSD only |
| macOS Time Machine | Native macOS feature, not our domain |
| Cloud storage integration | Future consideration, not in v1 |

---

## Performance Requirements

| Metric | Target | Measurement |
|--------|--------|-------------|
| Single file trash (<100KB) | <50ms | Hyperfine benchmark |
| 100 files trash | <5s | Hyperfine benchmark |
| List 1000 items | <500ms | Time command |
| Memory usage (idle) | <10MB | /usr/bin/time -v |
| Startup time | <20ms | Hyperfine --warmup 0 |

---

## Compatibility Requirements

| Requirement | Description |
|-------------|-------------|
| GNOME Nautilus | Files trashed by Tossy visible in Nautilus |
| KDE Dolphin | Files trashed by Tossy visible in Dolphin |
| XFCE Thunar | Files trashed by Tossy visible in Thunar |
| Other spec-compliant tools | Full interoperability |

---

## Command Interface Summary

| Command | Description | Key Flags |
|---------|-------------|------------|
| `tossy put` | Move files to trash | `-f` force, `-r` recursive (default on) |
| `tossy list` | List trashed items | `--format json\|text`, `--limit N` |
| `tossy restore` | Restore files | `--overwrite`, `--interactive` |
| `tossy rm` | Permanently delete | `--pattern`, `--dry-run`, `--strict` |
| `tossy empty` | Empty trash | `--days N`, `--force`, `--trash-dir` |

---

**Document Status**: Complete  
**Next Review**: Before v1.0 release  
**Owner**: Phenotype Engineering  
