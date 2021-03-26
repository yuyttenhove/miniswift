pub fn random_choose<T>(option1: T, option2: T) -> T {
    if rand::random() {option1} else {option2}
}