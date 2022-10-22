# Gender Guesser

#### _Guesses the gender of a given name_

[ Naminges File](https://raw.githubusercontent.com/Raduc4/gender_guesser/master/src/nam_dict.txt)

---

# Example

```
use gender_guesser::Detector;

let mut d = Detector::new();

assert_eq!(Gender::Male, d.get_gender("John"));
```
