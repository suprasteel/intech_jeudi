
# Exercice avec des complexes

On écrit le code suivant: 

```rust
fn main() {
    let c1 = Complexe::new(1.0, 2.0);
    let c2 = Complexe::new(4.0, 5.0);

    // let c3 = c1 + c2; (1)
    let c3 = Complexe::default(); // (2)

    // println!("{} + {} = {}", c1, c2, c3); (3)
}

#[test]
fn test_complexe_addition () {
    let c1 = Complexe::new(1.0, 2.0);
    let c2 = Complexe::new(4.0, 5.0);
    let c_valid = Complexe::new(5.0, 7.0);
    // assert_eq!(c_valid, c1 + c2); // à laisser commenté pour l'instant
}
```

Objectif de l'exercice:

- remplacer la ligne 2 par la ligne 1 dans main sans avoir d'erreur

## Structure et implémentation de default

1. Ajouter une structure `Complexe`

    - champs ``re: f64`` et ``im: f64``

1. Implémenter un constructeur qui prend en paramètre le réel et l'imaginaire

1. Implémenter le trait défault sur la structure Complexe
    - Une struct dont les types implémentent default peut implémenter default grace à l'annotation derive
    - quelle valeur vaut f64 par défaut ?

Le code doit compiler à ce moment.

## Afficher nos complexes

1. Ajouter le code

    ```rust
    impl Display for Complexe {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}+i{})", self.re, self.im)
        }
    }
    ```
    On doit pouvoir maintenant afficher nos complexes.

1. Décommenter println dans main


## Utiliser `+` avec des Complexes

> le trait qui est demandé à l'usage de `+` est `std::ops::Add`

1. Implémenter le trait `std::ops::Add` sur Complexe
    - ce trait possède un type associé nommé Output.
    - que veut-on comme output pour cette implémentation de Add sur Complexe ?
    - indice `impl Add for Complexe`
    - au besoin, un peu de [lecture](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types).

    > Lvl1

## Créer un module pour `Complexe`

1. Creer un fichier `complexe.rs` au meme niveau que `main.rs`
1. Y placer le code inhérent au complexes (struct, impl et trait impl à couper-coller)
1. Ajouter une ligne `mod complexe;` en haut du fichier main, en dessous des imports
    - A-t-on accès à nos types ?
    - C'est peut etre un problème de visibilité
    - Utiliser le mot clef `pub` selon vos besoins


## Translation

1. Ajouter le test:

    ```rust
    #[test]
    fn test_complexe_translation () {
        let c1 = Complexe::new(1.0, 2.0);
        let translation = Translation::new(4.0, 5.0);
        let c_valid = Complexe::new(5.0, 7.0);
        assert_eq!(c_valid, c1 + translation);
    }
    ```

    On souhaite que translation + complexe retourne un complexe

1. Implémenter `Translation` avec des champs x et y (f64)

1. Implémenter Add<Translation> sur Complexe
    - mais avec pour type associé `Complexe` (retour)
    - indice `impl Add<Translation> for Complexe`

1. Defile en aiguille, corriger les erreurs

1. Implémenter Copy pour Complexe si besoin

---

// LVL 1

YGBgcnVzdA0KdXNlIHN0ZDo6e2ZtdDo6RGlzcGxheSwgb3BzOjpBZGR9Ow0KbW9kIHRyYW5zbGF0aW9uOw0KDQojW2Rlcml2ZShEZWJ1ZywgRGVmYXVsdCwgUGFydGlhbEVxKV0NCnN0cnVjdCBDb21wbGV4ZSB7DQogICAgcmU6IGY2NCwNCiAgICBpbTogZjY0LA0KfQ0KDQppbXBsIENvbXBsZXhlIHsNCiAgICBmbiBuZXcocmVhbDogZjY1LCBpbWFnaW5hcnk6IGY2NCkgLT4gU2VsZiB7DQogICAgICAgIFNlbGYgeyByZTogcmVhbCwgaW06IGltYWdpbmFyeSB9DQogICAgfQ0KfQ0KDQppbXBsIERpc3BsYXkgZm9yIENvbXBsZXhlIHsNCiAgICBmbiBmbXQoJnNlbGYsIGY6ICZtdXQgc3RkOjpmbXQ6OkZvcm1hdHRlcjwnXz4pIC0+IHN0ZDo6Zm10OjpSZXN1bHQgew0KICAgICAgICB3cml0ZSEoZiwgIih7fStpe30pIiwgc2VsZi5yZSwgc2VsZi5pbSkNCiAgICB9DQp9DQoNCmltcGwgQWRkIGZvciBDb21wbGV4ZSB7DQogICAgLy8gYXNzb2NpYXRlZCB0eXBlDQogICAgdHlwZSBPdXRwdXQ9Q29tcGxleGU7IC8vIERlIHF1ZWwgdHlwZSBlc3QgbGUgcmVzdWx0YXQgZCd1biArIHN1ciBkZXV4IGNvbXBsZXhlcyANCg0KICAgIGZuIGFkZChzZWxmLCByaHM6IFNlbGYpIC0+IFNlbGY6Ok91dHB1dCB7DQogICAgICAgIENvbXBsZXhlOjpuZXcoc2VsZi5yZSArIHJocy5yZSwgc2VsZi5pbSArIHJocy5pbSkNCiAgICB9DQp9DQoNCmZuIG1haW4oKSB7DQogICAgbGV0IGMxID0gQ29tcGxleGU6Om5ldygxLjAsIDIuMCk7DQogICAgbGV0IGMyID0gQ29tcGxleGU6Om5ldyg0LjAsIDUuMCk7DQoNCiAgICAvLyBsZXQgYzMgPSBjMSArIGMyOw0KICAgIGxldCBjMyA9IENvbXBsZXhlOjpkZWZhdWx0KCk7DQoNCiAgICBwcmludGxuISgie30gKyB7fSA9IHt9IiwgYzEsIGMyLCBjMyk7IC8vIG9iamVjdGlmOiB1biBjb21wb3J0ZW1lbnQgY29ow6lyZW50DQp9DQoNCiNbdGVzdF0NCmZuIHRlc3RfY29tcGxleGVfYWRkaXRpb24gKCkgew0KICAgIGxldCBjMSA9IENvbXBsZXhlOjpuZXcoMS4wLCAyLjApOw0KICAgIGxldCBjMiA9IENvbXBsZXhlOjpuZXcoNC4wLCA1LjApOw0KICAgIGxldCBjX3ZhbGlkID0gQ29tcGxleGU6Om5ldyg1LjAsIDcuMCk7DQogICAgYXNzZXJ0X2VxIShjX3ZhbGlkLCBjMSArIGMyKTsNCn0NCmBgYA==

---

// LVL 2

dXNlIGNyYXRlOjpjb21wbGV4ZTo6Q29tcGxleGU7Cgptb2QgY29tcGxleGUgewoKICAgIHVzZSBzdGQ6OntmbXQ6OkRpc3BsYXksIG9wczo6QWRkfTsKICAgIHVzZSBzdXBlcjo6dHJhbnNsYXRpb246OlRyYW5zbGF0aW9uOwoKICAgICNbZGVyaXZlKERlYnVnLCBEZWZhdWx0LCBQYXJ0aWFsRXEpXQogICAgcHViIHN0cnVjdCBDb21wbGV4ZSB7CiAgICAgICAgcmU6IGY2NCwKICAgICAgICBpbTogZjY0LAogICAgfQoKICAgIGltcGwgQ29tcGxleGUgewogICAgICAgIHB1YiBmbiBuZXcocmVhbDogZjY0LCBpbWFnaW5hcnk6IGY2NCkgLT4gU2VsZiB7CiAgICAgICAgICAgIFNlbGYgeyByZTogcmVhbCwgaW06IGltYWdpbmFyeSB9CiAgICAgICAgfQogICAgfQoKICAgIGltcGwgRGlzcGxheSBmb3IgQ29tcGxleGUgewogICAgICAgIGZuIGZtdCgmc2VsZiwgZjogJm11dCBzdGQ6OmZtdDo6Rm9ybWF0dGVyPCdfPikgLT4gc3RkOjpmbXQ6OlJlc3VsdCB7CiAgICAgICAgICAgIHdyaXRlIShmLCAiKHt9K2l7fSkiLCBzZWxmLnJlLCBzZWxmLmltKQogICAgICAgIH0KICAgIH0KCiAgICBpbXBsIEFkZCBmb3IgQ29tcGxleGUgewogICAgICAgIC8vIGFzc29jaWF0ZWQgdHlwZQogICAgICAgIHR5cGUgT3V0cHV0PUNvbXBsZXhlOyAvLyBEZSBxdWVsIHR5cGUgZXN0IGxlIHJlc3VsdGF0IGQndW4gKyBzdXIgZGV1eCBjb21wbGV4ZXMgCgogICAgICAgIGZuIGFkZChzZWxmLCByaHM6IFNlbGYpIC0+IFNlbGY6Ok91dHB1dCB7CiAgICAgICAgICAgIENvbXBsZXhlOjpuZXcoc2VsZi5yZSArIHJocy5yZSwgc2VsZi5pbSArIHJocy5pbSkKICAgICAgICB9CiAgICB9CgogICAgaW1wbCBBZGQ8VHJhbnNsYXRpb24+IGZvciBDb21wbGV4ZSB7CiAgICAgICAgLy8gYXNzb2NpYXRlZCB0eXBlCiAgICAgICAgdHlwZSBPdXRwdXQ9Q29tcGxleGU7CiAgICAKICAgICAgICBmbiBhZGQoc2VsZiwgcmhzOiBUcmFuc2xhdGlvbikgLT4gU2VsZjo6T3V0cHV0IHsKICAgICAgICAgICAgQ29tcGxleGU6Om5ldyhzZWxmLnJlICsgcmhzLngsIHNlbGYuaW0gKyByaHMueSkKICAgICAgICB9CiAgICB9Cn0KCm1vZCB0cmFuc2xhdGlvbiB7CgogICAgI1tkZXJpdmUoRGVidWcsIERlZmF1bHQsIFBhcnRpYWxFcSldCiAgICBwdWIoY3JhdGUpIHN0cnVjdCBUcmFuc2xhdGlvbiB7CiAgICAgICAgcHViIHg6IGY2NCwKICAgICAgICBwdWIgeTogZjY0LAogICAgfQoKICAgIGltcGwgVHJhbnNsYXRpb24gewogICAgICAgIHB1YihjcmF0ZSkgZm4gbmV3KHg6IGY2NCwgeTogZjY0KSAtPiBTZWxmIHsKICAgICAgICAgICAgU2VsZiB7IHgsIHkgfQogICAgICAgIH0KICAgIH0KfQoKZm4gbWFpbigpIHsKICAgIGxldCBjMSA9IENvbXBsZXhlOjpuZXcoMS4wLCAyLjApOwogICAgbGV0IGMyID0gQ29tcGxleGU6Om5ldyg0LjAsIDUuMCk7CgogICAgbGV0IGMzID0gYzEgKyBjMjsKICAgIC8vIGxldCBjMyA9IENvbXBsZXhlOjpkZWZhdWx0KCk7CgogICAgcHJpbnRsbiEoInt9ICsge30gPSB7fSIsIGMxLCBjMiwgYzMpOyAvLyBvYmplY3RpZjogdW4gY29tcG9ydGVtZW50IGNvaMOpcmVudAp9CgojW3Rlc3RdCmZuIHRlc3RfY29tcGxleGVfYWRkaXRpb24gKCkgewogICAgbGV0IGMxID0gQ29tcGxleGU6Om5ldygxLjAsIDIuMCk7CiAgICBsZXQgYzIgPSBDb21wbGV4ZTo6bmV3KDQuMCwgNS4wKTsKICAgIGxldCBjX3ZhbGlkID0gQ29tcGxleGU6Om5ldyg1LjAsIDcuMCk7CiAgICBhc3NlcnRfZXEhKGNfdmFsaWQsIGMxICsgYzIpOwp9CgojW3Rlc3RdCmZuIHRlc3RfY29tcGxleGVfdHJhbnNsYXRpb24gKCkgewogICAgdXNlIHRyYW5zbGF0aW9uOjpUcmFuc2xhdGlvbjsKICAgIGxldCBjMSA9IENvbXBsZXhlOjpuZXcoMS4wLCAyLjApOwogICAgbGV0IHRyYW5zbGF0aW9uID0gVHJhbnNsYXRpb246Om5ldyg0LjAsIDUuMCk7CiAgICBsZXQgY192YWxpZCA9IENvbXBsZXhlOjpuZXcoNS4wLCA3LjApOwogICAgYXNzZXJ0X2VxIShjX3ZhbGlkLCBjMSArIHRyYW5zbGF0aW9uKTsKfQ==