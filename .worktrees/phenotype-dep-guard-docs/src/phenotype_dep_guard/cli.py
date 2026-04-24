import click

from phenotype_dep_guard.agent import AgenticAnalyzer
from phenotype_dep_guard.console import console
from phenotype_dep_guard.resolver import DependencyResolver
from phenotype_dep_guard.triage import TriageEngine


@click.group()
def main():
    """Phenotype Dependency Guard - Malicious code detection in dependencies."""
    pass

@main.command()
@click.argument('path', type=click.Path(exists=True))
def scan(path):
    """Scan a project for malicious dependencies."""
    console.print(f"Scanning project at: {path}")
    
    resolver = DependencyResolver(path)
    triage = TriageEngine()
    agent = AgenticAnalyzer()

    # 1. Dependency Resolution
    with console.status("Resolving dependencies..."):
        deps = resolver.get_all_dependencies()
    console.print(f"Found {len(deps)} dependencies.")

    # 2. Heuristic Triage
    findings_count = 0
    for dep in deps:
        # For simulation, we scan the project directory itself as if it were a dependency
        dep_path = path # Use current path for testing triage logic
        with console.status(f"Triaging {dep['name']}..."):
            findings = triage.triage_dependency(dep_path)
        
        if findings:
            findings_count += len(findings)
            console.print(f"Found {len(findings)} suspicious patterns in {dep['name']}")
            
            # 3. Agentic Deep Analysis for high severity
            high_severity = [f for f in findings if f['severity'] == 'high']
            if high_severity:
                with console.status(f"Invoking Agentic Deep Dive for {dep['name']}..."):
                    analysis = agent.analyze_dependency(dep['name'], findings, dep_path)
                
                console.print(f"Agent Analysis Result ({dep['name']}): {analysis['status']}")
                console.print(f"  Reasoning: {analysis.get('reasoning')}")
                console.print(f"  Confidence: {analysis.get('confidence')}")
                console.print(f"  Action: {analysis.get('action')}")

    if findings_count == 0:
        console.print("No suspicious patterns detected.")
    else:
        console.print(f"\nTotal findings: {findings_count}")

if __name__ == "__main__":
    main()
