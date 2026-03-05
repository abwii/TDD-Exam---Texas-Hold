# Texas Hold'em Poker Hand Evaluator (EFREI TDD Exam)

## Projet

Ce projet est la réalisation d'un évaluateur de mains de Poker Texas Hold'em en Rust, développé dans le cadre de l'examen de **Test-Driven Development (TDD)** pour **EFREI**.

L'application est conçue pour :
1. Déterminer la meilleure combinaison de 5 cartes pour chaque joueur à partir d'un pool de 7 cartes (5 cartes communes + 2 cartes privées).
2. Comparer les mains des différents joueurs pour désigner le(s) gagnant(s) (gestion complète des égalités et des pots partagés).
3. Renvoyer la catégorie de la main EXACTE (ex: *Straight Flush*, *Full House*) ainsi que les 5 cartes retenues, triées de manière déterministe.

Ce projet inclut exclusivement la logique de comparaison de mains et de combinatoire (aucune logique de mises/betting).

## Exécution et Tests

Ce projet a été construit suivant un cycle strict de Test-Driven Development (TDD). 
Le cœur du code est accompagné d'une suite exhaustive de tests unitaires couvrant l'ensemble des règles du poker, les classements des mains (`HandCategory`), la génération des 21 combinaisons possibles sur 7 cartes, et la résolution des égalités exactes (jusqu'au 5ème "kicker").

Afin de compiler le code et d'exécuter l'ensemble de la suite de tests unitaires, utilisez l'outil `cargo` inclus dans Rust :

```bash
cargo test
```

Afin de simplement compiler le projet et le vérifier sans lancer les tests :
```bash
cargo build
```

## Règles & Hypothèses Métier Appliquées

*   **Format d'entrée:** Chaque carte doit être explicitement représentée par 2 caractères (ex: `As` pour As de Pique, `Tc` pour 10 de Trèfle, `2h` pour 2 de Cœur). La casse est sensible.
*   **Égalités SANS Couleur:** Les "Suits" (Pique, Cœur, Carreau, Trèfle) n'ont pas de valeur hiérarchique au poker. Elles ne servent jamais à départager une égalité (ex: Une couleur à l'As de Pique et une couleur à l'As de Trèfle résultent en une égalité / split pot).
*   **Quinte "Wheel":** Une quinte (Straight) autorise l'As comme carte la plus faible (A, 2, 3, 4, 5).

## Ordre de Tri Déterministe (Sortie 5 cartes)

Lors du renvoi des 5 cartes exactes choisies dans la structure finale, celles-ci sont ordonnées dynamiquement de façon déterministe par l'algorithme :
1. Par **Fréquence** d'apparition (les Paires/Brelans apparaissent en premier dans le tableau).
2. Par **Rang** descendant (carte la plus forte numériquement en premier).
3. Par **Symbole/Suit** descendant en cas d'apparition de même rang sur les "kickers".

## Auteur
**Wassim BACHA**