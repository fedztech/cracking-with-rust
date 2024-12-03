// One Away: There are three types of edits that can be performed on strings:
// insert a character, remove a character or replace a character.
// Given 2 strings, write a function to check if they are one edit (or zero edits)
// away
// EXAMPLE
// pale, ple -> true
// pales, pale -> true
// pale, bale -> true
// pale, bake -> false

static REFERENCE_SENTENCE: &'static str = "Hi, my name is Juan.";

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn ex_01_05_unmodified() {
        let are_one_edit = ex_01_05::is_one_edit_away(REFERENCE_SENTENCE, REFERENCE_SENTENCE);
        assert!(are_one_edit);
    }

    #[test]
    fn ex_01_05_one_insertion() {
        let mut modified: String = String::from_str(&REFERENCE_SENTENCE).unwrap();
        modified.insert(5, 'X');

        let are_one_edit = ex_01_05::is_one_edit_away(REFERENCE_SENTENCE, modified.as_str());
        assert!(are_one_edit);
    }

    #[test]
    fn ex_01_05_one_deletion() {
        let mut modified: String = String::from_str(&REFERENCE_SENTENCE).unwrap();
        modified.remove(5);
        println!("Modified {modified}");

        let are_one_edit = ex_01_05::is_one_edit_away(REFERENCE_SENTENCE, modified.as_str());
        assert!(are_one_edit);
    }

    #[test]
    fn ex_01_05_one_replacement() {
        let mut modified: String = String::from_str(&REFERENCE_SENTENCE).unwrap();
        modified.replace_range(3..3, "x");

        let are_one_edit = ex_01_05::is_one_edit_away(REFERENCE_SENTENCE, modified.as_str());
        assert!(are_one_edit);
    }

    #[test]
    fn ex_01_05_remove_last() {
        let mut modified: String = String::from_str(&REFERENCE_SENTENCE).unwrap();
        let length = modified.len();
        modified.remove(length-1);

        let are_one_edit = ex_01_05::is_one_edit_away(REFERENCE_SENTENCE, modified.as_str());
        assert!(are_one_edit);
    }

    #[test]
    fn ex_01_05_multiple_edits() {
        let mut modified: String = String::from_str(&REFERENCE_SENTENCE).unwrap();
        modified.replace_range(3..3, "x");
        modified.remove(5);
        let are_one_edit = ex_01_05::is_one_edit_away(REFERENCE_SENTENCE, modified.as_str());
        assert!(are_one_edit == false);
    }
}

mod ex_01_05 {
    pub fn is_one_edit_away(reference: &str, to_evaluate: &str) -> bool {
        let mut equal: bool = true;
        let reference_bytes = reference.as_bytes();
        let to_evaluate_bytes = to_evaluate.as_bytes();
        let mut insertions = 0;
        let mut deletions = 0;
        let mut replacements = 0;
        let mut ref_ix = 0;
        let mut to_evaluate_ix = 0;

        // If the difference of the length of the strings exceed 2, then return false early

        if (reference_bytes.len() as i64 - to_evaluate_bytes.len() as i64).abs() >= 2 {
            return false;
        }

        while ref_ix < reference_bytes.len() {
            if to_evaluate_ix >= to_evaluate.len() {
                break;
            }

            if reference_bytes[ref_ix] != to_evaluate_bytes[to_evaluate_ix] {
                if to_evaluate_ix + 1 < to_evaluate_bytes.len() {
                    // Insertion
                    // The next character in the "to evaluate" string then matches the reference character
                    // e.g. aword vs paword
                    if reference_bytes[ref_ix] == to_evaluate_bytes[to_evaluate_ix + 1] {
                        insertions += 1;
                        to_evaluate_ix = to_evaluate_ix + 1;
                        continue;
                    }
                    // Substitution
                    else if ref_ix + 1 < reference_bytes.len() {
                        // The next character in both strings match, then it was a replacement
                        // e.g. aword vs pword
                        if reference_bytes[ref_ix + 1] == to_evaluate_bytes[to_evaluate_ix + 1] {
                            replacements += 1;
                            ref_ix += 1;
                            to_evaluate_ix += 1;
                            continue;
                        }
                    } else if reference_bytes[ref_ix + 1] == to_evaluate_bytes[to_evaluate_ix] {
                        // Deletion
                        // e.g. aword vs word
                        deletions += 1;
                        ref_ix += 1;
                    }
                } else {
                    // to evaluate is depleted, thus deletion ocurred.
                    deletions += 1;
                }
            }

            ref_ix += 1;
            to_evaluate_ix += 1;
        }

        (insertions + deletions + replacements) <= 1
    }
}
