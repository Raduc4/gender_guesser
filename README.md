# Gender Guesser

#### _Guesses the gender of a given name_

---

# Example

```
use gender_guesser::Detector;

let mut d = Detector::new();

assert_eq!(Gender::Male, d.get_gender("John"));
```
