use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phenotype_skills::{
    Skill, SkillManifest, Runtime, SkillDependency,
    SkillRegistry,
    DependencyResolver,
};

fn create_test_manifest(name: &str) -> SkillManifest {
    SkillManifest::new(name, "1.0.0", Runtime::Wasm, "main.wasm")
}

fn bench_skill_creation(c: &mut Criterion) {
    let manifest = create_test_manifest("bench-skill");
    
    c.bench_function("skill_creation", |b| {
        b.iter(|| {
            let skill = Skill::from_manifest(black_box(manifest.clone()), None);
            black_box(skill);
        })
    });
}

fn bench_registry_registration(c: &mut Criterion) {
    c.bench_function("registry_registration", |b| {
        b.iter(|| {
            let mut reg = SkillRegistry::new();
            let manifest = create_test_manifest("reg-skill");
            let skill = Skill::from_manifest(manifest, None);
            reg.register(black_box(skill)).unwrap();
        })
    });
}

fn bench_registry_lookup(c: &mut Criterion) {
    let mut registry = SkillRegistry::new();
    let manifest = create_test_manifest("lookup-skill");
    let skill = Skill::from_manifest(manifest, None);
    let skill_id = skill.id.clone();
    registry.register(skill).unwrap();
    
    c.bench_function("registry_lookup", |b| {
        b.iter(|| {
            let result = registry.get(black_box(&skill_id));
            black_box(result);
        })
    });
}

fn bench_dependency_resolution(c: &mut Criterion) {
    let mut resolver = DependencyResolver::new();
    
    // Register 100 available dependencies
    for i in 0..100 {
        let dep = create_test_manifest(&format!("dep-{}", i));
        resolver.register_available(dep);
    }
    
    let mut manifest = SkillManifest::new("test", "1.0.0", Runtime::Wasm, "main.wasm");
    manifest.dependencies.push(SkillDependency {
        name: "dep-50".to_string(),
        version_constraint: ">=1.0.0".to_string(),
    });
    
    c.bench_function("dependency_resolution", |b| {
        b.iter(|| {
            let result = resolver.resolve(black_box(&manifest));
            black_box(result);
        })
    });
}

criterion_group!(
    benches,
    bench_skill_creation,
    bench_registry_registration,
    bench_registry_lookup,
    bench_dependency_resolution
);
criterion_main!(benches);
