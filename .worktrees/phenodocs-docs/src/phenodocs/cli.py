#!/usr/bin/env python3
"""PhenoDocs CLI - Documentation generation and management system."""

import argparse
import subprocess
import sys
from pathlib import Path

def run_spec_populator(args):
    """Run spec populator for a repository."""
    # Tools are in repos root, phenodocs is a subdirectory
    tools_path = Path(__file__).parent.parent.parent.parent / "tools" / "docgen" / "spec_populator.py"
    cmd = [
        "python3",
        str(tools_path),
        "--repo", args.repo,
        "--name", args.name or args.repo
    ]
    if args.annotate:
        cmd.append("--annotate")
    result = subprocess.run(cmd, capture_output=True, text=True)
    print(result.stdout)
    if result.stderr:
        print(result.stderr, file=sys.stderr)
    return result.returncode

def run_validate_coverage(args):
    """Run traceability validation."""
    tools_path = Path(__file__).parent.parent.parent.parent / "tools" / "docgen" / "validate_coverage.py"
    cmd = [
        "python3", 
        str(tools_path),
        args.repo
    ]
    if args.format:
        cmd.extend(["--format", args.format])
    result = subprocess.run(cmd, capture_output=True, text=True)
    print(result.stdout)
    return result.returncode

def run_gif_recording(args):
    """Record E2E GIF for user journeys."""
    repo_path = Path.cwd().parent / args.repo
    if not repo_path.exists():
        repo_path = Path.cwd() / args.repo
    
    print(f"🎬 Recording E2E GIFs for {args.repo}...")
    print(f"Repository path: {repo_path}")
    print("\nTo record a journey:")
    print(f"  cd {repo_path}")
    print(f"  asciinema rec docs/journeys/{args.journey}.cast")
    print(f"  # Then convert: agg docs/journeys/{args.journey}.cast docs/journeys/{args.journey}.gif")
    return 0

def run_full_pipeline(args):
    """Run full documentation generation pipeline."""
    print(f"🚀 Running full pipeline for {args.repo}...")
    
    # Step 1: Generate specs
    print("\n📋 Step 1: Generating specs...")
    result = run_spec_populator(args)
    if result != 0:
        return result
    
    # Step 2: Validate coverage
    print("\n✅ Step 2: Validating traceability...")
    result = run_validate_coverage(args)
    
    print("\n✨ Pipeline complete!")
    return result


def run_setup():
    """Install dependencies for documentation generation."""
    print("🔧 Setting up PhenoDocs dependencies...")
    
    deps = ["asciinema", "agg", "gifski"]
    missing = []
    
    for dep in deps:
        result = subprocess.run(["which", dep], capture_output=True)
        if result.returncode != 0:
            missing.append(dep)
    
    if missing:
        print(f"Missing dependencies: {', '.join(missing)}")
        print("\nInstall with:")
        if "asciinema" in missing:
            print("  pip install asciinema")
        if "agg" in missing or "gifski" in missing:
            print("  cargo install agg gifski")
    else:
        print("✅ All dependencies installed!")
    
    return 0


def run_list_specs(args):
    """List all generated specs for a repository."""
    repo_path = Path.cwd().parent / args.repo
    if not repo_path.exists():
        repo_path = Path.cwd() / args.repo
    
    specs_dir = repo_path / ".agileplus" / "specs" / "functional-requirements"
    
    if not specs_dir.exists():
        print(f"⚠️  No specs found for {args.repo}")
        print(f"Run: phenodocs spec --repo {args.repo}")
        return 1
    
    specs = list(specs_dir.glob("fr_*.yaml"))
    print(f"📋 Found {len(specs)} specs for {args.repo}:\n")
    
    for spec in sorted(specs):
        print(f"  • {spec.stem}")
    
    return 0

def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        prog="phenodocs",
        description="PhenoDocs - Documentation generation and management system"
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # Spec command
    spec_parser = subparsers.add_parser("spec", help="Generate AgilePlus specs")
    spec_parser.add_argument("--repo", required=True, help="Repository name")
    spec_parser.add_argument("--name", help="Spec name (defaults to repo)")
    spec_parser.add_argument("--annotate", action="store_true", help="Annotate code with FR-XXX")
    
    # Validate command
    validate_parser = subparsers.add_parser("validate", help="Validate traceability coverage")
    validate_parser.add_argument("--repo", required=True, help="Repository name")
    validate_parser.add_argument("--format", choices=["terminal", "json", "markdown"], 
                                  default="terminal", help="Output format")
    
    # GIF command
    gif_parser = subparsers.add_parser("gif", help="Record E2E GIFs")
    gif_parser.add_argument("--repo", required=True, help="Repository name")
    gif_parser.add_argument("--journey", default="onboarding", help="Journey name")
    
    # Full pipeline command
    # Full pipeline command
    full_parser = subparsers.add_parser("full", help="Run full documentation pipeline")
    full_parser.add_argument("--repo", required=True, help="Repository name")
    full_parser.add_argument("--name", help="Spec name (defaults to repo)")

    # Setup command
    setup_parser = subparsers.add_parser("setup", help="Install dependencies")

    # List command
    list_parser = subparsers.add_parser("list", help="List generated specs")
    list_parser.add_argument("--repo", required=True, help="Repository name")

    args = parser.parse_args()

    if args.command == "spec":
        return run_spec_populator(args)
    elif args.command == "validate":
        return run_validate_coverage(args)
    elif args.command == "gif":
        return run_gif_recording(args)
    elif args.command == "full":
        return run_full_pipeline(args)
    elif args.command == "setup":
        return run_setup()
    elif args.command == "list":
        return run_list_specs(args)
    else:
        parser.print_help()
        return 0
if __name__ == "__main__":
    sys.exit(main())
