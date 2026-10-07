//! c-nano — éditeur de code TUI, thème Minuit, fenêtres arrondies.

use std::path::PathBuf;

fn main() {
    let file = std::env::args().nth(1).map(PathBuf::from);
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some("-h") | Some("--help") | Some("-V") | Some("--version")
    ) {
        println!(
            "c-nano — l'éditeur C de la piscine\n\n\
             Usage: c-nano [fichier]\n\n\
             Tab        4 espaces, toujours\n\
             Ctrl+S     sauvegarder (norme vérifiée dans la marge)\n\
             Ctrl+B     compiler — erreurs et warnings dans la marge\n\
             Ctrl+N/P   diagnostic suivant / précédent\n\
             Ctrl+Q     quitter\n\
             Ctrl+K/U   couper / coller une ligne\n\
             Ctrl+F/R/G chercher · remplacer · aller à la ligne\n\
             Ctrl+T     explorateur de fichiers (c-nano . ouvre sur le dossier)\n\
             Ctrl+O     recherche de fichiers flottante (filtre flou)\n\
             F2         changer de panneau (éditeur → explorateur → recherche)\n\
                        (Alt+Tab, Ctrl+Tab ou Shift+Tab si le système les livre)\n\
             Échap      annuler / revenir"
        );
        return;
    }
    if let Err(e) = c_nano::editor::run(file) {
        eprintln!("c-nano: {e}");
        std::process::exit(1);
    }
}
