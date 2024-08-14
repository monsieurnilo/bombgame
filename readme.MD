# Bombgame

## Auteurs
- Raphaël PAVARD
- Teddy VICTORIEN DIT RICHARD

## Introduction

Ce projet est un jeu en ligne de commande qui se déroule sur une carte où le joueur peut se déplacer, éviter des bombes et gagner via une porte de sortie.

### Lancer le jeu

Assurez-vous d'avoir Rust installé.
Dans un terminal à la racine du jeu :
- cargo build
- cargo run

### Dépendances utilisées

- crossterm : Pour la gestion du terminal.
- rand : Pour la génération de la carte.

## Fonctionnement

### Fichiers
- main.rs : Point d'entrée du programme. Initialise le terminal et lance le jeu.
- game.rs : Contient la boucle principale du jeu, gère les événements de clavier et les interactions entre le joueur et la carte.
- player.rs : Définit la structure et les comportements du joueur, y compris les déplacements et le dessin du joueur sur la carte.
- map.rs : Gère la carte du jeu, y compris le chargement depuis un fichier, le dessin de la carte, et la gestion des éléments comme les murs, les portes et les bombes.

### Initialisation du programme

Le programme commence dans main.rs où le terminal est configuré.
La fonction run_game de game.rs est appelée pour démarrer la boucle de jeu.

#### Génération de la carte

La carte est chargée depuis un fichier texte spécifié (assets/maps/beach_map.txt) dans game.rs.
Des bombes sont placées aléatoirement sur la carte sur le caractère "░".

#### Génération du joueur

Le joueur est initialisé à une position aléatoire sur le caractère "▓" sur la carte.
Le joueur ne peut pas traverser les murs et il est renvoyé à une position aléatoire sur le caractère "▓" s'il marche sur une bombe.

#### Victoire du joueur

Le jeu peut être remporté si le joueur atteint la fin de la carte en marchant sur le caractère "█".

### Contrôles en jeu

Les touches de direction (haut, bas, gauche, droite) permettent de déplacer le joueur.
Sinon appuyez sur 'q' pour quitter le jeu.

