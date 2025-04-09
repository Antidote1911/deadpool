#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::NamedTempFile;

    fn cmd() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn test_cli_version() {
        cmd()
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_cli_help() {
        cmd()
            .arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage"));
    }

    #[test]
    fn test_generate_password_default_length() {
        for _ in 0..30 {
            cmd().assert().stdout(predicate::function(|s: &str| s.trim().len() == 10));
        }
    }


    #[test]
    fn test_generate_password_specified_length() {
        let expected_length = 24;
        for _ in 0..100 {
            cmd().args(&["-L", "24"])
                .assert()
                .stdout(predicate::function(|s: &str| s.trim().chars().count() == expected_length));
        }
    }

    #[test]
    fn test_invalid_length_input() {
        cmd()
            .args(["-L", "0"])
            .assert()
            .success()
            .stderr(predicates::str::contains("Error generating password"));
    }

    #[test]
    fn test_generate_password_exclude_chars() {
        let exclude_chars = "aeiou01234";
        let length = 12;
        for _ in 0..30 {
            cmd()
                .args(&["-L", "12", "--exclude", exclude_chars])
                .assert()
                .success()
                .stdout(predicate::function(move |s: &str| {
                    let password = s.trim();
                    password.len() == length && password.chars().all(|c| !exclude_chars.contains(c))
                }));
        }
    }


    #[test]
    fn test_generate_multiple_passwords() {
        let count = 100;
        let output = cmd()
            .args(&["-C", &count.to_string(), "-L", "12"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone();

        let binding = String::from_utf8_lossy(&output);
        let passwords: Vec<&str> = binding
            .lines()
            .collect();

        assert_eq!(passwords.len(), count);
        for password in passwords {
            assert_eq!(password.len(), 12);
        }
    }

    #[test]
    fn test_output_argument() {
        // Création d'un fichier temporaire
        let output_file = NamedTempFile::new().expect("Impossible de créer un fichier temporaire");
        let output_path = output_file.path();

        let expected_length = 20;

        // Exécution de la commande
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.args(&["--output", output_path.to_str().unwrap(), "-L", "20", "-dlu"])
            .assert()
            .success();

        // Vérification du contenu du fichier
        let output_content = std::fs::read_to_string(output_path)
            .expect("Échec de la lecture du fichier de sortie");
        assert_eq!(output_content.trim().chars().count(), expected_length, "Le contenu du fichier ne correspond pas à la longueur attendue. Contenu : '{}'", output_content);
    }

    #[test]
    fn test_generate_password_include_chars() {
        // Caractères à inclure et longueur
        let include_chars = "abc123";
        let length = 12;

        // Exécuter la commande avec les arguments spécifiés
        cmd()
            .args(&["-L", &length.to_string(), "--include", include_chars]) // Arguments
            .assert()
            .success()           // La commande ne doit pas échouer
            .stdout(predicate::function(move |s: &str| {
                let password = s.trim();
                password.len() == length && password.chars().all(|c| include_chars.contains(c))
            }));   // Vérifie le motif
    }

    #[test]
    fn test_generate_password_only_one_char() {
        let include_chars = "X"; // Caractère à inclure
        let length = 12;        // Longueur du mot de passe à générer
        let expected_output = "XXXXXXXXXXXX\n"; // Résultat attendu

        let mut command = cmd();
        command
            .args(&["-L", &length.to_string(), "--include", include_chars])
            .assert()
            .success()
            .stdout(predicate::eq(expected_output)); // Vérifie que la sortie correspond exactement à ce qui est attendu
    }
}