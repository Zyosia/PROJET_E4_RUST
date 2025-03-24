# PROJET E4 RUST

Version 1.0 

Plateforme de compétition de programmes écrit avec le langage Rust appliqué à l'Intelligence Artifficielle pour le jeu BreakThrough.

## Installation
1. Télécharger et décompresser l'archive <span style="color:rgb(38, 87, 178)">PROJET_E4_RUST.zip</span>.

> Assurez vous d'avoir dans votre répertoire <span style="color:rgb(78, 178, 38)">PROJET_E4_RUST</span> les fichiers et dossiers suivant :
>
>```
> 📦PROJET_E4_RUST
>┣ 📂.git
>┣ 📂src
>┣ 📜.gitignore
>┣ 📜Cargo.lock
>┣ 📜Cargo.toml
>┣ 📜LICENSE
>┗ 📜README.md
>```
> Note : <span style="color:rgb(78, 178, 38)">📂.git</span> est un dossier caché, si vous n'avez pas activé l'option "afficher les éléments masqués" ce dossier n'apparaîtra pas.


## Usage

Ouvrez un <span style="color:rgb(38, 87, 178)">Terminal</span> de commande et assurez vous de vous placer dans le répertoire PROJET_E4_RUST. 

> Dans votre terminal si vous entrez les commandes suivantes vous devriez obtenir un affichage similaire :
>
>Linux :
>```bash
> $ ls
>Cargo.lock Cargo.toml LICENSE README.md src/
>```
>Windows :
>```powershell
> PS C:\Chemin_vers_votre_dossier\PROJET_E4_RUST> dir
>
>Répertoire : C:\Chemin_vers_votre_dossier\PROJET_E4_RUST
>
>
>Mode                 LastWriteTime         Length Name
>----                 -------------         ------ ----
>d-----        24/03/2025     02:07                src
>-a----        20/03/2025     01:27            817 .gitignore
>-a----        24/03/2025     03:29          12524 Cargo.lock
>-a----        24/03/2025     03:14            250 Cargo.toml
>-a----        20/03/2025     01:27          35823 LICENSE
>-a----        24/03/2025     20:12            171 README.md
>```

Ensuite, compilez le projet à l'aide de la commande suivante :

```bash
$ cargo b --release
```

Enfin pour exécuter le programme entrez la commande suivante :

```bash
$ cargo r
```

Pour accéder à la documentation du projet vous pouvez utiliser la commande suivante : 

```bash
$ cargo doc --open
```

*Cette commande génère la documentation si elle n'a pas été générée, l'option <span style="color:rgb(152, 178, 38)">--open</span> permet d'ouvrir la page html correspondant à la documentation.*

> Note : Les commandes sont les mêmes peut importe la plateforme (Windows, Linux, MacOS). 

## Features

- Système de création de tournois 
    - Ajout d'IA à un tournoi en écrivant le code directement dans le terminal.
    - Possibilité d'afficher les participants
    - Système de score et Tableau des scores.
- Historique des parties.
    - Replay des parties.
    - Enregistrement des parties.
- Interface Terminale Interactive (Vous pouvez avoir un aperçu [ici](#aperçu))

## Aperçu 

Interface principal :

```bash
🎮 Welcome to the Tournament Manager 🎮
1️⃣  Create a Tournament
2️⃣  Exit
👉 Enter your choice:
```

Interface de gestion de tournois :

```bash
🏆 Tournament Menu 🏆
📌 Status: ⏳ Waiting
1️⃣  Add AI
2️⃣  Show Participants
3️⃣  Start Tournament
4️⃣  Display Scoreboard
5️⃣  Run Tournament
6️⃣  Replay Game
7️⃣  End Tournament
👉 Enter your choice:
```


Exemple de tableau des scores :

```bash
🏆 Résultats du Tournois 🏆
1. ia_test - 2 points
2. ai_random2 - 1 points
3. ai_random - 0 points
🔄 Press Enter to continue...
```