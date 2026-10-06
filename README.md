# c-nano

L'éditeur C de la piscine — un IDE TUI sobre et rapide, thème « Minuit »
profond, fenêtres arrondies façon lazy.nvim.

## Build (Windows / Linux / macOS)

```sh
cargo build --release
```

Le binaire est `target/release/c-nano` (`c-nano.exe` sous Windows).

Prérequis : Rust stable (`rustup`). Aucune autre dépendance système —
le thème et les syntaxes sont embarqués. Optionnels mais recommandés :
`git` (branche + marqueurs dans l'explorateur) et `gcc` (compilation ^B).

## Gestes

| Touche | Action |
|---|---|
| `c-nano .` | ouvre l'explorateur sur le dossier |
| `Alt+Tab` / `Ctrl+Tab` / `Shift+Tab` | change de panneau |
| `^O` | recherche de fichiers flottante (filtre flou) |
| `^T` | explorateur |
| `^B` | compiler (gcc), erreurs en marge, survol façon LSP |
| `^N` / `^P` | diagnostic suivant / précédent |
| `^S` / `^Q` | sauvegarder / quitter |
| `^F` `^R` `^G` | chercher · remplacer · aller à la ligne |
| `^K` `^U` `^Z` | couper · coller · annuler |
| `Tab` | 4 espaces (la Norme) |

Auto-paires, règle colonne 80, vérification de norme à la sauvegarde,
notifications toast, statusline segmentée. Zéro configuration.
